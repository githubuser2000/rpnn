from __future__ import annotations

"""SQLite persistence and audit layer for Reta architecture materialisations.

Persistence is intentionally orthogonal to the mathematical core.  It stores
instances, snapshots, cache materialisations and audit events; it does not define
what a topology, garbe, morphism, functor or natural transformation means.
"""

import hashlib
import json
import os
import sqlite3
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable, Mapping, Sequence


def _json_dumps(value: Any) -> str:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":"), default=str)


def stable_digest(value: Any) -> str:
    return hashlib.sha256(_json_dumps(value).encode("utf-8")).hexdigest()


@dataclass(frozen=True)
class PersistenceConfig:
    db_path: str = ":memory:"
    initialise: bool = True
    journal_mode: str = "WAL"

    @classmethod
    def from_environment(cls, repo_root: Path | None = None) -> "PersistenceConfig":
        env_path = os.environ.get("RETA_PERSISTENCE_DB") or os.environ.get("RETA_AUDIT_DB")
        if env_path:
            return cls(db_path=env_path)
        return cls(db_path=":memory:")

    def snapshot(self) -> dict:
        return {"class": type(self).__name__, "db_path": self.db_path, "initialise": self.initialise, "journal_mode": self.journal_mode}


@dataclass(frozen=True)
class PersistedRecord:
    table: str
    key: str
    digest: str
    rowid: int | None = None

    def snapshot(self) -> dict:
        return {"class": type(self).__name__, "table": self.table, "key": self.key, "digest": self.digest, "rowid": self.rowid}


@dataclass(frozen=True)
class PersistenceBundle:
    config: PersistenceConfig

    def connect(self) -> sqlite3.Connection:
        if self.config.db_path != ":memory:":
            Path(self.config.db_path).expanduser().resolve().parent.mkdir(parents=True, exist_ok=True)
        connection = sqlite3.connect(self.config.db_path)
        connection.row_factory = sqlite3.Row
        if self.config.db_path != ":memory:" and self.config.journal_mode:
            try:
                connection.execute(f"PRAGMA journal_mode={self.config.journal_mode}")
            except sqlite3.DatabaseError:
                pass
        if self.config.initialise:
            initialise_persistence_schema(connection)
        return connection

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "category": "PersistenceCategory",
            "config": self.config.snapshot(),
            "tables": [
                "open_contexts",
                "local_sections",
                "sheaf_snapshots",
                "execution_runs",
                "audit_events",
                "cache_entries",
            ],
            "morphisms": [
                "persist_section",
                "persist_sections_batch",
                "load_section",
                "persist_sheaf_snapshot",
                "persist_sheaf_snapshots_batch",
                "persist_execution_run",
                "record_audit_event",
                "query_audit_events",
                "cache_put",
                "cache_put_many",
                "cache_get",
                "invalidate_cache",
            ],
            "universal_property": "load_persisted_snapshot_equals_original_snapshot_when_digest_matches",
        }


def bootstrap_persistence(repo_root: Path | None = None, config: PersistenceConfig | None = None, db_path: str | None = None) -> PersistenceBundle:
    if config is None:
        config = PersistenceConfig(db_path=db_path) if db_path is not None else PersistenceConfig.from_environment(repo_root)
    bundle = PersistenceBundle(config=config)
    if config.initialise:
        connection = bundle.connect()
        try:
            initialise_persistence_schema(connection)
        finally:
            connection.close()
    return bundle


def initialise_persistence_schema(connection: sqlite3.Connection) -> None:
    connection.executescript(
        """
        CREATE TABLE IF NOT EXISTS open_contexts (
            context_hash TEXT PRIMARY KEY,
            context_json TEXT NOT NULL,
            created_at REAL NOT NULL
        );
        CREATE TABLE IF NOT EXISTS local_sections (
            section_hash TEXT PRIMARY KEY,
            kind TEXT NOT NULL,
            name TEXT NOT NULL,
            context_hash TEXT,
            payload_json TEXT NOT NULL,
            created_at REAL NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_local_sections_kind_name ON local_sections(kind, name);
        CREATE TABLE IF NOT EXISTS sheaf_snapshots (
            snapshot_hash TEXT PRIMARY KEY,
            sheaf_name TEXT NOT NULL,
            context_hash TEXT,
            payload_json TEXT NOT NULL,
            created_at REAL NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_sheaf_snapshots_name ON sheaf_snapshots(sheaf_name);
        CREATE TABLE IF NOT EXISTS execution_runs (
            run_hash TEXT PRIMARY KEY,
            operation TEXT NOT NULL,
            context_hash TEXT,
            task_count INTEGER NOT NULL,
            payload_json TEXT NOT NULL,
            created_at REAL NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_execution_runs_operation ON execution_runs(operation);
        CREATE TABLE IF NOT EXISTS audit_events (
            event_id INTEGER PRIMARY KEY AUTOINCREMENT,
            event_type TEXT NOT NULL,
            subject TEXT NOT NULL,
            payload_hash TEXT NOT NULL,
            payload_json TEXT NOT NULL,
            created_at REAL NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_audit_events_type_subject ON audit_events(event_type, subject);
        CREATE TABLE IF NOT EXISTS cache_entries (
            cache_key TEXT PRIMARY KEY,
            value_hash TEXT NOT NULL,
            value_json TEXT NOT NULL,
            valid INTEGER NOT NULL DEFAULT 1,
            created_at REAL NOT NULL,
            updated_at REAL NOT NULL
        );
        """
    )
    connection.commit()


def persist_context(connection: sqlite3.Connection, context: Mapping[str, Any] | None) -> str | None:
    if context is None:
        return None
    payload = dict(context)
    digest = stable_digest(payload)
    connection.execute(
        "INSERT OR IGNORE INTO open_contexts(context_hash, context_json, created_at) VALUES (?, ?, ?)",
        (digest, _json_dumps(payload), time.time()),
    )
    return digest


def persist_section(
    connection: sqlite3.Connection,
    *,
    kind: str,
    name: str,
    payload: Any,
    context: Mapping[str, Any] | None = None,
) -> PersistedRecord:
    context_hash = persist_context(connection, context)
    section_payload = {"kind": kind, "name": name, "context_hash": context_hash, "payload": payload}
    digest = stable_digest(section_payload)
    connection.execute(
        """
        INSERT OR REPLACE INTO local_sections(section_hash, kind, name, context_hash, payload_json, created_at)
        VALUES (?, ?, ?, ?, ?, ?)
        """,
        (digest, kind, name, context_hash, _json_dumps(payload), time.time()),
    )
    connection.commit()
    return PersistedRecord("local_sections", f"{kind}:{name}", digest)


def load_section(connection: sqlite3.Connection, section_hash: str) -> dict | None:
    row = connection.execute("SELECT * FROM local_sections WHERE section_hash = ?", (section_hash,)).fetchone()
    if row is None:
        return None
    return {
        "section_hash": row["section_hash"],
        "kind": row["kind"],
        "name": row["name"],
        "context_hash": row["context_hash"],
        "payload": json.loads(row["payload_json"]),
        "created_at": row["created_at"],
    }


def persist_sheaf_snapshot(
    connection: sqlite3.Connection,
    *,
    sheaf_name: str,
    payload: Any,
    context: Mapping[str, Any] | None = None,
) -> PersistedRecord:
    context_hash = persist_context(connection, context)
    snapshot_payload = {"sheaf_name": sheaf_name, "context_hash": context_hash, "payload": payload}
    digest = stable_digest(snapshot_payload)
    connection.execute(
        """
        INSERT OR REPLACE INTO sheaf_snapshots(snapshot_hash, sheaf_name, context_hash, payload_json, created_at)
        VALUES (?, ?, ?, ?, ?)
        """,
        (digest, sheaf_name, context_hash, _json_dumps(payload), time.time()),
    )
    connection.commit()
    return PersistedRecord("sheaf_snapshots", sheaf_name, digest)


def load_sheaf_snapshot(connection: sqlite3.Connection, snapshot_hash: str) -> dict | None:
    row = connection.execute("SELECT * FROM sheaf_snapshots WHERE snapshot_hash = ?", (snapshot_hash,)).fetchone()
    if row is None:
        return None
    return {
        "snapshot_hash": row["snapshot_hash"],
        "sheaf_name": row["sheaf_name"],
        "context_hash": row["context_hash"],
        "payload": json.loads(row["payload_json"]),
        "created_at": row["created_at"],
    }


def persist_execution_run(
    connection: sqlite3.Connection,
    *,
    operation: str,
    task_count: int,
    payload: Any,
    context: Mapping[str, Any] | None = None,
) -> PersistedRecord:
    context_hash = persist_context(connection, context)
    run_payload = {"operation": operation, "task_count": int(task_count), "context_hash": context_hash, "payload": payload}
    digest = stable_digest(run_payload)
    connection.execute(
        """
        INSERT OR REPLACE INTO execution_runs(run_hash, operation, context_hash, task_count, payload_json, created_at)
        VALUES (?, ?, ?, ?, ?, ?)
        """,
        (digest, operation, context_hash, int(task_count), _json_dumps(payload), time.time()),
    )
    connection.commit()
    return PersistedRecord("execution_runs", operation, digest)


def record_audit_event(connection: sqlite3.Connection, *, event_type: str, subject: str, payload: Any) -> PersistedRecord:
    digest = stable_digest(payload)
    cursor = connection.execute(
        """
        INSERT INTO audit_events(event_type, subject, payload_hash, payload_json, created_at)
        VALUES (?, ?, ?, ?, ?)
        """,
        (event_type, subject, digest, _json_dumps(payload), time.time()),
    )
    connection.commit()
    return PersistedRecord("audit_events", f"{event_type}:{subject}", digest, rowid=cursor.lastrowid)


def query_audit_events(connection: sqlite3.Connection, *, event_type: str | None = None, subject: str | None = None, limit: int = 50) -> list[dict]:
    clauses = []
    values: list[Any] = []
    if event_type is not None:
        clauses.append("event_type = ?")
        values.append(event_type)
    if subject is not None:
        clauses.append("subject = ?")
        values.append(subject)
    where = " WHERE " + " AND ".join(clauses) if clauses else ""
    values.append(max(1, int(limit)))
    rows = connection.execute(
        f"SELECT * FROM audit_events{where} ORDER BY event_id DESC LIMIT ?",
        tuple(values),
    ).fetchall()
    return [
        {
            "event_id": row["event_id"],
            "event_type": row["event_type"],
            "subject": row["subject"],
            "payload_hash": row["payload_hash"],
            "payload": json.loads(row["payload_json"]),
            "created_at": row["created_at"],
        }
        for row in rows
    ]


def cache_put(connection: sqlite3.Connection, cache_key: str, value: Any) -> PersistedRecord:
    digest = stable_digest(value)
    now = time.time()
    connection.execute(
        """
        INSERT INTO cache_entries(cache_key, value_hash, value_json, valid, created_at, updated_at)
        VALUES (?, ?, ?, 1, ?, ?)
        ON CONFLICT(cache_key) DO UPDATE SET
            value_hash = excluded.value_hash,
            value_json = excluded.value_json,
            valid = 1,
            updated_at = excluded.updated_at
        """,
        (cache_key, digest, _json_dumps(value), now, now),
    )
    connection.commit()
    return PersistedRecord("cache_entries", cache_key, digest)


def cache_get(connection: sqlite3.Connection, cache_key: str) -> Any | None:
    row = connection.execute("SELECT value_json FROM cache_entries WHERE cache_key = ? AND valid = 1", (cache_key,)).fetchone()
    if row is None:
        return None
    return json.loads(row["value_json"])


def invalidate_cache(connection: sqlite3.Connection, cache_key: str | None = None) -> int:
    if cache_key is None:
        cursor = connection.execute("UPDATE cache_entries SET valid = 0, updated_at = ? WHERE valid = 1", (time.time(),))
    else:
        cursor = connection.execute("UPDATE cache_entries SET valid = 0, updated_at = ? WHERE cache_key = ?", (time.time(), cache_key))
    connection.commit()
    return int(cursor.rowcount)


def _prepare_cache_entries_worker(payload):
    entries = payload
    prepared = []
    for cache_key, value in entries:
        prepared.append((str(cache_key), stable_digest(value), _json_dumps(value)))
    return prepared


def _prepare_section_entries_worker(payload):
    entries = payload
    prepared = []
    for kind, name, value_payload, context in entries:
        context_hash = stable_digest(dict(context)) if context is not None else None
        section_payload = {"kind": kind, "name": name, "context_hash": context_hash, "payload": value_payload}
        prepared.append((kind, name, context_hash, _json_dumps(value_payload), stable_digest(section_payload), dict(context) if context is not None else None))
    return prepared


def _prepare_sheaf_snapshot_entries_worker(payload):
    entries = payload
    prepared = []
    for sheaf_name, value_payload, context in entries:
        context_hash = stable_digest(dict(context)) if context is not None else None
        snapshot_payload = {"sheaf_name": sheaf_name, "context_hash": context_hash, "payload": value_payload}
        prepared.append((sheaf_name, context_hash, _json_dumps(value_payload), stable_digest(snapshot_payload), dict(context) if context is not None else None))
    return prepared


def _prepare_persistence_entries_in_processes(entries: Sequence, worker, operation: str):
    try:
        from .parallel_execution import ParallelExecutionConfig, _chunks, _pool_map_ordered

        config = ParallelExecutionConfig.from_environment()
        if not config.should_use_processes(len(entries)):
            return None
        chunks = list(_chunks(list(entries), config.chunk_size))
        if len(chunks) <= 1:
            return None
        chunk_results, _workers = _pool_map_ordered(worker, chunks, config)
        prepared = []
        for chunk in chunk_results:
            prepared.extend(chunk)
        return prepared
    except Exception:
        return None


def cache_put_many(connection: sqlite3.Connection, entries: Sequence[tuple[str, Any]]) -> list[PersistedRecord]:
    """Store many cache values, computing JSON/digests in process chunks when useful.

    SQLite writes remain serial in this process.  Only pure payload preparation is
    parallelised, so database locking and cache semantics stay deterministic.
    """
    prepared = _prepare_persistence_entries_in_processes(entries, _prepare_cache_entries_worker, "cache_put_many")
    if prepared is None:
        prepared = []
        for cache_key, value in entries:
            prepared.append((str(cache_key), stable_digest(value), _json_dumps(value)))
    now = time.time()
    records: list[PersistedRecord] = []
    for cache_key, digest, value_json in prepared:
        connection.execute(
            """
            INSERT INTO cache_entries(cache_key, value_hash, value_json, valid, created_at, updated_at)
            VALUES (?, ?, ?, 1, ?, ?)
            ON CONFLICT(cache_key) DO UPDATE SET
                value_hash = excluded.value_hash,
                value_json = excluded.value_json,
                valid = 1,
                updated_at = excluded.updated_at
            """,
            (cache_key, digest, value_json, now, now),
        )
        records.append(PersistedRecord("cache_entries", cache_key, digest))
    connection.commit()
    return records


def persist_sections_batch(
    connection: sqlite3.Connection,
    entries: Sequence[tuple[str, str, Any, Mapping[str, Any] | None]],
) -> list[PersistedRecord]:
    """Persist many local sections with process-parallel digest preparation."""
    prepared = _prepare_persistence_entries_in_processes(entries, _prepare_section_entries_worker, "persist_sections_batch")
    if prepared is None:
        prepared = _prepare_section_entries_worker(entries)
    now = time.time()
    records: list[PersistedRecord] = []
    for kind, name, context_hash, payload_json, digest, context in prepared:
        if context is not None:
            connection.execute(
                "INSERT OR IGNORE INTO open_contexts(context_hash, context_json, created_at) VALUES (?, ?, ?)",
                (context_hash, _json_dumps(context), now),
            )
        connection.execute(
            """
            INSERT OR REPLACE INTO local_sections(section_hash, kind, name, context_hash, payload_json, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            """,
            (digest, kind, name, context_hash, payload_json, now),
        )
        records.append(PersistedRecord("local_sections", f"{kind}:{name}", digest))
    connection.commit()
    return records


def persist_sheaf_snapshots_batch(
    connection: sqlite3.Connection,
    entries: Sequence[tuple[str, Any, Mapping[str, Any] | None]],
) -> list[PersistedRecord]:
    """Persist many sheaf snapshots with process-parallel digest preparation."""
    prepared = _prepare_persistence_entries_in_processes(entries, _prepare_sheaf_snapshot_entries_worker, "persist_sheaf_snapshots_batch")
    if prepared is None:
        prepared = _prepare_sheaf_snapshot_entries_worker(entries)
    now = time.time()
    records: list[PersistedRecord] = []
    for sheaf_name, context_hash, payload_json, digest, context in prepared:
        if context is not None:
            connection.execute(
                "INSERT OR IGNORE INTO open_contexts(context_hash, context_json, created_at) VALUES (?, ?, ?)",
                (context_hash, _json_dumps(context), now),
            )
        connection.execute(
            """
            INSERT OR REPLACE INTO sheaf_snapshots(snapshot_hash, sheaf_name, context_hash, payload_json, created_at)
            VALUES (?, ?, ?, ?, ?)
            """,
            (digest, sheaf_name, context_hash, payload_json, now),
        )
        records.append(PersistedRecord("sheaf_snapshots", sheaf_name, digest))
    connection.commit()
    return records
