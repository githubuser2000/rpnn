# Transcompilation Architecture Stage 18

Stage 18 moves the Rust port from the Stage-17 flattened alias-to-column matrix to a richer alias-to-bucket matrix.

## Main change

`python_arch_reference/i18n/words_matrix.py::paraNdataMatrix` is now generated into Rust with:

- flattened integer column projection, as before;
- exact legacy bucket index for each projection;
- symbolic bucket payloads such as `primMotivStern`, `PrimCSV`, and gebrochen-rational string selectors like `"2"`.

This matters because old Reta does not store every `-spalten` value as an ordinary integer column. Some values are generated-column selectors, concat/CSV selectors, Kombi selectors, or gebrochen-rational local sections.

## Files changed

```text
crates/reta_architecture/src/facade.rs
crates/reta_architecture/src/parameter_matrix.rs
crates/reta_architecture/src/parameter_runtime.rs
crates/reta_architecture/src/semantics_builder.rs
crates/reta_architecture/src/sheaf.rs
crates/reta_architecture/src/table_generation.rs
tools/generate_parameter_matrix.py
```

## Important new Rust surfaces

```text
ParameterBucketProjection
OwnedParameterBucketProjection
bucket_projections_for_alias_pair
symbolic_bucket_projection_count
nonempty_bucket_projection_count
ParameterCommandSets::column_buckets
ParameterCommandSets::symbolic_column_buckets
ParameterCommandSets::excluded_symbolic_column_buckets
produce_all_column_bucket_numbers
produce_all_symbolic_column_buckets
TableGenerationPlan::from_parameter_command_sets
TableGenerationPlan::symbolic_column_buckets
```

## Regression cases kept

```text
-spalten --kontinuum=m -> ordinary bucket 0 contains 493 and 744
-spalten --multiplikationen=motivstern -> bucket 7 contains primMotivStern
-spalten --gebrochenuniversum=2 -> bucket 5 contains symbolic selector "2"
-spalten --gebrochenuniversum=2,-2 -> symbolic selector "2" is removed by negation
```

## Performance/semantics fix

`semantics_builder::build()` now constructs sparse reverse dictionaries from the actual matrix entries instead of materialising every known column against every parameter pair. That made the architecture tests complete quickly again while keeping the `744` projection visible.

`bootstrap_sheaves(None)` now boots from `bootstrap_schema()` instead of an empty sheaf, so the default sheaf resolves generated matrix aliases such as `kontinuum/m`.
