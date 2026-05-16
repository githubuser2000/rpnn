#!/usr/bin/env python3
"""Stage 59 probe: language parity is a hard commit/readiness/promotion guard.

Stage 58 introduced a language-parity witness.  Stage 59 wires that witness into
commit, audit, readiness and promotion so a localized CSV section cannot become
visible when it would drop a requested direct column such as 744.
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
        "shadow_policy_requires_language_parity": "pub require_language_parity_ready: bool" in shadow
        and "require_language_parity_ready: true" in shadow,
        "shadow_decision_carries_language_fields": "pub language_parity_ready: bool" in shadow
        and "pub language_effective_asset_name: String" in shadow
        and "pub language_failed_guards: Vec<String>" in shadow,
        "shadow_commit_uses_language_ok": "let language_ok = !policy.require_language_parity_ready || report.language_parity.ready();" in shadow
        and "language_ok" in shadow
        and "language_parity_blocked" in shadow,
        "commit_audit_requires_language_guard": "language_parity_ready" in audit
        and "localized table sections may commit only" in audit
        and "language_parity: report.language_parity.clone()" in audit,
        "readiness_policy_requires_language_guard": "pub require_language_parity_ready: bool" in readiness
        and "--activation-readiness-require-language-parity" in readiness
        and "language_parity_ready" in readiness,
        "promotion_policy_requires_language_guard": "pub require_language_parity_ready: bool" in promotion
        and "--activation-promotion-require-language-parity" in promotion
        and "localized materialization may be promoted only" in promotion,
        "runtime_strips_language_guard_flags": "--activation-readiness-require-language-parity" in runtime
        and "--activation-promotion-require-language-parity" in runtime,
        "runtime_exposes_language_guard_gates": "table_view_language_parity.commit_guard" in runtime
        and "table_view_activation_readiness.language_parity_guard" in runtime
        and "table_view_activation_promotion.language_parity_guard" in runtime,
        "migration_has_language_commit_guard_step": "step-table-view-language-commit-guard" in migration
        and "table_view_language_parity.commit_guard" in migration,
        "workflow_diagnostics_show_language_guard": "language_ready" in workflow
        and "language_asset" in workflow,
    }
    failed = [name for name, ok in checks.items() if not ok]
    report = {
        "stage": 59,
        "status": "ok" if not failed else "failed",
        "checks": checks,
        "failed": failed,
        "guarded_path": [
            "ShadowTableViewOutputReport.language_parity",
            "ShadowTableViewOutputCommitDecision.language_parity_ready",
            "TableViewCommitAuditReport.language_parity_ready",
            "TableViewActivationReadinessReport.language_parity_ready",
            "TableViewActivationPromotionReport.language_parity_ready",
        ],
        "blocked_example": "reta -language=english --no-language-fallback -spalten --kontinuum=m",
        "expected_block_reason": "language_parity_blocked / selected_744_not_materialized_as_direct_csv_cell",
    }
    print(json.dumps(report, ensure_ascii=False, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if not failed else 1


if __name__ == "__main__":
    raise SystemExit(main())
