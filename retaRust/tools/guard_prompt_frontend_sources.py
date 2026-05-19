#!/usr/bin/env python3
"""Prevent rrp/rrpl/rrpe/rrpb from regressing into heavy Rust frontends.

The public prompt executables are ABI launchers.  All interactive behavior,
autocomplete/autosuggest logic, command parsing and command execution must live
behind libretaprompt_input.so and/or libretaprompt_commands.so.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover - Python < 3.11 fallback
    tomllib = None  # type: ignore[assignment]

ROOT = Path(__file__).resolve().parents[1]
PUBLIC_BINS = {
    "rp.rs": ("retaprompt_input", "retaprompt_commands"),
    "rpl.rs": ("retaprompt_input", "retaprompt_commands"),
    "rpe.rs": ("retaprompt_input", "retaprompt_commands"),
    "rpb.rs": ("retaprompt_commands",),
}
FORBIDDEN_RUST_API_PATTERNS = [
    re.compile(r"\bretaprompt_input::"),
    re.compile(r"\bretaprompt_commands::"),
    re.compile(r"\buse\s+retaprompt_input\b"),
    re.compile(r"\buse\s+retaprompt_commands\b"),
]
LEGACY_PROMPT_FRONTEND_PATHS = [
    ROOT / "src" / "bin" / "rp.rs",
    ROOT / "src" / "bin" / "rpl.rs",
    ROOT / "src" / "bin" / "rpe.rs",
    ROOT / "src" / "bin" / "rpb.rs",
    ROOT / "crates" / "retaprompt",
    ROOT / "crates" / "retaprompt_frontends" / "src" / "bin" / "retaprompt_launcher.rs",
]


def fail(message: str) -> None:
    print(f"prompt frontend source guard failed: {message}", file=sys.stderr)
    sys.exit(1)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing expected file: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def load_toml(path: Path) -> dict:
    text = read(path)
    if tomllib is not None:
        return tomllib.loads(text)
    # Tiny fallback that is enough for this guard on older Python versions:
    # it extracts workspace.default-members from the root Cargo.toml.
    match = re.search(r"(?ms)^\[workspace\]\s*(.*?)(?:^\[|\Z)", text)
    if not match:
        return {}
    section = match.group(1)
    members_match = re.search(r"(?ms)^default-members\s*=\s*\[(.*?)\]", section)
    if not members_match:
        return {"workspace": {}}
    entries = re.findall(r'"([^"]+)"', members_match.group(1))
    return {"workspace": {"default-members": entries}}


def guard_no_legacy_prompt_frontends() -> None:
    for path in LEGACY_PROMPT_FRONTEND_PATHS:
        if path.exists():
            fail(
                f"legacy prompt frontend path still exists: {path.relative_to(ROOT)}. "
                "Delete it; public rrp/rrpl/rrpe/rrpb must live only in "
                "crates/retaprompt_frontends as thin ABI launchers."
            )


def guard_public_bin(filename: str, required_links: tuple[str, ...]) -> None:
    path = ROOT / "crates" / "retaprompt_frontends" / "src" / "bin" / filename
    text = read(path)

    for pattern in FORBIDDEN_RUST_API_PATTERNS:
        if pattern.search(text):
            fail(
                f"{path.relative_to(ROOT)} calls a retaprompt Rust crate API. "
                "Use only extern \"C\" ABI symbols so behavior remains in the .so libraries."
            )

    if "extern \"C\"" not in text:
        fail(f"{path.relative_to(ROOT)} is not an ABI launcher; missing extern \"C\" block")

    for library in required_links:
        expected = f'#[link(name = "{library}")]'
        if expected not in text:
            fail(f"{path.relative_to(ROOT)} is missing direct ABI link attribute {expected}")

    if filename == "rpb.rs" and "retaprompt_input" in text:
        fail("rrpb must be command-only and must not reference retaprompt_input")

    if filename in {"rp.rs", "rpl.rs", "rpe.rs"}:
        if "retain_direct_dependency" not in text:
            fail(f"{path.relative_to(ROOT)} must retain the direct retaprompt_commands.so dependency")


def guard_frontend_cargo_toml() -> None:
    path = ROOT / "crates" / "retaprompt_frontends" / "Cargo.toml"
    text = read(path)

    if "retaprompt_input = { path = \"../retaprompt_input\", optional = true }" not in text:
        fail("retaprompt_input must remain optional in retaprompt_frontends; public launchers must not embed it")
    if "retaprompt_commands = { path = \"../retaprompt_commands\", optional = true }" not in text:
        fail("retaprompt_commands must remain optional in retaprompt_frontends; public launchers must not embed it")
    if 'required-features = ["arch-inspect"]' not in text:
        fail("rretaprompt_arch_inspect must stay behind arch-inspect so normal frontend builds remain thin")
    if "retaprompt_launcher" in text:
        fail("retaprompt_launcher is retired; keep only rrp/rrpl/rrpe/rrpb plus gated diagnostics")


def guard_workspace_default_members() -> None:
    data = load_toml(ROOT / "Cargo.toml")
    default_members = data.get("workspace", {}).get("default-members", [])
    if "crates/retaprompt_frontends" not in default_members:
        fail(
            "crates/retaprompt_frontends must stay in workspace.default-members so "
            "`cargo run --bin rrpb -- -h` works from the repository root. "
            "The size guard keeps these targets thin."
        )


def main() -> None:
    guard_no_legacy_prompt_frontends()
    for filename, required_links in PUBLIC_BINS.items():
        guard_public_bin(filename, required_links)
    guard_frontend_cargo_toml()
    guard_workspace_default_members()
    print("prompt frontend source guard passed")


if __name__ == "__main__":
    main()
