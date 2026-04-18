#!/usr/bin/env python3
"""Generate Rust prompt semantic-choice constants from Python `i18n.words`.

The generated file is the Rust source of truth for retaPrompt commands
`15_...`, `16_...` and `16_15_...`. It intentionally mirrors the Python
runtime mutation in `retaPrompt.py`:

    wahl15[""] = wahl15["15"]
    wahl16[""] = wahl16["16"]

Run with `python3 -S` when the local Python environment injects site hooks.
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path
from typing import Iterable, Sequence


def rust_str(value: str) -> str:
    return (
        '"'
        + value.replace("\\", "\\\\")
        .replace('"', '\\"')
        .replace("\r", "\\r")
        .replace("\n", "\\n")
        + '"'
    )


def render_entries(name: str, items: Sequence[tuple[str, str]]) -> str:
    lines = [f"pub const {name}: &[SemanticChoiceEntry] = &["]
    for key, value in items:
        lines.append(
            f"    SemanticChoiceEntry {{ key: {rust_str(key)}, value: {rust_str(value)} }},"
        )
    lines.append("];\n")
    return "\n".join(lines)


def render_keys(name: str, items: Sequence[tuple[str, str]]) -> str:
    lines = [f"pub const {name}: &[&str] = &["]
    for key, _ in items:
        lines.append(f"    {rust_str(key)},")
    lines.append("];\n")
    return "\n".join(lines)


def build_source(wahl15: Sequence[tuple[str, str]], wahl16: Sequence[tuple[str, str]]) -> str:
    wahl15_map = dict(wahl15)
    wahl16_map = dict(wahl16)
    retaprompt_wahl15 = [*wahl15, ("", wahl15_map["15"])]
    retaprompt_wahl16 = [*wahl16, ("", wahl16_map["16"])]

    source = '''//! Python-exakte Prompt-Auswahldaten für `15_...`, `16_...` und `16_15_...`.
//!
//! Diese Konstanten entsprechen `i18n.words.wahl15`/`wahl16` plus der
//! Prompt-spezifischen Mutation aus `retaPrompt.py`:
//! `wahl15[""] = wahl15["15"]` und `wahl16[""] = wahl16["16"]`.
//! Ausführung und Completion müssen diese Daten gemeinsam benutzen.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SemanticChoiceEntry {
    pub key: &'static str,
    pub value: &'static str,
}

'''
    source += render_entries("WAHL15_I18N_ENTRIES", wahl15)
    source += render_entries("WAHL16_I18N_ENTRIES", wahl16)
    source += render_entries("RETAPROMPT_WAHL15_ENTRIES", retaprompt_wahl15)
    source += render_entries("RETAPROMPT_WAHL16_ENTRIES", retaprompt_wahl16)
    source += render_keys("RETAPROMPT_WAHL15_KEYS", retaprompt_wahl15)
    source += render_keys("RETAPROMPT_WAHL16_KEYS", retaprompt_wahl16)
    source += '''pub fn retaprompt_wahl15_entries() -> &'static [SemanticChoiceEntry] {
    RETAPROMPT_WAHL15_ENTRIES
}

pub fn retaprompt_wahl16_entries() -> &'static [SemanticChoiceEntry] {
    RETAPROMPT_WAHL16_ENTRIES
}

pub fn semantic_wahl15_ordered_keys() -> &'static [&'static str] {
    RETAPROMPT_WAHL15_KEYS
}

pub fn semantic_wahl16_ordered_keys() -> &'static [&'static str] {
    RETAPROMPT_WAHL16_KEYS
}

pub fn semantic_wahl15_value(key: &str) -> Option<&'static str> {
    RETAPROMPT_WAHL15_ENTRIES
        .iter()
        .find(|entry| entry.key == key)
        .map(|entry| entry.value)
}

pub fn semantic_wahl16_value(key: &str) -> Option<&'static str> {
    RETAPROMPT_WAHL16_ENTRIES
        .iter()
        .find(|entry| entry.key == key)
        .map(|entry| entry.value)
}

pub fn is_wahl15_key(key: &str) -> bool {
    semantic_wahl15_value(key).is_some()
}

pub fn is_wahl16_key(key: &str) -> bool {
    semantic_wahl16_value(key).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_mutation_keeps_python_empty_choice_aliases() {
        assert_eq!(semantic_wahl15_value(""), semantic_wahl15_value("15"));
        assert_eq!(semantic_wahl16_value(""), semantic_wahl16_value("16"));
    }

    #[test]
    fn wahl15_values_match_python_canonical_strings_for_known_drift_cases() {
        assert_eq!(
            semantic_wahl15_value("15"),
            Some("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Geist_(15),Model_of_Hierarchical_Complexity,Biologischer_Baum_(15),Teilchen_anderes_Universum,nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)")
        );
        assert_eq!(semantic_wahl15_value("9_6"), Some("Größenordnung"));
    }

    #[test]
    fn prompt_choice_counts_match_python_words_plus_retaprompt_empty_alias() {
        assert_eq!(WAHL15_I18N_ENTRIES.len(), 65);
        assert_eq!(WAHL16_I18N_ENTRIES.len(), 9);
        assert_eq!(RETAPROMPT_WAHL15_ENTRIES.len(), 66);
        assert_eq!(RETAPROMPT_WAHL16_ENTRIES.len(), 10);
        assert_eq!(RETAPROMPT_WAHL15_KEYS.len(), RETAPROMPT_WAHL15_ENTRIES.len());
        assert_eq!(RETAPROMPT_WAHL16_KEYS.len(), RETAPROMPT_WAHL16_ENTRIES.len());
    }
}
'''
    return source


def load_words(python_root: Path):
    sys.path.insert(0, str(python_root))
    sys.path.insert(0, str(python_root / "libs"))
    import i18n.words as words  # type: ignore

    return list(words.wahl15.items()), list(words.wahl16.items())


def main(argv: Iterable[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--python-root", type=Path, required=True)
    parser.add_argument("--out", type=Path, default=Path("src/prompt/semantic_choices.rs"))
    args = parser.parse_args(list(argv) if argv is not None else None)

    wahl15, wahl16 = load_words(args.python_root.resolve())
    args.out.write_text(build_source(wahl15, wahl16), encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
