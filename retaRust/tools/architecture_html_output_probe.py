#!/usr/bin/env python3
"""Stage-31 HTML attribute/output wiring probe.

This is intentionally dependency-free and does not require a Rust build.  It
checks that the htmlclassesPy witness catalog is now connected to the typed
TableViewOutput path through a disabled-by-default policy and explicit gates.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def load_jsonl(path: Path):
    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if line:
            yield json.loads(line)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    html_jsonl = ROOT / "python_arch_reference" / "htmlclassesPy.jsonl"
    records = list(load_jsonl(html_jsonl))
    rec_493 = next((r for r in records if r.get("column_number") == 493 and r.get("row_number") == 0), None)
    rec_m_kontinuum = next((r for r in records if r.get("row_number") == 0 and "M Kontinuum" in r.get("text", "")), None)
    rec_744 = next((r for r in records if r.get("column_number") == 744), None)

    module = (ROOT / "crates" / "reta_architecture" / "src" / "table_view_html_attributes.rs").read_text(encoding="utf-8")
    output = (ROOT / "crates" / "reta_architecture" / "src" / "table_view_output.rs").read_text(encoding="utf-8")
    lib = (ROOT / "crates" / "reta_architecture" / "src" / "lib.rs").read_text(encoding="utf-8")
    runtime = (ROOT / "crates" / "reta_architecture" / "src" / "runtime_switch.rs").read_text(encoding="utf-8")
    migration = (ROOT / "crates" / "reta_architecture" / "src" / "migration_control.rs").read_text(encoding="utf-8")
    ffi = (ROOT / "src" / "ffi.rs").read_text(encoding="utf-8")
    cargo = (ROOT / "Cargo.toml").read_text(encoding="utf-8")

    checks = {
        "module_exists": "TableViewHtmlAttributeConfig" in module,
        "policies_present": all(name in module for name in ["ClassOnly", "RawOpenTag", "RawHtmlWitness"]),
        "default_disabled": "enabled: false" in module and "policy: TableViewHtmlAttributePolicy::Plain" in module,
        "output_config_field": "html_attributes: TableViewHtmlAttributeConfig" in output,
        "output_report_fields": all(name in output for name in ["html_attribute_enabled", "html_attribute_policy", "html_attribute_report"]),
        "cli_flags": all(flag in output for flag in ["htmlclasses", "htmlrawclasses", "htmlclasswitness"]),
        "html_renderer_wired": "render_html_table_with_attributes" in output,
        "row_text_match_wired": "find_html_record_by_row_text" in module,
        "lib_exported": "pub mod table_view_html_attributes" in lib and "TableViewHtmlAttributeConfig" in lib,
        "runtime_gates": all(name in runtime for name in ["table_view_html_attributes.class_projection", "table_view_html_attributes.raw_open_tag", "table_view_html_attributes.raw_html_witness"]),
        "migration_step": "step-table-view-html-attributes" in migration,
        "ffi_export": "reta_architecture_table_view_html_attributes_json" in ffi,
        "inspect_binary": "rreta_arch_html_output" in cargo and (ROOT / "src" / "bin" / "reta_arch_html_output.rs").exists(),
        "html_catalog_493_present": rec_493 is not None,
        "html_catalog_493_class_has_r493": bool(rec_493 and "r_493" in rec_493.get("class_string", "")),
        "html_catalog_m_kontinuum_present": rec_m_kontinuum is not None,
        "html_catalog_m_kontinuum_class_present": bool(rec_m_kontinuum and rec_m_kontinuum.get("class_string")),
        "html_catalog_744_present": rec_744 is not None,
        "html_catalog_744_text_question": bool(rec_744 and rec_744.get("text") == "?"),
    }
    result = {
        "stage": 31,
        "status": "ok" if all(checks.values()) else "failed",
        "checks": checks,
        "html_catalog_record_count": len(records),
        "column_493_text_preview": (rec_493 or {}).get("text", "")[:120],
        "m_kontinuum_html_column": (rec_m_kontinuum or {}).get("column_number"),
        "m_kontinuum_text_preview": (rec_m_kontinuum or {}).get("text", "")[:120],
        "column_744_text": (rec_744 or {}).get("text"),
        "universal_property": "HTML class witnesses are explicit local attributes and remain disabled until policy/gate activation",
    }
    print(json.dumps(result, ensure_ascii=False, indent=2 if args.pretty else None, sort_keys=True))


if __name__ == "__main__":
    main()
