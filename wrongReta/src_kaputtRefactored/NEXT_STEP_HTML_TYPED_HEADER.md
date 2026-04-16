Dieser Schritt ersetzt den HTML-Header-Klassenpfad weiter durch künstliche Typen.

Geändert:
- `HtmlHeaderClass` als typisierte Repräsentation einer Header-CSS-Klasse
- `resolve_html_header_class(...)` statt nur roher String-Erzeugung
- generierte Eigenschafts-Slots verwenden `EigenschaftKeyId::canonical_name()` direkt
- tote `normalize_meta_label(...)`-Stringersetzungen in beiden CLI-HTML-Ausgaben entfernt

Nächster harter Schritt:
- `HtmlDeclMeta` selbst typisieren, also `Vec<String>` / `Option<String>` durch künstliche Slot-Enums ersetzen
- `python_html_meta.rs` weg von sichtbarem Text-Matching hin zu deklarativen Typ-Mappings
