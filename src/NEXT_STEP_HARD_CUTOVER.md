Next hard cutover step:
- add more entries to typed_exact_decl.rs
- replace legacy_exact_decl_meta_for_column(...) body with None or panic to expose remaining gaps
- remove EXACT_HTML_META readers after typed coverage is complete
