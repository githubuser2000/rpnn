Nächster Schritt:
- EXACT_HTML_META selbst nicht mehr als &(u32, &str), sondern als echte HtmlDeclMeta-Tabelle pflegen.
- HtmlDeclMeta::parse() dann nur noch als Übergangs-Fallback.
- Kategorien, HTML-Header und Python-Source-of-Truth nur noch über typisierte Deklarationen laufen lassen.
