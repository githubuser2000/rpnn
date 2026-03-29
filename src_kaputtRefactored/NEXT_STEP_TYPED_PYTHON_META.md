Nächster Schritt der Entstringifizierung:

1. EXACT_HTML_META nicht mehr als &(u32, &str), sondern als typisierte HtmlDeclMeta-Tabelle generieren.
2. HtmlDeclMeta::parse dann nur noch für Altlasten behalten.
3. categories.rs und reverse_request_report.rs auf exact_decl_meta_for_column(...) umstellen.
4. HtmlP1Group und HtmlSlotLabel weiter vervollständigen, bis parsefreie Pfade dominieren.
