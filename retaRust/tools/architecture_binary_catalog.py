#!/usr/bin/env python3
"""Print a compact catalog of the architecture/diagnostic executables.

This is intentionally dependency-free. It is meant for Termux/Linux runs where
we want a quick overview of the many rreta_arch_* binaries added during the
architecture migration.
"""
from __future__ import annotations

import argparse
import json
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Iterable, List


@dataclass(frozen=True)
class BinaryInfo:
    name: str
    package: str
    category: str
    purpose: str
    useful_output: str
    typical_case: str
    needs_legacy_lines: bool = False


CATALOG: List[BinaryInfo] = [
    BinaryInfo("rreta", "reta", "legacy-visible", "Runs the current visible rreta command path.", "Legacy output lines used as comparison input.", "-zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0"),
    BinaryInfo("rreta_arch_inspect", "reta", "cli-plan", "Explains architecture switch, runtime gates and CLI plan.", "JSON with parsed architecture mode, gates, selected parameters.", "--reta-arch=dry-run -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0"),
    BinaryInfo("rreta_arch_materialize", "reta", "table-data", "Materializes selected CSV/virtual sections from CLI arguments.", "JSON showing direct CSV cells, requested columns, row/column orders.", "-zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0"),
    BinaryInfo("rreta_arch_view", "reta", "table-data", "Builds a MaterializedTableView from materialized sections.", "JSON with view rows, cells, direct/virtual classification.", "-zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0"),
    BinaryInfo("rreta_arch_view_output", "reta", "table-output", "Renders a MaterializedTableView as shell/csv/html/bbcode/markdown/etc.", "JSON with rendered_lines and output-mode metadata.", "-zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m -ausgabe --art=markdown --breite=0"),
    BinaryInfo("rreta_arch_view_output_parity", "reta", "parity", "Compares Rust TableViewOutput against supplied legacy lines.", "Raw and semantic parity JSON.", "--legacy-lines-file legacy.txt -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0", True),
    BinaryInfo("rreta_arch_view_output_shadow", "reta", "parity", "Runs shadow TableViewOutput report and commit decision against legacy lines.", "Shadow report, commit decision, raw/semantic equality.", "--legacy-lines-file legacy.txt --reta-arch=commit -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0", True),
    BinaryInfo("rreta_arch_commit_audit", "reta", "activation", "Audits all required TableViewOutput commit guards.", "Required/diagnostic guard list, failed guards, language/virtual guard status.", "--legacy-lines-file legacy.txt --reta-arch=commit -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0", True),
    BinaryInfo("rreta_arch_activation_transaction", "reta", "activation", "Selects visible source via an explicit audited transaction.", "Selected source, selected lines checksum, fallback reason.", "--legacy-lines-file legacy.txt --reta-arch=commit -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0", True),
    BinaryInfo("rreta_arch_activation_journal", "reta", "activation", "Records activation transactions into a replayable journal.", "Journal records and replay result.", "--legacy-lines-file legacy.txt --reta-arch=commit -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0", True),
    BinaryInfo("rreta_arch_activation_replay", "reta", "activation", "Checks whether a previous journal can be safely replayed.", "Replay guards: transaction id, legacy checksum, rollback decision.", "--legacy-lines-file legacy.txt --reta-arch=commit -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0", True),
    BinaryInfo("rreta_arch_activation_ledger", "reta", "activation", "Turns activation journal records into a hash-chain ledger.", "Ledger entries, chain hashes, validation guards.", "--legacy-lines-file legacy.txt --reta-arch=commit -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0", True),
    BinaryInfo("rreta_arch_activation_store", "reta", "activation", "Encodes/parses activation journal+ledger as a line-oriented store.", "Store text metadata, parse report, checksum validation.", "--legacy-lines-file legacy.txt --reta-arch=commit -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0", True),
    BinaryInfo("rreta_arch_activation_persistence", "reta", "activation", "Persists activation-store text through the architecture persistence layer.", "Roundtrip digest, parse readiness, audit/cache status.", "--legacy-lines-file legacy.txt --reta-arch=commit -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0", True),
    BinaryInfo("rreta_arch_activation_file", "reta", "activation", "Writes/reads an activation store file with read-back validation.", "File path, write/read status, digest and parse readiness.", "--legacy-lines-file legacy.txt --activation-store-file target/reta_arch_diagnostics/store.txt --reta-arch=commit -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0", True),
    BinaryInfo("rreta_arch_activation_recovery", "reta", "activation", "Reads an existing activation-store file and validates recovery safety.", "Recovery candidate/allowed replay, selected source, failed guards.", "--legacy-lines-file legacy.txt --activation-recovery-file store.txt --reta-arch=commit -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0", True),
    BinaryInfo("rreta_arch_activation_readiness", "reta", "activation", "Folds commit/audit/journal/store/recovery witnesses into readiness.", "Ready/blocked status, required guards, promotion-level hint.", "--legacy-lines-file legacy.txt --reta-arch=commit -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0", True),
    BinaryInfo("rreta_arch_activation_promotion", "reta", "activation", "Decides whether a case can be promoted from shadow to default visible path.", "Promotion ready/blocked status and failed promotion guards.", "--legacy-lines-file legacy.txt --reta-arch=commit -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0", True),
    BinaryInfo("rreta_arch_column_order", "reta", "table-diagnostics", "Verifies --spaltenreihenfolgeundnurdiese ordering.", "Requested/materialized column order, especially 744,493 cases.", "-zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m -ausgabe --spaltenreihenfolgeundnurdiese=744,493 --breite=0"),
    BinaryInfo("rreta_arch_row_order", "reta", "table-diagnostics", "Verifies explicit row ordering.", "Requested/materialized row order and header handling.", "-zeilen --vorhervonausschnitt=3,1-2 -spalten --religion=493 --breite=0"),
    BinaryInfo("rreta_arch_output_flags", "reta", "table-output", "Shows parsed output flags such as headers, empty rows, widths and wrapping.", "TableViewOutput options and rendered report.", "-zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m -ausgabe --keineueberschriften --breite=8"),
    BinaryInfo("rreta_arch_numbering", "reta", "table-output", "Explains legacy Zaehlung/Nummerierung prefix projection.", "Numbering report and output with numbering enabled.", "-zeilen --vorhervonausschnitt=1-4 -spalten --kontinuum=m --breite=0"),
    BinaryInfo("rreta_arch_layout", "reta", "table-output", "Explains shell layout, widths and horizontal column pages.", "Measured/effective widths, layout pages, rendered lines.", "-zeilen --vorhervonausschnitt=1-2 -spalten --kontinuum=m -ausgabe --breiten=4,12 --breite=0"),
    BinaryInfo("rreta_arch_html_classes", "reta", "style", "Snapshots the generated htmlclassesPy.jsonl catalog.", "HTML class catalog counts and records.", ""),
    BinaryInfo("rreta_arch_html_output", "reta", "style", "Renders HTML output with optional htmlclasses witnesses.", "HTML output plus attribute report.", "-zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m -ausgabe --art=html --htmlclasses"),
    BinaryInfo("rreta_arch_row_styles", "reta", "style", "Tests legacy row color wrappers for HTML/BBCode.", "Row-style report and styled output.", "-zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m -ausgabe --art=html --rowcolors"),
    BinaryInfo("rreta_arch_cell_styles", "reta", "style", "Tests legacy generateCell/cell-wrapper projection.", "Cell-style report and styled output.", "-zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m -ausgabe --art=html --cellstyles"),
    BinaryInfo("rreta_arch_style_composition", "reta", "style", "Merges HTML-class attributes with cell-style wrappers.", "Composition counts and composed HTML output.", "-zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m -ausgabe --art=html --htmlclasses --cellstyles"),
    BinaryInfo("rreta_arch_style_parity", "reta", "style-parity", "Compares plain vs styled HTML/BBCode semantically.", "Style-aware raw/semantic parity report.", "-zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m -ausgabe --art=html --htmlclasses --cellstyles --rowcolors"),
    BinaryInfo("rreta_arch_shell_styles", "reta", "style", "Tests shell/ANSI color projection and ANSI-strip parity.", "ANSI-cell count and styled shell output.", "-zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m -ausgabe --shellcolors"),
    BinaryInfo("rreta_arch_virtual_columns", "reta", "virtual-columns", "Renders non-direct/virtual columns by policy.", "Virtual policy, virtual cells and rendered witnesses.", "-zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m -ausgabe --spaltenreihenfolgeundnurdiese=744,493 --virtualcolumns --breite=0"),
    BinaryInfo("rreta_arch_virtual_parity", "reta", "virtual-columns", "Checks virtual-policy changes preserve direct CSV cell identity.", "Direct-cell identity and added-virtual-only guards.", "-zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m -ausgabe --spaltenreihenfolgeundnurdiese=744,493 --virtualcolumns --breite=0"),
    BinaryInfo("rreta_arch_language_parity", "reta", "language", "Checks selected/effective language asset and direct 744 safety.", "Language parity status, requested/effective asset, failed guards.", "-language=english -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0"),
    BinaryInfo("rreta_arch_language_coverage", "reta", "language", "Reports which language CSVs cover requested direct columns.", "Per-language coverage, stale languages, missing columns.", "-language=english -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0"),
    BinaryInfo("rreta_arch_language_sync", "reta", "language", "Reports pending language CSV sync actions.", "Pending languages/columns/assets and sync readiness.", "-language=english -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0"),
    BinaryInfo("rreta_arch_prompt_language_completion", "reta", "prompt", "Shows language parameter/value completions for prompt text.", "Prompt completion candidates and language coverage/sync witness.", "--prompt-text 'reta -language=e'"),
    BinaryInfo("rreta_arch_prompt_language_guard", "reta", "prompt", "Checks prompt language completion/coverage/sync safety.", "Prompt language guard ready/blocked and failed guards.", "reta -language=english -spalten --kontinuum=m"),
    BinaryInfo("rreta_arch_prompt_language_commit", "reta", "prompt", "Checks prompt shadow commit with language guard.", "Prompt shadow report, commit policy, commit decision.", "--reta-arch=commit reta -language=english -spalten --kontinuum=m"),
    BinaryInfo("rreta_arch_prompt_activation_readiness", "reta", "prompt", "Folds prompt shadow commit and language guard into prompt readiness.", "Prompt activation readiness report and failed guards.", "--reta-arch=commit reta -language=english -spalten --kontinuum=m"),
    BinaryInfo("rretaprompt_arch_inspect", "retaprompt_frontends", "prompt", "Prompt-frontends inspect binary for retaprompt shadow/commit/readiness.", "Cleaned args, shadow report, commit/readiness JSON.", "--reta-arch=commit reta -language=english -spalten --kontinuum=m"),
]


def by_name(items: Iterable[BinaryInfo]) -> List[BinaryInfo]:
    return sorted(items, key=lambda item: (item.category, item.name))


def print_markdown(items: List[BinaryInfo]) -> None:
    print("| Binary | Kategorie | Zweck | Nützliche Ausgabe | Legacy-Lines? |")
    print("|---|---|---|---|---:|")
    for item in by_name(items):
        print(
            f"| `{item.name}` | {item.category} | {item.purpose} | {item.useful_output} | "
            f"{'ja' if item.needs_legacy_lines else 'nein'} |"
        )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--format", choices=["json", "markdown"], default="json")
    parser.add_argument("--category", action="append", help="Limit output to one or more categories")
    args = parser.parse_args()

    items = CATALOG
    if args.category:
        wanted = set(args.category)
        items = [item for item in items if item.category in wanted]

    if args.format == "markdown":
        print_markdown(items)
    else:
        print(json.dumps([asdict(item) for item in by_name(items)], ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
