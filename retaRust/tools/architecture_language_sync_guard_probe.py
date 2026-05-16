#!/usr/bin/env python3
"""Stage 63 probe: language sync is a hard activation guard.

Stage 62 synchronized translated religion CSV assets for column 744.  Stage 63
wires that synchronization witness into commit, audit, readiness and promotion
so a future localized activation cannot be promoted while translation sync
backlog actions are still pending.
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
        "shadow_report_carries_sync": "pub language_sync: TableViewLanguageSyncReport" in shadow,
        "shadow_policy_requires_sync": "pub require_language_sync_ready: bool" in shadow
        and "require_language_sync_ready: true" in shadow,
        "shadow_decision_carries_sync_fields": "pub language_sync_ready: bool" in shadow
        and "pub language_sync_pending_action_count: usize" in shadow
        and "pub language_sync_pending_languages: Vec<String>" in shadow,
        "shadow_commit_uses_sync_guard": "let language_sync_ok = !policy.require_language_sync_ready || report.language_sync.ready();" in shadow
        and "language_sync_blocked" in shadow
        and "language_sync_ok && size_ok" in shadow,
        "commit_audit_requires_sync_guard": "pub language_sync_ready: bool" in audit
        and "language_sync_ready" in audit
        and "language synchronization backlog must be empty" in audit
        and "language_sync: report.language_sync.clone()" in audit,
        "readiness_policy_requires_sync_guard": "pub require_language_sync_ready: bool" in readiness
        and "--activation-readiness-require-language-sync" in readiness
        and "language_sync_ready" in readiness,
        "promotion_policy_requires_sync_guard": "pub require_language_sync_ready: bool" in promotion
        and "--activation-promotion-require-language-sync" in promotion
        and "default promotion requires every language sync action" in promotion,
        "runtime_strips_sync_flags": "--activation-readiness-require-language-sync" in runtime
        and "--activation-promotion-require-language-sync" in runtime,
        "runtime_exposes_sync_guard_gates": "table_view_language_sync.commit_guard" in runtime
        and "table_view_activation_readiness.language_sync_guard" in runtime
        and "table_view_activation_promotion.language_sync_guard" in runtime,
        "migration_has_sync_guard_step": "step-table-view-language-sync-guard" in migration
        and "table_view_language_sync.commit_guard" in migration,
        "workflow_diagnostics_show_sync_guard": "sync_ready" in workflow
        and "sync_pending" in workflow,
    }
    failed = [name for name, ok in checks.items() if not ok]
    report = {
        "stage": 63,
        "status": "ok" if not failed else "failed",
        "checks": checks,
        "failed": failed,
        "guarded_path": [
            "ShadowTableViewOutputReport.language_sync",
            "ShadowTableViewOutputCommitDecision.language_sync_ready",
            "TableViewCommitAuditReport.language_sync_ready",
            "TableViewActivationReadinessReport.language_sync_ready",
            "TableViewActivationPromotionReport.language_sync_ready",
        ],
        "safe_after_stage62": "en/cn/vn/kr religion assets carry column 744, so sync pending actions should be zero",
        "universal_property": "language_sync_must_be_ready_before_a_synchronized_language_table_view_can_be_promoted",
    }
    print(json.dumps(report, ensure_ascii=False, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if not failed else 1


if __name__ == "__main__":
    raise SystemExit(main())
