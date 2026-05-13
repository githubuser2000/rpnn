# Stage 20 – Tabellen-Ausgabe als explizite Output-Morphismenschicht

Stage 20 baut direkt auf Stage 19 auf. Der Schwerpunkt war diesmal nicht mehr `lib4tables_concat.py`, sondern der nächste große Restblock in `libs/tableHandling.py`: die alte verschachtelte Klasse `Tables.Output`.

## Neu

- `reta_architecture/table_output.py`
  - neue explizite Schicht für Tabellen-Ausgabe und Renderer-nahe Morphismen
  - enthält jetzt:
    - `TableOutput`
    - `TableOutputBundle`
    - `bootstrap_table_output(...)`
  - übernimmt die bisherige Ausgabe-/Rendering-Logik aus `Tables.Output`
  - hält historische Ausgabe-Parität, trennt aber die globale Tabellen-Sektion von konkreter Darstellung

- `libs/tableHandling.py`
  - importiert jetzt `TableOutput` aus `reta_architecture.table_output`
  - besitzt keine eigene verschachtelte Klasse `Output` mehr
  - `Tables.__init__(...)` erzeugt `self.getOut = TableOutput(self, Txt)`
  - Umfang fiel von ca. **1539 Zeilen** auf ca. **881 Zeilen**

- `reta_architecture/facade.py`
  - besitzt jetzt `table_output`
  - neuer Bootstrap:
    - `RetaArchitecture.bootstrap_table_output(...)`
  - `snapshot()` enthält jetzt `table_output`
  - die Table-Output-Schicht wird bewusst lazy geladen, damit kein `center`/`reta_architecture`-Importzyklus entsteht

- `reta_architecture_probe_py.py`
  - neuer Befehl:
    - `table-output-json`

- `reta_architecture/package_integrity.py`
  - `reta_architecture/table_output.py` ist jetzt Pflichtdatei

- `tests/test_architecture_refactor.py`
  - neue Regression für `TableOutputBundle`
  - zusätzliche Prüfung, dass `tableHandling.py` an `reta_architecture.table_output` delegiert und keine eigene `class Output` mehr enthält
  - Paketmanifest erwartet jetzt `reta_architecture/table_output.py`

## Importzyklus-Korrektur

Beim ersten Herausziehen wurde sichtbar, dass eine direkte Top-Level-Einbindung von `table_output.py` in `reta_architecture.__init__`/`facade.py` einen Importzyklus erzeugt:

```text
center -> reta_architecture -> facade -> table_output -> center
```

Stage 20 löst das sauber:

- `RetaArchitecture` lädt `table_output` lazy in `bootstrap_table_output(...)`.
- `tableHandling.py` darf `TableOutput` direkt verwenden, weil dort `center` und `lib4tables` bereits im normalen Legacy-Pfad initialisiert sind.
- Die Architektur bleibt damit inspizierbar, ohne die Importreihenfolge zu destabilisieren.

## Architekturwirkung

Die Tabelle selbst wird damit stärker als globale Sektion/Garbe behandelt, während Ausgabeformate wie Shell, Markdown, HTML, CSV, Emacs und BBCode als konkrete Darstellungsmorphismen darüber liegen.

Das ist wichtig, weil die Ausgabelogik bisher tief in `tableHandling.py` verschachtelt war. Stage 20 macht daraus eine explizite Architekturschicht, ohne das historische Rendering-Verhalten umzuschreiben.

## Geprüft

- `py_compile` über alle Python-Dateien: OK
- `reta_architecture_probe_py.py table-output-json`: OK
- `reta_architecture_probe_py.py package-integrity-json`: OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- echter Smoke-Test für `Religionen/sternpolygon`: OK
- `python -B -S -m unittest tests.test_architecture_refactor -q`: **37 Tests, OK**
- `python -B -S -m unittest tests.test_command_parity -q`: **1 Paritätsmatrix-Test, OK**
- `python -B -S -m unittest -q`: **38 Tests, OK**

## Manuelle Parität gegen das Original

Zusätzlich wurden repräsentative Original-vs-Stage-20-Ausgaben verglichen:

- Shell-Ausgabe `Religionen/sternpolygon`: byte-identisch
- Markdown-Ausgabe `Religionen/sternpolygon`: byte-identisch
- Bruch-/CSV-Gluing-Fall `--gebrochenuniversum=5`: byte-identisch
- HTML-Ausgabe `Religionen/sternpolygon`: identisch nach derselben HTML-Normalisierung wie im Paritätstest

## Paketstatus

- Dateien im finalen Baum: **280**
- fehlende Pflichtdateien: **0**
- Runtime-Artefakte: **0**
- `csv/religion.csv`: **1043 Zeilen**
- `csv/vn-religion.csv`: **1043 Zeilen**
- Paket-Digest: über `package-integrity-json` reproduzierbar

## Noch offen

Der nächste große Architekturblock ist `libs/tableHandling.py` selbst:

- `Tables.Combi` sollte als Kombi-Join-/Relationen-Schicht herausgezogen werden.
- `Tables.Maintable.createSpalteGestirn(...)` kann als Generated-Column-Morphismus in `generated_columns.py` landen.
- Die Restklasse `Tables` sollte langfristig nur noch eine dünne globale Tabellen-Sektionsfassade sein.
