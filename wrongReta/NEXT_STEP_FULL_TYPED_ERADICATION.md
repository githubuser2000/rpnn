# Vollständige Ausrottung der String-Ersetzungen – nächster Stand

Dieser Schritt zieht weitere Kernpfade von String-Schreddern auf echte Typen um.

## Geändert

### `src/domain/spalten_anfrage.rs`
- `normalize_key(...).replace(...)` vollständig entfernt
- Alias-Auflösung läuft jetzt über `LegacyOberToken` und explizite Unter-Enums
- Eigenschaftsprüfung bleibt typisiert über `EigenschaftKeyId`

### `src/domain/python_html_meta.rs`
- keine Matches mehr gegen Legacy-String-Anfragen
- HTML-Meta-Matching läuft direkt gegen
  `domain::model::spalten_anfrage::SpaltenAnfrage`

### `src/table_printer/query.rs`
- Full-Table-Entscheidung nicht mehr über flache normalisierte Strings
- neue Typen:
  - `LegacyOberToken`
  - `GeneratedCommandToken`
  - `QueryHintToken`

### `src/domain/parser/legacy_cli_typed.rs`
- `LegacyOberToken` ist jetzt `Ord`/`PartialOrd`, damit typisierte Mengen in `BTreeSet` sauber funktionieren

### `src/domain/html_meta_builder.rs`
- der aggressive Zeichenschredder ist durch `fold_visible_text(...)` ersetzt
- noch nicht vollständig deklarativ, aber kein `_ - /`-Wegwerfen mehr in dieser Stelle

## Noch offen

Die letzten hybriden Reste sitzen jetzt hauptsächlich in zwei Bereichen:

1. `src/domain/html_meta_builder.rs`
   - dort wird noch aus sichtbaren Header-Texten auf Semantik geschlossen
   - Ziel: echter `HtmlHeaderSemantic`-Typ statt Textheuristik

2. `src/cli/cli_output.rs`
   - `normalize_meta_label(...)` baut CSS-Klassen noch aus Text um
   - Ziel: CSS-Klassen direkt aus strukturierten Meta-Tags

## Nächster sinnvoller Umbau

- `HtmlHeaderSemantic` als Enum mit künstlichen Untertypen
- `build_python_exact_html_class(...)` nur noch aus diesem Typ
- `normalize_meta_label(...)` vollständig entfernen
