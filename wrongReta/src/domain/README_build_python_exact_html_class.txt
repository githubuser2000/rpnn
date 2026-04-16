Replace these files in your project:
- src/domain/html_meta_builder.rs
- src/domain/html_header_model.rs

This patch adds:
- fn build_python_exact_html_class(raw: &str, col_idx: usize, is_header_row: bool) -> Option<String>

Behavior:
- keeps first two HTML header classes exact
- for other columns, resolves classes using the richer of:
  - exact_meta_for_column via COL / ID
  - python_html_meta via visible header text
- strips trailing (ID_xxx) from visible text when a class was resolved

No runtime Python is used.
