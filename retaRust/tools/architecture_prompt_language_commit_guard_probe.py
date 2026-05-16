#!/usr/bin/env python3
"""Static probe for Stage 66 prompt language guard commit integration."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def contains(path: str, needle: str) -> bool:
    return needle in (ROOT / path).read_text(encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    shadow = (ROOT / "crates/reta_architecture/src/shadow_pipeline.rs").read_text(encoding="utf-8")
    runtime = (ROOT / "crates/reta_architecture/src/runtime_switch.rs").read_text(encoding="utf-8")
    migration = (ROOT / "crates/reta_architecture/src/migration_control.rs").read_text(encoding="utf-8")
    cargo = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    inspect = (ROOT / "crates/retaprompt_frontends/src/bin/retaprompt_arch_inspect.rs").read_text(encoding="utf-8")
    binary = (ROOT / "src/bin/reta_arch_prompt_language_commit.rs").read_text(encoding="utf-8")

    checks = {
        "shadow_prompt_report_carries_guard": "pub prompt_language_guard: PromptLanguageGuardReport" in shadow,
        "commit_policy_requires_guard": "pub require_prompt_language_guard_ready: bool" in shadow,
        "commit_decision_reports_guard": "pub prompt_language_guard_failed_guards: Vec<String>" in shadow,
        "shadow_prompt_computes_guard": "prompt_language_guard_for_text(" in shadow and "commit_candidate: gate.allowed_to_commit && prompt_language_guard_ready" in shadow,
        "commit_blocks_unready_guard": "prompt_language_guard_not_ready" in shadow and "language_guard_ok" in shadow,
        "policy_from_cli_present": "impl ShadowPromptCommitPolicy" in shadow and "from_cli_args" in shadow,
        "runtime_gates_present": "shadow_pipeline.prompt_language_guard_commit" in runtime and "prompt_language_guard.shadow_prompt_commit_guard" in runtime,
        "runtime_strips_commit_flags": "--prompt-language-commit-ignore-guard" in runtime and "--prompt-commit-allow-force" in runtime,
        "migration_step_present": "step-prompt-language-commit-guard" in migration,
        "retaprompt_inspect_uses_policy": "ShadowPromptCommitPolicy::from_cli_args" in inspect and "prompt_commit_policy" in inspect,
        "root_binary_registered": "rreta_arch_prompt_language_commit" in cargo,
        "root_binary_present": "evaluate_shadow_prompt_commit" in binary and "PromptLanguageCommitInspect" in binary,
        "test_added": "prompt_commit_requires_language_guard_by_default" in shadow,
    }
    status = "ok" if all(checks.values()) else "failed"
    result = {
        "stage": 66,
        "status": status,
        "checks": checks,
        "failed": [name for name, ok in checks.items() if not ok],
    }
    print(json.dumps(result, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if status == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
