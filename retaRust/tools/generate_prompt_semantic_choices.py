#!/usr/bin/env python3
"""Generate Rust prompt semantic-choice constants from Python `i18n.words`.

The generated file is the Rust source of truth for retaPrompt commands
`15_...`, `16_...` and `16_15_...`. It also carries the small prompt
regex/completion inventories that Python gets from `i18n.words` and
`LibRetaPrompt.py`.

It intentionally mirrors the Python runtime mutation in `retaPrompt.py`:

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


def dedupe_by_key_preserving_order(items: Sequence[tuple[str, str]]) -> list[tuple[str, str]]:
    """Match Python dict semantics before rendering Rust constants."""
    out: list[tuple[str, str]] = []
    seen: set[str] = set()
    for key, value in items:
        if key in seen:
            continue
        seen.add(key)
        out.append((key, value))
    return out


def with_retaprompt_empty_alias(
    items: Sequence[tuple[str, str]], fallback_key: str
) -> list[tuple[str, str]]:
    cleaned = dedupe_by_key_preserving_order(items)
    values = dict(cleaned)
    if fallback_key not in values:
        raise KeyError(f"Python wahl table has no canonical key {fallback_key!r}")
    if "" not in values:
        cleaned.append(("", values[fallback_key]))
    return cleaned


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


def render_str_slice(name: str, items: Sequence[str]) -> str:
    lines = [f"pub const {name}: &[&str] = &["]
    for item in items:
        lines.append(f"    {rust_str(item)},")
    lines.append("];\n")
    return "\n".join(lines)


def render_str_const(name: str, value: str) -> str:
    return f"pub const {name}: &str = {rust_str(value)};\n"


def build_source(
    wahl15: Sequence[tuple[str, str]],
    wahl16: Sequence[tuple[str, str]],
    reta_main_switches: Sequence[str],
    reta_section_switches: Sequence[str],
    zeilen_regex_parameters: Sequence[str],
    zeilen_parameter_tokens: Sequence[str],
    zeilen_typ_parameter: str,
    zeilen_typ_values: Sequence[str],
    zeilen_zeit_parameter: str,
    zeilen_zeit_values: Sequence[str],
    zeilen_primzahlen_parameter: str,
    zeilen_primzahlen_values: Sequence[str],
    ausgabe_regex_parameters: Sequence[str],
    ausgabe_parameter_tokens: Sequence[str],
    ausgabe_art_parameter: str,
    ausgabe_art_values: Sequence[str],
    ausgabe_breite_parameter: str,
    ausgabe_breiten_parameter: str,
    kombination_galaxie_parameter: str,
    kombination_universum_parameter: str,
    kombination_parameter_tokens: Sequence[str],
) -> str:
    wahl15 = dedupe_by_key_preserving_order(wahl15)
    wahl16 = dedupe_by_key_preserving_order(wahl16)
    retaprompt_wahl15 = with_retaprompt_empty_alias(wahl15, "15")
    retaprompt_wahl16 = with_retaprompt_empty_alias(wahl16, "16")

    source = '''//! Python-exakte Prompt-Auswahldaten für `15_...`, `16_...` und `16_15_...`.
//!
//! Diese Konstanten entsprechen `i18n.words.wahl15`/`wahl16` plus der
//! Prompt-spezifischen Mutation aus `retaPrompt.py`:
//! `wahl15[""] = wahl15["15"]` und `wahl16[""] = wahl16["16"]`.
//! Die Regex-/Completion-Inventare stammen aus denselben Python-Wörterbüchern,
//! die `regExReplace` und `LibRetaPrompt.NestedCompleter` verwenden.
//! Ausführung, Regex-Expansion und Completion müssen diese Daten gemeinsam benutzen.

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
    source += render_str_slice("RETAPROMPT_RETA_MAIN_SWITCHES", reta_main_switches)
    source += render_str_slice("RETAPROMPT_RETA_SECTION_SWITCHES", reta_section_switches)
    source += render_str_slice("RETAPROMPT_ZEILEN_REGEX_PARAMETERS", zeilen_regex_parameters)
    source += render_str_slice("RETAPROMPT_ZEILEN_PARAMETER_TOKENS", zeilen_parameter_tokens)
    source += render_str_const("RETAPROMPT_ZEILEN_TYP_PARAMETER", zeilen_typ_parameter)
    source += render_str_slice("RETAPROMPT_ZEILEN_TYP_VALUES", zeilen_typ_values)
    source += render_str_const("RETAPROMPT_ZEILEN_ZEIT_PARAMETER", zeilen_zeit_parameter)
    source += render_str_slice("RETAPROMPT_ZEILEN_ZEIT_VALUES", zeilen_zeit_values)
    source += render_str_const("RETAPROMPT_ZEILEN_PRIMZAHLEN_PARAMETER", zeilen_primzahlen_parameter)
    source += render_str_slice("RETAPROMPT_ZEILEN_PRIMZAHLEN_VALUES", zeilen_primzahlen_values)
    source += render_str_slice("RETAPROMPT_AUSGABE_REGEX_PARAMETERS", ausgabe_regex_parameters)
    source += render_str_slice("RETAPROMPT_AUSGABE_PARAMETER_TOKENS", ausgabe_parameter_tokens)
    source += render_str_const("RETAPROMPT_AUSGABE_ART_PARAMETER", ausgabe_art_parameter)
    source += render_str_slice("RETAPROMPT_AUSGABE_ART_VALUES", ausgabe_art_values)
    source += render_str_const("RETAPROMPT_AUSGABE_BREITE_PARAMETER", ausgabe_breite_parameter)
    source += render_str_const("RETAPROMPT_AUSGABE_BREITEN_PARAMETER", ausgabe_breiten_parameter)
    source += render_str_const("RETAPROMPT_KOMBINATION_GALAXIE_PARAMETER", kombination_galaxie_parameter)
    source += render_str_const("RETAPROMPT_KOMBINATION_UNIVERSUM_PARAMETER", kombination_universum_parameter)
    source += render_str_slice("RETAPROMPT_KOMBINATION_PARAMETER_TOKENS", kombination_parameter_tokens)
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

    fn assert_unique_choice_keys(entries: &[SemanticChoiceEntry]) {
        let mut seen = std::collections::BTreeSet::new();
        for entry in entries {
            assert!(seen.insert(entry.key), "duplicate semantic choice key {:?}", entry.key);
        }
    }

    #[test]
    fn prompt_mutation_keeps_python_empty_choice_aliases() {
        assert_eq!(semantic_wahl15_value(""), semantic_wahl15_value("15"));
        assert_eq!(semantic_wahl16_value(""), semantic_wahl16_value("16"));
    }

    #[test]
    fn prompt_choice_keys_keep_python_dict_uniqueness_after_prompt_mutation() {
        assert_unique_choice_keys(WAHL15_I18N_ENTRIES);
        assert_unique_choice_keys(WAHL16_I18N_ENTRIES);
        assert_unique_choice_keys(RETAPROMPT_WAHL15_ENTRIES);
        assert_unique_choice_keys(RETAPROMPT_WAHL16_ENTRIES);
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

    #[test]
    fn prompt_regex_and_completion_tables_come_from_python_words() {
        assert!(RETAPROMPT_RETA_MAIN_SWITCHES.contains(&"-debug"));
        assert!(RETAPROMPT_RETA_SECTION_SWITCHES.contains(&"-kombination"));
        assert!(RETAPROMPT_ZEILEN_PARAMETER_TOKENS.contains(&"--typ="));
        assert!(RETAPROMPT_ZEILEN_TYP_VALUES.contains(&"SonneMitMondanteil"));
        assert!(RETAPROMPT_AUSGABE_PARAMETER_TOKENS.contains(&"--keineueberschriften"));
        assert!(RETAPROMPT_AUSGABE_ART_VALUES.contains(&"markdown"));
        assert_eq!(RETAPROMPT_KOMBINATION_GALAXIE_PARAMETER, "galaxie");
        assert!(RETAPROMPT_KOMBINATION_PARAMETER_TOKENS.contains(&"--universum="));
    }
}
'''
    return source


def load_words(python_root: Path):
    sys.path.insert(0, str(python_root))
    sys.path.insert(0, str(python_root / "libs"))
    import i18n.words as words  # type: ignore

    reta_main_switches = [f"-{value}" for value in words.hauptForNeben.values()]
    reta_section_switches = [
        f"-{words.hauptForNeben[key]}"
        for key in ("zeilen", "spalten", "kombination", "ausgabe")
    ]

    zeilen_completion_specs = [
        ("zeit", True),
        ("zaehlung", True),
        ("vorhervonausschnitt", True),
        ("vorhervonausschnittteiler", False),
        ("primzahlvielfache", True),
        ("nachtraeglichneuabzaehlung", True),
        ("nachtraeglichneuabzaehlungvielfache", True),
        ("alles", False),
        ("potenzenvonzahlen", True),
        ("typ", True),
        ("vielfachevonzahlen", True),
        ("oberesmaximum", True),
        ("primzahlen", True),
        ("invertieren", False),
    ]
    zeilen_parameter_tokens = [
        "--" + words.zeilenParas[key] + ("=" if has_eq else "")
        for key, has_eq in zeilen_completion_specs
    ] + ["--*="]

    ausgabe_parameter_tokens = [
        "--" + value + ("=" if words.ausgabeParasEqSign[key] else "")
        for key, value in words.ausgabeParas.items()
    ] + ["--*="]

    kombination_parameter_tokens = [
        f"--{value}=" for value in words.kombiMainParas.values()
    ] + ["--*="]

    return (
        list(words.wahl15.items()),
        list(words.wahl16.items()),
        reta_main_switches,
        reta_section_switches,
        list(words.zeilenParas.values()),
        zeilen_parameter_tokens,
        words.zeilenParas["typ"],
        [
            words.zeilenParas["sonne"],
            words.zeilenParas["mond"],
            words.zeilenParas["planet"],
            words.zeilenParas["schwarzesonne"],
            words.zeilenParas["SonneMitMondanteil"],
        ],
        words.zeilenParas["zeit"],
        [
            words.zeilenParas["heute"],
            words.zeilenParas["gestern"],
            words.zeilenParas["morgen"],
        ],
        words.zeilenParas["primzahlen"],
        [
            words.zeilenParas["aussenerste"],
            words.zeilenParas["innenerste"],
            words.zeilenParas["aussenalle"],
            words.zeilenParas["innenalle"],
        ],
        list(words.ausgabeParas.values()),
        ausgabe_parameter_tokens,
        words.ausgabeParas["art"],
        list(words.ausgabeArt.values()),
        words.ausgabeParas["breite"],
        words.ausgabeParas["breiten"],
        words.kombiMainParas["galaxie"],
        words.kombiMainParas["universum"],
        kombination_parameter_tokens,
    )


def main(argv: Iterable[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--python-root", type=Path, required=True)
    parser.add_argument("--out", type=Path, default=Path("src/prompt/semantic_choices.rs"))
    args = parser.parse_args(list(argv) if argv is not None else None)

    loaded = load_words(args.python_root.resolve())
    args.out.write_text(build_source(*loaded), encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
