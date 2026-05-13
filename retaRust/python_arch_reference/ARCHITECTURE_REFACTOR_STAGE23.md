# Reta Topology Architecture Refactor – Stage 23

Stage 23 setzt den begonnenen Umbau fort: Reta wird weiter in explizite Architekturschichten für Topologie, Prägarben, Garben, Morphismen und universelle Konstruktionen zerlegt.

## Kernänderung

Nach Stage 22 waren Zeilenfilter und große Teile der Tabellenlogik bereits ausgelagert. Stage 23 zieht zwei kleinere, aber strukturell wichtige Restkerne aus dem Legacybereich heraus:

1. **Textumbruch/Breitenlogik** als `table_wrapping`
2. **Zahlen-/Prim-/Mond-/Vielfachheitslogik** als `number_theory`

Diese beiden Bereiche wirken klein, aber sie lagen vorher quer durch Tabellenaufbereitung, Zeilenfilterung, Renderer und generierte Spalten. Genau solche Querverbindungen machen Legacy-Code schwer entflechtbar.

## Neue Schicht: Table Wrapping

`reta_architecture/table_wrapping.py` modelliert Wrapping und Breitenbestimmung als Renderer-nahe Morphismen.

Architektonische Rolle:

- Morphismus zwischen Zellinhalt und gerenderten Zellfragmenten
- Kontextabhängige Breitenwahl für Tabellenzeilen
- Kompatibilitätsbrücke zur historischen `Prepare`-Klasse

## Neue Schicht: Number Theory

`reta_architecture/number_theory.py` modelliert die arithmetische Hilfssemantik als reine Morphismen-Schicht.

Architektonische Rolle:

- dependency-light Kern für Zahlenmorphismen
- keine Abhängigkeit von CLI, Tabellen, i18n oder Renderern
- gemeinsame Grundlage für:
  - Zeilenfilterung
  - Meta-Spalten
  - Generated Columns
  - Tabellen-Output

## Wirkung auf die Gesamtarchitektur

Die Reta-Architektur enthält damit jetzt explizit:

- `table_wrapping`
- `number_theory`

Beide sind über `RetaArchitecture.snapshot()` inspizierbar und über das Probe-Werkzeug direkt abfragbar.

## Stand nach Stage 23

Der Legacy-Kern ist weiter geschrumpft. `libs/lib4tables_prepare.py` ist stärker Fassadenmodul geworden; `row_filtering.py`, `table_preparation.py` und `table_wrapping.py` tragen die eigentliche Architektur. Die Zahlenlogik ist als saubere Morphismen-Schicht vorhanden, auch wenn `lib4tables.py` aus Kompatibilitätsgründen noch eigene Exporte besitzt.

## Nächste sinnvolle Schritte

1. `libs/lib4tables.py` weiter zerlegen:
   - Ausgabe-Syntaxklassen in eine klarere Output-Syntax-Schicht
   - alte Zahlenfunktionen auf Compatibility-Wrapper reduzieren

2. `reta_architecture/table_output.py` weiter vom Legacy-Output-Syntax-Modul entkoppeln.

3. Die Command-Parität weiter ausbauen, vor allem für:
   - HTML
   - BBCode
   - Emacs/Org-artige Ausgabe
   - Kombi-Tabellen
   - Prompt-Fälle
