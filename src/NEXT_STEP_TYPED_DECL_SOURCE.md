Nächster Schritt der Entstringifizierung:

1. Weitere Blöcke aus EXACT_HTML_META nach domain/typed_exact_decl.rs migrieren.
2. python_source_of_truth::exact_decl_meta_for_column nur noch typisierte Quelle + letzter Legacy-Fallback.
3. Danach HtmlDeclMeta::parse nur noch für Altlastenpfade behalten.
