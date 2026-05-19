#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Vergleicht wichtige retaPrompt-Completion-Fälle zwischen der Python-Referenz
(`retaPrompt.py`/`NestedCompleter`) und der Rust-Completion-Probe.

Das Werkzeug ändert keine interaktive Completion. Es fragt nur beide Seiten ab
und prüft konservative Python-vs-Rust-Invarianten: Kandidaten, die Python für
bestimmte Eingaben anbietet, müssen in Rust ebenfalls verfügbar sein. Die volle
Fuzzy-Liste wird bewusst nicht bitgenau verglichen, weil Python prompt_toolkit
versionsabhängige Fuzzy-Rankings liefert.
"""
from __future__ import annotations

import argparse
import contextlib
import importlib
import io
import json
import os
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, List, Sequence


@dataclass(frozen=True)
class Case:
    line: str
    must_have: Sequence[str]
    description: str
    mode: str = "normal"
    context: str = ""


CASES: Sequence[Case] = (
    Case("", ("HELP", "hilfe", "reta"), "Top-Level-Befehle wie Python"),
    Case("he", ("HELP", "hilfe"), "Python-Fuzzy fuer Hilfe/HELP"),
    Case("reta ", ("-zeilen", "-spalten", "-kombination", "-ausgabe", "-nichts", "-help", "-h"), "reta-Hauptparameter"),
    Case("reta -zeilen --ze", ("--zeit=", "--zaehlung=", "--primzahlen="), "Zeilen-Unterparameter"),
    Case("reta -zeilen --zeit=h", ("heute", "-heute"), "Zeilen-Zeitwerte"),
    Case("reta -zeilen --zeit=[heute,gestern],m", ("morgen", "-morgen"), "Kommawert nach Klammern"),
    Case("reta -spalten --menschliches=bew", ("Bewusstsein_und_Wahrnehmung", "bewusstsein"), "Spalten-Werte aus Python-paraNdataMatrix"),
    Case("reta -ausgabe --art=h", ("html",), "Ausgabeart-Werte"),
    Case("reta -kombination --galaxie=le", ("Lebewesen", "lebewesen"), "Kombinationswerte Galaxie"),
    Case("15_13_", ("15_13_6", "15_13_17", "15_13_1pro8"), "wahl15-Nested-Completion"),
    Case("16_15_1pro", ("16_15_1pro12", "16_15_1pro13", "16_15_1pro19"), "wahl16/wahl15-Nested-Completion"),
    Case("--ze", ("--zeit=",), "gespeicherter reta -zeilen Kontext", context="reta -zeilen"),
    Case("1-", tuple(), "loeschenSelect deaktiviert Completion", mode="loeschen-select"),
)


def _repo_default(name: str) -> Path:
    here = Path(__file__).resolve()
    candidates = [here.parent.parent / name, here.parent.parent.parent / name, Path.cwd() / name, Path.cwd().parent / name]
    for path in candidates:
        if path.exists():
            return path.resolve()
    return candidates[0].resolve()


def _write_python_stubs(stub_dir: Path) -> None:
    (stub_dir / "bbcode.py").write_text("# stub for completion probe\n", encoding="utf-8")
    (stub_dir / "html2text.py").write_text("# stub for completion probe\n", encoding="utf-8")
    (stub_dir / "textwrap2.py").write_text(
        "def fill(*args, **kwargs):\n    return args[0] if args else ''\n", encoding="utf-8"
    )
    (stub_dir / "orderedset.py").write_text(
        "class OrderedSet(set):\n    pass\n", encoding="utf-8"
    )
    rich = stub_dir / "rich"
    rich.mkdir(exist_ok=True)
    (rich / "__init__.py").write_text("", encoding="utf-8")
    (rich / "console.py").write_text(
        "class Console:\n"
        "    def __init__(self, *args, **kwargs): pass\n"
        "    def print(self, *args, **kwargs): pass\n",
        encoding="utf-8",
    )
    (rich / "markdown.py").write_text(
        "class Markdown:\n    def __init__(self, *args, **kwargs): self.args = args\n",
        encoding="utf-8",
    )
    (rich / "syntax.py").write_text(
        "class Syntax:\n    def __init__(self, *args, **kwargs): self.args = args\n",
        encoding="utf-8",
    )


@contextlib.contextmanager
def _silence_process_output():
    old_stdout = os.dup(1)
    old_stderr = os.dup(2)
    devnull = os.open(os.devnull, os.O_WRONLY)
    try:
        os.dup2(devnull, 1)
        os.dup2(devnull, 2)
        with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
            yield
    finally:
        os.dup2(old_stdout, 1)
        os.dup2(old_stderr, 2)
        os.close(old_stdout)
        os.close(old_stderr)
        os.close(devnull)


def _import_python_prompt(python_repo: Path):
    stub_dir = Path(tempfile.mkdtemp(prefix="reta_completion_stubs_"))
    _write_python_stubs(stub_dir)
    sys.path.insert(0, str(stub_dir))
    sys.path.insert(0, str(python_repo / "libs"))
    sys.path.insert(0, str(python_repo))
    sys.argv = [str(python_repo / "retaPrompt.py")]
    with _silence_process_output():
        return importlib.import_module("retaPrompt")


def python_completions(reta_prompt_module, line: str) -> List[str]:
    from prompt_toolkit.completion import CompleteEvent
    from prompt_toolkit.document import Document

    with _silence_process_output():
        startpunkt = reta_prompt_module.PromptAllesVorGroesserSchleife()[4]
        return [
            completion.text
            for completion in startpunkt.get_completions(
                Document(line, cursor_position=len(line)), CompleteEvent()
            )
        ]


def rust_completions(rust_repo: Path, line: str, mode: str, context: str) -> List[str]:
    cmd = [
        "cargo",
        "run",
        "--quiet",
        "-p",
        "retaprompt_input",
        "--features",
        "completion-probe",
        "--bin",
        "retaprompt_completion_probe",
        "--",
        "--mode",
        mode,
        "--line",
        line,
    ]
    if context:
        cmd.extend(["--context", context])
    env = os.environ.copy()
    env.setdefault("COLUMNS", "120")
    result = subprocess.run(
        cmd,
        cwd=rust_repo,
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        timeout=300,
    )
    if result.returncode != 0:
        raise RuntimeError(
            "Rust-Completion-Probe fehlgeschlagen:\n"
            + " ".join(cmd)
            + "\nSTDERR:\n"
            + result.stderr
        )
    return list(json.loads(result.stdout))


def _missing(values: Iterable[str], expected: Iterable[str]) -> List[str]:
    value_set = set(values)
    return [item for item in expected if item not in value_set]


def main(argv: Sequence[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--python-repo", type=Path, default=_repo_default("reta"))
    parser.add_argument("--rust-repo", type=Path, default=Path.cwd())
    parser.add_argument("--skip-python", action="store_true", help="nur Rust gegen die konservativen Fixture-Erwartungen prüfen")
    parser.add_argument("--json", action="store_true", help="Ergebnis als JSON ausgeben")
    args = parser.parse_args(argv[1:])

    rust_repo = args.rust_repo.resolve()
    python_module = None if args.skip_python else _import_python_prompt(args.python_repo.resolve())
    failures = []
    rows = []

    for case in CASES:
        rust_values = rust_completions(rust_repo, case.line, case.mode, case.context)
        python_values = [] if python_module is None else python_completions(python_module, case.line)

        expected = list(case.must_have)
        if python_values:
            expected = [item for item in expected if item in python_values]
        missing = _missing(rust_values, expected)
        if missing:
            failures.append({"case": case.description, "line": case.line, "missing": missing})
        if case.mode == "loeschen-select" and rust_values:
            failures.append({"case": case.description, "line": case.line, "unexpected": rust_values})

        rows.append(
            {
                "description": case.description,
                "line": case.line,
                "mode": case.mode,
                "context": case.context,
                "must_have": list(case.must_have),
                "python_count": len(python_values) if python_module is not None else None,
                "rust_count": len(rust_values),
                "missing": missing,
            }
        )

    if args.json:
        print(json.dumps({"ok": not failures, "cases": rows, "failures": failures}, ensure_ascii=False, indent=2))
    else:
        for row in rows:
            status = "OK" if not row["missing"] else "FEHLT: " + ", ".join(row["missing"])
            print(f"{status:40} {row['description']}: {row['line']!r}")

    if failures:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
