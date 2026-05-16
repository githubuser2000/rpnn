#!/usr/bin/env python3
"""Stage 61 probe: language coverage is a hard activation guard.

Stage 60 exposed language CSV coverage as a diagnostic witness. Stage 61 wires
that witness into commit, audit, readiness and promotion so stale localized CSV
assets cannot be promoted unless base fallback safely covers every requested
direct column such as 744.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "crates" / "reta_architecture" / "src"


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="replace")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    shadow = read(SRC / "shadow_pipeline.rs")
    audit = read(SRC / "table_view_commit_audit.rs")
    readiness = read(SRC / "table_view_activation_readiness.rs")
    promotion = read(SRC / "table_view_activation_promotion.rs")
    runtime = read(SRC / "runtime_switch.rs")
    migration = read(SRC / "migration_control.rs")
    workflow = read(ROOT / "src" / "reta_workflow_py.rs")

    checks = {
        "shadow_policy_requires_language_coverage": "pub require_language_coverage_ready: bool" in shadow
        and "require_language_coverage_ready: true" in shadow,
        "shadow_decision_carries_coverage_fields": "pub language_coverage_ready: bool" in shadow
        and "pub language_coverage_status: String" in shadow
        and "pub language_coverage_stale_language_count: usize" in shadow
        and "pub language_coverage_failed_guards: Vec<String>" in shadow,
        "shadow_commit_uses_coverage_guard": "let language_coverage_ok = !policy.require_language_coverage_ready || report.language_coverage.ready();" in shadow
        and "gate_ok && diff_ok && virtual_direct_ok && language_ok && language_coverage_ok" in shadow
        and "language_coverage_blocked" in shadow,
        "commit_audit_requires_coverage_guard": "pub language_coverage_ready: bool" in audit
        and "language_coverage_ready" in audit
        and "language coverage must show" in audit
        and "language_coverage: report.language_coverage.clone()" in audit,
        "readiness_policy_requires_coverage_guard": "pub require_language_coverage_ready: bool" in readiness
        and "--activation-readiness-require-language-coverage" in readiness
        and "language_coverage_ready" in readiness,
        "promotion_policy_requires_coverage_guard": "pub require_language_coverage_ready: bool" in promotion
        and "--activation-promotion-require-language-coverage" in promotion
        and "localized materialization may become default only" in promotion,
        "runtime_strips_coverage_guard_flags": "--activation-readiness-require-language-coverage" in runtime
        and "--activation-promotion-require-language-coverage" in runtime,
        "runtime_exposes_coverage_guard_gates": "table_view_language_coverage.commit_guard" in runtime
        and "table_view_activation_readiness.language_coverage_guard" in runtime
        and "table_view_activation_promotion.language_coverage_guard" in runtime,
        "migration_has_coverage_guard_step": "step-table-view-language-coverage-guard" in migration
        and "table_view_language_coverage.commit_guard" in migration,
        "workflow_diagnostics_show_coverage_guard": "coverage_ready" in workflow
        and "coverage_status" in workflow
        and "stale_languages" in workflow,
    }
    failed = [name for name, ok in checks.items() if not ok]
    report = {
        "stage": 61,
        "status": "ok" if not failed else "failed",
        "checks": checks,
        "failed": failed,
        "guarded_path": [
            "ShadowTableViewOutputReport.language_coverage",
            "ShadowTableViewOutputCommitDecision.language_coverage_ready",
            "TableViewCommitAuditReport.language_coverage_ready",
            "TableViewActivationReadinessReport.language_coverage_ready",
            "TableViewActivationPromotionReport.language_coverage_ready",
        ],
        "blocked_example": "reta -language=english --no-language-fallback -spalten --kontinuum=m",
        "safe_example": "reta -language=english -spalten --kontinuum=m  # falls back to base religion.csv",
        "universal_property": "language_coverage_must_be_ready_before_a_localized_view_output_can_be_promoted",
    }
    print(json.dumps(report, ensure_ascii=False, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if not failed else 1


if __name__ == "__main__":
    raise SystemExit(main())
