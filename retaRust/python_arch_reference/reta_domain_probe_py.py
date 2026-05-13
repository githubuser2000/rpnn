#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
Python-nahe Referenz- und Inspektionshilfe für reta.
Dieses Skript spiegelt die wichtigsten JSON-/Probe-Kommandos von
`reta_domain_probe` auf Basis der jetzt expliziten Topologie-/Garbensicht von
reta.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Dict, List, Optional, Sequence, Tuple


def _prepare_imports() -> Path:
    here = Path(__file__).resolve()
    candidates = [here.parent, here.parent.parent, Path.cwd()]
    for base in candidates:
        if (base / "i18n" / "words.py").exists():
            sys.path.insert(0, str(base))
            sys.path.insert(0, str(base / "i18n"))
            sys.path.insert(0, str(base / "libs"))
            return base
    raise SystemExit(
        "Konnte i18n/words.py nicht finden. "
        "Lege das Skript im Python-reta-Repo ab oder starte es von dort."
    )


REPO_ROOT = _prepare_imports()

from reta_architecture import RetaArchitecture  # type: ignore  # noqa: E402

ARCHITECTURE = RetaArchitecture.bootstrap(REPO_ROOT)
PARAMETER_SHEAF = ARCHITECTURE.sheaves.parameter_semantics
HTML_SHEAF = ARCHITECTURE.sheaves.html_reference


def canonical_main_alias_groups() -> List[Dict[str, object]]:
    return PARAMETER_SHEAF.canonical_main_alias_groups()


def parameter_alias_groups_for_main(main_name: str) -> List[Dict[str, object]]:
    return PARAMETER_SHEAF.parameter_alias_groups_for_main(main_name)


def resolve_main_alias(main_name: str) -> Optional[str]:
    return PARAMETER_SHEAF.resolve_main_alias(main_name)


def resolve_parameter_alias(main_name: str, parameter_name: str) -> Optional[str]:
    return PARAMETER_SHEAF.resolve_parameter_alias(main_name, parameter_name)


def canonicalize_pair(main_name: str, parameter_name: str) -> Optional[Tuple[str, str]]:
    return PARAMETER_SHEAF.canonicalize_pair(main_name, parameter_name)


def column_numbers_for_pair(main_name: str, parameter_name: str) -> List[int]:
    return PARAMETER_SHEAF.column_numbers_for_pair(main_name, parameter_name)


def reverse_map_canonical_pairs() -> Dict[int, List[Tuple[str, str]]]:
    return PARAMETER_SHEAF.reverse_map_canonical_pairs()


def exact_meta_for_column(column_number: int) -> List[Dict[str, object]]:
    return PARAMETER_SHEAF.exact_meta_for_column(column_number)


def main_json(main_name: str) -> Dict[str, object]:
    canonical_main = resolve_main_alias(main_name)
    if canonical_main is None:
        raise SystemExit(f"Unbekannter Hauptparameter: {main_name}")

    aliases: List[str] = []
    for group in canonical_main_alias_groups():
        if group["canonical"] == canonical_main:
            aliases = [str(x) for x in group["aliases"]]
            break

    all_columns = set()
    pairs = []
    for group in parameter_alias_groups_for_main(canonical_main):
        canonical_parameter = str(group["canonical"])
        parameter_aliases = [str(x) for x in group["aliases"]]
        cols = column_numbers_for_pair(canonical_main, canonical_parameter)
        if cols:
            all_columns |= set(cols)
            pairs.append(
                {
                    "parameter": canonical_parameter,
                    "aliases": parameter_aliases,
                    "columns": cols,
                }
            )

    return {
        "main": canonical_main,
        "aliases": aliases,
        "columns": sorted(all_columns),
        "pairs": pairs,
    }


def pairs_json(main_name: str) -> List[Dict[str, object]]:
    canonical_main = resolve_main_alias(main_name)
    if canonical_main is None:
        raise SystemExit(f"Unbekannter Hauptparameter: {main_name}")

    out = []
    for group in parameter_alias_groups_for_main(canonical_main):
        canonical_parameter = str(group["canonical"])
        cols = column_numbers_for_pair(canonical_main, canonical_parameter)
        if cols:
            out.append(
                {
                    "main": canonical_main,
                    "parameter": canonical_parameter,
                    "columns": cols,
                }
            )
    return out


def pair_json(main_name: str, parameter_name: str) -> Dict[str, object]:
    pair = canonicalize_pair(main_name, parameter_name)
    if pair is None:
        raise SystemExit(f"Unbekanntes Paar: {main_name} / {parameter_name}")

    canonical_main, canonical_parameter = pair
    main_aliases = []
    for group in canonical_main_alias_groups():
        if group["canonical"] == canonical_main:
            main_aliases = [str(x) for x in group["aliases"]]
            break

    parameter_aliases = []
    for group in parameter_alias_groups_for_main(canonical_main):
        if group["canonical"] == canonical_parameter:
            parameter_aliases = [str(x) for x in group["aliases"]]
            break

    return {
        "input_main": main_name,
        "input_parameter": parameter_name,
        "canonical_main": canonical_main,
        "canonical_parameter": canonical_parameter,
        "main_aliases": main_aliases,
        "parameter_aliases": parameter_aliases,
        "columns": column_numbers_for_pair(canonical_main, canonical_parameter),
    }


def html_meta_for_column(column_number: int) -> Dict[str, object]:
    value = HTML_SHEAF.html_meta_for_column(column_number)
    return value or {
        "column_number": int(column_number),
        "classes": [],
        "class_string": "",
        "class_attributes": [],
        "extra_class_strings": [],
        "all_classes": [],
        "data_attributes": {},
        "attributes": [],
        "attributes_first": {},
        "text": "",
        "raw_open_tag": "",
        "raw_html": "",
        "html_elements": [],
    }


def column_json(column_number: int) -> Dict[str, object]:
    reverse = reverse_map_canonical_pairs()
    pairs = [
        {"main": main, "parameter": parameter}
        for main, parameter in reverse.get(column_number, [])
    ]
    return {
        "column_number": int(column_number),
        "matches": exact_meta_for_column(column_number),
        "summary_pairs": pairs,
        "html": html_meta_for_column(column_number),
    }


def help_text(program_name: str) -> str:
    return f"""{program_name} - Python-Referenz- und Inspektionshilfe für reta

Aufruf:
  {program_name} -h
  {program_name} --help
  {program_name} mains
  {program_name} params <hauptparameter>
  {program_name} pairs <hauptparameter>
  {program_name} pairs-json <hauptparameter>
  {program_name} main-columns <hauptparameter>
  {program_name} main-json <hauptparameter>
  {program_name} pair <hauptparameter> <unterparameter>
  {program_name} pair-json <hauptparameter> <unterparameter>
  {program_name} column <spaltennummer>
  {program_name} column-json <spaltennummer>
  {program_name} reverse <spaltennummer>
  {program_name} html-json <spaltennummer>
  {program_name} html-all-json
  {program_name} pair-html-json <hauptparameter> <unterparameter>
  {program_name} schema-json
  {program_name} architecture-json

Befehle:
  mains
      Zeigt alle kanonischen Oberkategorien und ihre Aliase.

  params <hauptparameter>
      Zeigt alle kanonischen Unterkategorien und ihre Aliase.

  pairs <hauptparameter>
      Zeigt alle kanonischen Paare mit direkten Spalten.

  pairs-json <hauptparameter>
      Wie pairs, aber als JSON.

  main-columns <hauptparameter>
      Zeigt die Vereinigungsmenge direkter Spalten dieses Hauptparameters.

  main-json <hauptparameter>
      Gesamtansicht eines Hauptparameters als JSON.

  pair <hauptparameter> <unterparameter>
      Kanonisiert das Paar und zeigt die direkten Spalten.

  pair-json <hauptparameter> <unterparameter>
      Wie pair, aber als JSON.

  column <spaltennummer>
      Zeigt die direkten Python-Metaeinträge einer Spalte.

  column-json <spaltennummer>
      Wie column, aber als JSON.

  reverse <spaltennummer>
      Zeigt nur die kanonischen Rückwärts-Paare einer Spalte.

  html-json <spaltennummer>
      Zeigt die extrahierte HTML-Meta einer Spalte.

  html-all-json
      Zeigt die extrahierte HTML-Meta aller bekannten Spalten als JSON-Zeilen.

  pair-html-json <hauptparameter> <unterparameter>
      Zeigt für alle direkten Spalten eines Paars zusätzlich die HTML-Meta.

  schema-json
      Zeigt das aus den gesplitteten words-Modulen extrahierte Kontext-/Schemaskelett.

  architecture-json
      Zeigt eine kompakte Gesamtsicht auf Topologie, Prägarben, Garben,
      Morphismen und universelle Konstruktionen.
"""


def print_json(value: object) -> None:
    print(json.dumps(value, ensure_ascii=False, separators=(",", ":")))


def main(argv: Sequence[str]) -> int:
    program_name = Path(argv[0]).name if argv else "reta_domain_probe_py.py"

    if len(argv) <= 1 or argv[1] in {"-h", "--help", "help"}:
        print(help_text(program_name))
        return 0

    cmd = argv[1]

    if cmd == "mains":
        for group in canonical_main_alias_groups():
            print(f'{group["canonical"]} => {", ".join(group["aliases"])}')
        return 0

    if cmd == "params":
        if len(argv) != 3:
            raise SystemExit(f"Erwartet: {program_name} params <hauptparameter>")
        for group in parameter_alias_groups_for_main(argv[2]):
            print(f'{group["canonical"]} => {", ".join(group["aliases"])}')
        return 0

    if cmd == "pairs":
        if len(argv) != 3:
            raise SystemExit(f"Erwartet: {program_name} pairs <hauptparameter>")
        canonical_main = resolve_main_alias(argv[2])
        if canonical_main is None:
            raise SystemExit(f"Unbekannter Hauptparameter: {argv[2]}")
        for group in parameter_alias_groups_for_main(canonical_main):
            cols = column_numbers_for_pair(canonical_main, str(group["canonical"]))
            if cols:
                print(f"{canonical_main} / {group['canonical']} => {cols}")
        return 0

    if cmd == "pairs-json":
        if len(argv) != 3:
            raise SystemExit(f"Erwartet: {program_name} pairs-json <hauptparameter>")
        print_json(pairs_json(argv[2]))
        return 0

    if cmd == "main-columns":
        if len(argv) != 3:
            raise SystemExit(f"Erwartet: {program_name} main-columns <hauptparameter>")
        data = main_json(argv[2])
        print(f"main_columns={data['columns']}")
        return 0

    if cmd == "main-json":
        if len(argv) != 3:
            raise SystemExit(f"Erwartet: {program_name} main-json <hauptparameter>")
        print_json(main_json(argv[2]))
        return 0

    if cmd == "pair":
        if len(argv) != 4:
            raise SystemExit(f"Erwartet: {program_name} pair <hauptparameter> <unterparameter>")
        pair = canonicalize_pair(argv[2], argv[3])
        if pair is None:
            raise SystemExit(f"Unbekanntes Paar: {argv[2]} / {argv[3]}")
        print(f"canonical={pair[0]} / {pair[1]}")
        print(f"columns={column_numbers_for_pair(pair[0], pair[1])}")
        return 0

    if cmd == "pair-json":
        if len(argv) != 4:
            raise SystemExit(f"Erwartet: {program_name} pair-json <hauptparameter> <unterparameter>")
        print_json(pair_json(argv[2], argv[3]))
        return 0

    if cmd == "column":
        if len(argv) != 3:
            raise SystemExit(f"Erwartet: {program_name} column <spaltennummer>")
        column_number = int(argv[2])
        matches = exact_meta_for_column(column_number)
        if not matches:
            raise SystemExit(f"Unbekannte oder nicht-direkte Spalte: {column_number}")
        for match in matches:
            print(f"{column_number} => {match}")
        reverse = reverse_map_canonical_pairs()
        print(f"summary_pairs={reverse.get(column_number, [])}")
        return 0

    if cmd == "column-json":
        if len(argv) != 3:
            raise SystemExit(f"Erwartet: {program_name} column-json <spaltennummer>")
        print_json(column_json(int(argv[2])))
        return 0

    if cmd == "reverse":
        if len(argv) != 3:
            raise SystemExit(f"Erwartet: {program_name} reverse <spaltennummer>")
        reverse = reverse_map_canonical_pairs()
        print(f"summary_pairs={reverse.get(int(argv[2]), [])}")
        return 0

    if cmd == "html-json":
        if len(argv) != 3:
            raise SystemExit(f"Erwartet: {program_name} html-json <spaltennummer>")
        print_json(html_meta_for_column(int(argv[2])))
        return 0

    if cmd == "html-all-json":
        for column_number in sorted(HTML_SHEAF.reference_map):
            print_json(HTML_SHEAF.reference_map[column_number])
        return 0

    if cmd == "pair-html-json":
        if len(argv) != 4:
            raise SystemExit(f"Erwartet: {program_name} pair-html-json <hauptparameter> <unterparameter>")
        pair = canonicalize_pair(argv[2], argv[3])
        if pair is None:
            raise SystemExit(f"Unbekanntes Paar: {argv[2]} / {argv[3]}")
        canonical_main, canonical_parameter = pair
        columns = column_numbers_for_pair(canonical_main, canonical_parameter)
        print_json(
            {
                "input_main": argv[2],
                "input_parameter": argv[3],
                "canonical_main": canonical_main,
                "canonical_parameter": canonical_parameter,
                "columns": columns,
                "html": [html_meta_for_column(col) for col in columns],
            }
        )
        return 0

    if cmd == "schema-json":
        print_json(ARCHITECTURE.schema.snapshot())
        return 0

    if cmd == "architecture-json":
        print_json(ARCHITECTURE.snapshot())
        return 0

    raise SystemExit(f"Unbekannter Befehl: {cmd}")


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
