Nächster Schritt der vollständigen Entstringifizierung:

1. `EXACT_HTML_META` selbst nicht mehr als `&[(u32, &str)]`, sondern als statische typisierte Tabelle.
2. `HtmlDeclMeta::parse(...)` nur noch als Legacy-Fallback für noch nicht migrierte Spalten.
3. `categories.rs` kann danach die alten `extract_*_from_meta(&str)`-Hilfen komplett verlieren.
4. Danach `html_meta_builder.rs` und `python_html_meta.rs` auf eine einzige typisierte Quelle zusammenziehen.
