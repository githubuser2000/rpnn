# RPnn Refactor Stage 3

Diese Stufe zerlegt die bisher noch sehr dichten Module weiter, ohne die Fachlogik absichtlich zu reduzieren.

## Ziel

Die Komplexität bleibt fachlich erhalten, aber sie wird räumlich besser verteilt:

- weniger God-Files
- mehr lokale Zuständigkeiten
- klarere Verantwortlichkeiten
- bessere Basis für weitere Extraktion und Tests

## Neue Zerlegungen

### CLI

Die frühere große Datei `src/cli/parser.rs` wurde in kleinere Bausteine aufgeteilt:

- `src/cli/parser.rs`
  - hält jetzt primär den Hauptfluss des Parsens
- `src/cli/parser_types.rs`
  - `SpaltenNamen`, `SpaltenNamenListe`
- `src/cli/parser_support.rs`
  - Listenparser, Flag-Helfer, PyPy-Kompatibilitätsparser, Kategorie-Hilfs-Ausgaben
- `src/cli/parser_ranges.rs`
  - Parsen von Zeilenbereichsangaben

### Table Printer

Die frühere große Datei `src/table_printer/printer.rs` wurde entlang echter Teilverantwortungen zerlegt:

- `src/table_printer/printer.rs`
  - Orchestrierung des Renderings
- `src/table_printer/sanitize.rs`
  - Zell- und Header-Bereinigung
- `src/table_printer/meta_columns.rs`
  - Potenz-/Zeilen-Metaspalten
- `src/table_printer/widths.rs`
  - Breitenberechnung, Chunking-Budget, explizite Breitenlogik

## Warum das besser ist

Vorher lagen in einzelnen Dateien gleichzeitig:

- Domänenlogik
- Datenbereinigung
- Budget-/Breitenheuristiken
- Hilfsparser
- Ausgabehilfen
- Datentypen

Das macht Änderungen riskant, weil man einen Bereich berührt und ungewollt einen anderen beschädigt.

Die neue Form trennt jetzt mindestens diese Achsen:

- **Typen**
- **Parser-Hilfen**
- **Bereichsparser**
- **Sanitizing**
- **Meta-Spalten-Logik**
- **Breiten-/Chunk-Heuristik**
- **Orchestrierung**

## Noch offene dicke Module

Stage 3 ist nützlich, aber noch nicht das Ende. Die nächsten lohnenden Kandidaten sind weiterhin:

- `src/processing/kategorie_verarbeiter.rs`
- `src/processing/spalten_verarbeiter.rs`
- `src/domain/generator_registry.rs`
- `src/domain/categories.rs`
- `src/lib4tables_concat.rs`
- `src/table_printer/query.rs`

## Sinnvolle Stage 4

Die nächste harte, wirklich lohnende Stufe wäre:

1. `kategorie_verarbeiter.rs` zerlegen in
   - Normalisierung
   - Alias-Mapping
   - Generator-Inferenz
   - direkte Kategorieauflösung

2. `spalten_verarbeiter.rs` zerlegen in
   - CLI→Bereich-Übernahme
   - exakte Kategorien
   - Generator-Spalten
   - Sichtbarkeits-/Reihenfolgelogik

3. `generator_registry.rs` in registrierte Generatorgruppen splitten
   - Prim-/Vielfach-Generatoren
   - Bedeutungs-/ProContra-Generatoren
   - Struktur-/Kosmos-Generatoren

4. `domain/categories.rs` nicht nur verschieben, sondern intern aufteilen
   - Datentypen
   - Lookup-Indizes
   - Normalisierung
   - Suchstrategien

## Wichtiger Hinweis

Diese Refaktor-Stufe ist strukturell gebaut. In dieser Umgebung war kein `cargo`/`rustc` verfügbar, daher konnte kein lokaler Gegenbau erfolgen.
