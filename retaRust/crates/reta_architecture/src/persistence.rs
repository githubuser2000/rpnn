//! Persistence and audit morphisms transcompiled from
//! `python_arch_reference/reta_architecture/persistence.py`.
//!
//! The Python source owns SQLite materialisation.  This Rust stage keeps the
//! same table names, record shapes, cache/audit operations and universal
//! property in a dependency-free in-memory store.  A later stage can swap the
//! store implementation for SQLite without changing callers.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub fn stable_digest_text(value: &str) -> String {
    // Dependency-free stable FNV-1a digest.  The Python layer uses SHA-256;
    // this is a deterministic architecture key until the concrete DB backend
    // gets wired with a crypto crate or the standard library gains one.
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn jsonish_pair_map(items: &BTreeMap<String, String>) -> String {
    let mut text = String::from("{");
    for (idx, (key, value)) in items.iter().enumerate() {
        if idx > 0 {
            text.push(',');
        }
        text.push_str(&format!("{:?}:{:?}", key, value));
    }
    text.push('}');
    text
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PersistenceConfig {
    pub db_path: String,
    pub initialise: bool,
    pub journal_mode: String,
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            db_path: ":memory:".to_string(),
            initialise: true,
            journal_mode: "WAL".to_string(),
        }
    }
}

impl PersistenceConfig {
    pub fn from_environment() -> Self {
        let db_path = std::env::var("RETA_PERSISTENCE_DB")
            .ok()
            .or_else(|| std::env::var("RETA_AUDIT_DB").ok())
            .unwrap_or_else(|| ":memory:".to_string());
        Self {
            db_path,
            ..Self::default()
        }
    }

    pub fn snapshot(&self) -> PersistenceConfigSnapshot {
        PersistenceConfigSnapshot {
            class: "PersistenceConfig".to_string(),
            db_path: self.db_path.clone(),
            initialise: self.initialise,
            journal_mode: self.journal_mode.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PersistenceConfigSnapshot {
    pub class: String,
    pub db_path: String,
    pub initialise: bool,
    pub journal_mode: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PersistedRecord {
    pub table: String,
    pub key: String,
    pub digest: String,
    pub rowid: Option<u64>,
}

impl PersistedRecord {
    pub fn new(
        table: impl Into<String>,
        key: impl Into<String>,
        digest: impl Into<String>,
    ) -> Self {
        Self {
            table: table.into(),
            key: key.into(),
            digest: digest.into(),
            rowid: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PersistedSection {
    pub section_hash: String,
    pub kind: String,
    pub name: String,
    pub context_hash: Option<String>,
    pub payload_json: String,
    pub created_at_counter: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SheafSnapshotRecord {
    pub snapshot_hash: String,
    pub sheaf_name: String,
    pub context_hash: Option<String>,
    pub payload_json: String,
    pub created_at_counter: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecutionRunRecord {
    pub run_hash: String,
    pub operation: String,
    pub context_hash: Option<String>,
    pub task_count: usize,
    pub payload_json: String,
    pub created_at_counter: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuditEventRecord {
    pub event_id: u64,
    pub event_type: String,
    pub subject: String,
    pub payload_hash: String,
    pub payload_json: String,
    pub created_at_counter: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CacheEntryRecord {
    pub cache_key: String,
    pub value_hash: String,
    pub value_json: String,
    pub valid: bool,
    pub created_at_counter: u64,
    pub updated_at_counter: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PersistenceSnapshot {
    pub class: String,
    pub category: String,
    pub config: PersistenceConfigSnapshot,
    pub tables: Vec<String>,
    pub morphisms: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PersistenceStore {
    pub contexts: BTreeMap<String, String>,
    pub local_sections: BTreeMap<String, PersistedSection>,
    pub sheaf_snapshots: BTreeMap<String, SheafSnapshotRecord>,
    pub execution_runs: BTreeMap<String, ExecutionRunRecord>,
    pub audit_events: Vec<AuditEventRecord>,
    pub cache_entries: BTreeMap<String, CacheEntryRecord>,
    counter: u64,
}

impl Default for PersistenceStore {
    fn default() -> Self {
        Self {
            contexts: BTreeMap::new(),
            local_sections: BTreeMap::new(),
            sheaf_snapshots: BTreeMap::new(),
            execution_runs: BTreeMap::new(),
            audit_events: Vec::new(),
            cache_entries: BTreeMap::new(),
            counter: 0,
        }
    }
}

impl PersistenceStore {
    fn tick(&mut self) -> u64 {
        self.counter += 1;
        self.counter
    }

    pub fn persist_context(&mut self, context: Option<BTreeMap<String, String>>) -> Option<String> {
        let payload = context?;
        let text = jsonish_pair_map(&payload);
        let digest = stable_digest_text(&text);
        self.contexts.entry(digest.clone()).or_insert(text);
        Some(digest)
    }

    pub fn persist_section(
        &mut self,
        kind: impl Into<String>,
        name: impl Into<String>,
        payload_json: impl Into<String>,
        context: Option<BTreeMap<String, String>>,
    ) -> PersistedRecord {
        let kind = kind.into();
        let name = name.into();
        let payload_json = payload_json.into();
        let context_hash = self.persist_context(context);
        let digest = stable_digest_text(&format!(
            "kind={kind};name={name};context={:?};payload={payload_json}",
            context_hash
        ));
        let created = self.tick();
        self.local_sections.insert(
            digest.clone(),
            PersistedSection {
                section_hash: digest.clone(),
                kind: kind.clone(),
                name: name.clone(),
                context_hash,
                payload_json,
                created_at_counter: created,
            },
        );
        PersistedRecord::new("local_sections", format!("{kind}:{name}"), digest)
    }

    pub fn load_section(&self, section_hash: &str) -> Option<&PersistedSection> {
        self.local_sections.get(section_hash)
    }

    pub fn persist_sheaf_snapshot(
        &mut self,
        sheaf_name: impl Into<String>,
        payload_json: impl Into<String>,
        context: Option<BTreeMap<String, String>>,
    ) -> PersistedRecord {
        let sheaf_name = sheaf_name.into();
        let payload_json = payload_json.into();
        let context_hash = self.persist_context(context);
        let digest = stable_digest_text(&format!(
            "sheaf={sheaf_name};context={:?};payload={payload_json}",
            context_hash
        ));
        let created = self.tick();
        self.sheaf_snapshots.insert(
            digest.clone(),
            SheafSnapshotRecord {
                snapshot_hash: digest.clone(),
                sheaf_name: sheaf_name.clone(),
                context_hash,
                payload_json,
                created_at_counter: created,
            },
        );
        PersistedRecord::new("sheaf_snapshots", sheaf_name, digest)
    }

    pub fn persist_execution_run(
        &mut self,
        operation: impl Into<String>,
        task_count: usize,
        payload_json: impl Into<String>,
        context: Option<BTreeMap<String, String>>,
    ) -> PersistedRecord {
        let operation = operation.into();
        let payload_json = payload_json.into();
        let context_hash = self.persist_context(context);
        let digest = stable_digest_text(&format!(
            "operation={operation};tasks={task_count};context={:?};payload={payload_json}",
            context_hash
        ));
        let created = self.tick();
        self.execution_runs.insert(
            digest.clone(),
            ExecutionRunRecord {
                run_hash: digest.clone(),
                operation: operation.clone(),
                context_hash,
                task_count,
                payload_json,
                created_at_counter: created,
            },
        );
        PersistedRecord::new("execution_runs", operation, digest)
    }

    pub fn record_audit_event(
        &mut self,
        event_type: impl Into<String>,
        subject: impl Into<String>,
        payload_json: impl Into<String>,
    ) -> PersistedRecord {
        let event_type = event_type.into();
        let subject = subject.into();
        let payload_json = payload_json.into();
        let digest = stable_digest_text(&payload_json);
        let created = self.tick();
        let event_id = self.audit_events.len() as u64 + 1;
        self.audit_events.push(AuditEventRecord {
            event_id,
            event_type: event_type.clone(),
            subject: subject.clone(),
            payload_hash: digest.clone(),
            payload_json,
            created_at_counter: created,
        });
        PersistedRecord {
            table: "audit_events".to_string(),
            key: format!("{event_type}:{subject}"),
            digest,
            rowid: Some(event_id),
        }
    }

    pub fn query_audit_events(
        &self,
        event_type: Option<&str>,
        subject: Option<&str>,
        limit: usize,
    ) -> Vec<AuditEventRecord> {
        let mut out = self
            .audit_events
            .iter()
            .rev()
            .filter(|event| {
                event_type
                    .map(|wanted| wanted == event.event_type)
                    .unwrap_or(true)
            })
            .filter(|event| {
                subject
                    .map(|wanted| wanted == event.subject)
                    .unwrap_or(true)
            })
            .take(limit.max(1))
            .cloned()
            .collect::<Vec<_>>();
        out.sort_by(|left, right| right.event_id.cmp(&left.event_id));
        out
    }

    pub fn cache_put(
        &mut self,
        cache_key: impl Into<String>,
        value_json: impl Into<String>,
    ) -> PersistedRecord {
        let cache_key = cache_key.into();
        let value_json = value_json.into();
        let digest = stable_digest_text(&value_json);
        let now = self.tick();
        let created = self
            .cache_entries
            .get(&cache_key)
            .map(|entry| entry.created_at_counter)
            .unwrap_or(now);
        self.cache_entries.insert(
            cache_key.clone(),
            CacheEntryRecord {
                cache_key: cache_key.clone(),
                value_hash: digest.clone(),
                value_json,
                valid: true,
                created_at_counter: created,
                updated_at_counter: now,
            },
        );
        PersistedRecord::new("cache_entries", cache_key, digest)
    }

    pub fn cache_put_many(&mut self, entries: &[(String, String)]) -> Vec<PersistedRecord> {
        entries
            .iter()
            .map(|(key, value)| self.cache_put(key.clone(), value.clone()))
            .collect()
    }

    pub fn cache_get(&self, cache_key: &str) -> Option<String> {
        self.cache_entries
            .get(cache_key)
            .filter(|entry| entry.valid)
            .map(|entry| entry.value_json.clone())
    }

    pub fn invalidate_cache(&mut self, cache_key: Option<&str>) -> usize {
        let now = self.tick();
        let mut changed = 0usize;
        for (key, entry) in self.cache_entries.iter_mut() {
            if cache_key.map(|wanted| wanted == key).unwrap_or(true) && entry.valid {
                entry.valid = false;
                entry.updated_at_counter = now;
                changed += 1;
            }
        }
        changed
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PersistenceBundle {
    pub config: PersistenceConfig,
}

impl PersistenceBundle {
    pub fn new(config: PersistenceConfig) -> Self {
        Self { config }
    }

    pub fn in_memory_store(&self) -> PersistenceStore {
        let _ = self.config.initialise;
        PersistenceStore::default()
    }

    pub fn snapshot(&self) -> PersistenceSnapshot {
        PersistenceSnapshot {
            class: "PersistenceBundle".to_string(),
            category: "PersistenceCategory".to_string(),
            config: self.config.snapshot(),
            tables: vec![
                "open_contexts".to_string(),
                "local_sections".to_string(),
                "sheaf_snapshots".to_string(),
                "execution_runs".to_string(),
                "audit_events".to_string(),
                "cache_entries".to_string(),
                "activation_stores_via_local_sections".to_string(),
            ],
            morphisms: vec![
                "persist_section".to_string(),
                "persist_sections_batch".to_string(),
                "load_section".to_string(),
                "persist_sheaf_snapshot".to_string(),
                "persist_sheaf_snapshots_batch".to_string(),
                "persist_execution_run".to_string(),
                "record_audit_event".to_string(),
                "query_audit_events".to_string(),
                "cache_put".to_string(),
                "cache_put_many".to_string(),
                "cache_get".to_string(),
                "invalidate_cache".to_string(),
                "persist_activation_store_text".to_string(),
                "load_activation_store_text".to_string(),
                "audit_activation_store_persistence".to_string(),
                "cache_latest_activation_store_digest".to_string(),
            ],
            universal_property:
                "load_persisted_snapshot_equals_original_snapshot_when_digest_matches".to_string(),
        }
    }
}

pub fn bootstrap_persistence(
    config: Option<PersistenceConfig>,
    db_path: Option<String>,
) -> PersistenceBundle {
    let config = config.unwrap_or_else(|| {
        db_path
            .map(|path| PersistenceConfig {
                db_path: path,
                ..PersistenceConfig::default()
            })
            .unwrap_or_else(PersistenceConfig::from_environment)
    });
    PersistenceBundle::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_roundtrip_and_invalidate() {
        let bundle = bootstrap_persistence(Some(PersistenceConfig::default()), None);
        let mut store = bundle.in_memory_store();
        let record = store.cache_put("a", "{\"x\":1}");
        assert_eq!(record.table, "cache_entries");
        assert_eq!(store.cache_get("a"), Some("{\"x\":1}".to_string()));
        assert_eq!(store.invalidate_cache(Some("a")), 1);
        assert_eq!(store.cache_get("a"), None);
    }

    #[test]
    fn audit_query_is_newest_first() {
        let mut store = PersistenceStore::default();
        store.record_audit_event("build", "a", "1");
        store.record_audit_event("build", "b", "2");
        let events = store.query_audit_events(Some("build"), None, 10);
        assert_eq!(events[0].subject, "b");
    }
}

// Stage 16: concrete persistence.py compatibility wrappers.
pub fn stable_digest(value: &str) -> String { stable_digest_text(value) }

pub fn _json_dumps(items: &BTreeMap<String, String>) -> String { jsonish_pair_map(items) }

pub fn connect(db_path: Option<String>) -> PersistenceBundle {
    bootstrap_persistence(None, db_path)
}

pub fn initialise_persistence_schema(bundle: &PersistenceBundle) -> PersistenceSnapshot {
    bundle.snapshot()
}

pub fn _prepare_section_entries_worker(items: &[(String, String, String)]) -> Vec<(String, String, String, String)> {
    items.iter().map(|(kind, name, payload)| (kind.clone(), name.clone(), payload.clone(), stable_digest_text(payload))).collect()
}

pub fn _prepare_sheaf_snapshot_entries_worker(items: &[(String, String)]) -> Vec<(String, String, String)> {
    items.iter().map(|(name, payload)| (name.clone(), payload.clone(), stable_digest_text(payload))).collect()
}

pub fn _prepare_cache_entries_worker(items: &[(String, String)]) -> Vec<(String, String, String)> {
    items.iter().map(|(key, value)| (key.clone(), value.clone(), stable_digest_text(value))).collect()
}

pub fn _prepare_persistence_entries_in_processes(items: &[(String, String)]) -> Vec<(String, String, String)> {
    _prepare_cache_entries_worker(items)
}

pub fn persist_sections_batch(store: &mut PersistenceStore, sections: &[(String, String, String)]) -> Vec<PersistedRecord> {
    sections.iter().map(|(kind, name, payload)| store.persist_section(kind.clone(), name.clone(), payload.clone(), None)).collect()
}

pub fn persist_sheaf_snapshots_batch(store: &mut PersistenceStore, snapshots: &[(String, String)]) -> Vec<PersistedRecord> {
    snapshots.iter().map(|(name, payload)| store.persist_sheaf_snapshot(name.clone(), payload.clone(), None)).collect()
}

pub fn load_sheaf_snapshot(store: &PersistenceStore, snapshot_hash: &str) -> Option<SheafSnapshotRecord> {
    store.sheaf_snapshots.get(snapshot_hash).cloned()
}

// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "_json_dumps",
    "_prepare_cache_entries_worker",
    "_prepare_persistence_entries_in_processes",
    "_prepare_section_entries_worker",
    "_prepare_sheaf_snapshot_entries_worker",
    "connect",
    "initialise_persistence_schema",
    "load_sheaf_snapshot",
    "stable_digest",
    "persist_sections_batch",
    "persist_sheaf_snapshots_batch",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
