# Stage 11 checks

Performed in this container:

- `python3 -m py_compile tools/architecture_shadow_probe.py tools/architecture_commit_probe.py` passed.
- `git diff --check` passed.
- `Cargo.toml` / architecture Cargo manifests were inspected and unchanged except for no new dependency requirement.
- Static symbol checks found the new commit policy/decision exports and root bridge usage.

Not performed here:

- `cargo check`
- `cargo test`
- full workspace build

Reason: this container currently has no `cargo` and no `rustc` executable available.

Recommended local checks:

```bash
cargo check -p reta_architecture
cargo test -p reta_architecture
cargo check -p reta --lib
cargo check -p retaprompt_commands --lib
python3 tools/architecture_commit_probe.py --rust target/debug/rreta
```
