Dieser Schritt typisiert HtmlDeclMeta weiter:

- p1_groups: Vec<HtmlP1Group>
- p2_slots: Vec<HtmlP2Slot>
- p4_tags: Vec<HtmlP4Tag>
- HtmlHeaderClass als künstlicher Typ für die gerenderte CSS-Klasse

Nächster harter Schritt:
- python_html_meta.rs von &str-Metadaten auf typisierte HtmlDeclMeta/HtmlHeaderClass umstellen
- exact_meta_for_column perspektivisch nicht mehr als freier String, sondern als typisierte Deklaration liefern
- String-basierte Sonderfälle in css_class_for_visible_header abbauen
