Rust/Cargo:
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)

Real workspace check (expected blocked by missing crates.io DNS/cache):
error: no matching package named `indexmap` found
location searched: crates.io index
required by package `reta v0.6.0 (/mnt/data/work_stage5/retaRust)`
As a reminder, you're using offline mode (--offline) which can sometimes cause surprising resolution failures, if this error is too confusing you may wish to retry without `--offline`.

Isolated reta_architecture check with local serde stub:

running 49 tests
.................................................
test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

