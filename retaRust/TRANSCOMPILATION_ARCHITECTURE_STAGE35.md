# Transcompilation Architecture Stage 35

Stage 35 extends the `table_view_output_parity` layer so styled HTML/BBCode output can be compared semantically without weakening the raw output commit guard.

## Main change

The parity layer now supports style-aware markup normalization:

- HTML documents are parsed as whole documents, not only independent lines.
- Multi-line `<td ...> ... </td>` cells normalize to the same cells as compact HTML lines.
- BBCode cells with attributes like `[td="..."]...[/td]` normalize like plain `[td]...[/td]` cells.
- Styled rows such as `<tr style="...">` or `[tr="..."]` are counted as style wrappers.
- `raw_equal` remains the only commit-safe condition.

## New / changed Rust surfaces

```text
crates/reta_architecture/src/table_view_output_parity.rs
src/bin/reta_arch_style_parity.rs
src/ffi.rs
```

New exported function:

```text
parse_markup_document_rows
```

New FFI export:

```text
reta_architecture_table_view_style_parity_json
```

New inspect binary:

```text
rreta_arch_style_parity
```

## Runtime and migration gates

New gates:

```text
table_view_style_parity.markup_document_normalize
table_view_style_parity.bbcode_styled_td
table_view_style_parity.raw_commit_guard
```

New migration step:

```text
step-table-view-style-parity
```

## Safety property

```text
plain output
styled HTML/BBCode output
        ↓
style-aware semantic normalization
        ↓
same semantic table cells
        ↓
raw line diff still guards visible commit
```

Stage 35 does not switch visible output behavior. It improves diagnostic parity for styled shadow output.
