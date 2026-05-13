# Stage 16: Table-Preparation-Schicht

Stage 16 baut direkt auf Stage 15 auf und zieht den nächsten alten Tabellenblock heraus: die Ausgabevorbereitung aus `libs/lib4tables_prepare.py`.

## Neu

- `reta_architecture/table_preparation.py`
  - neue explizite Schicht für lokale-zu-globale Tabellen-Ausgabevorbereitung
  - enthält jetzt:
    - `TablePreparationBundle`
    - `MainTablePreparationResult`
    - `KombiTablePreparationResult`
    - `bootstrap_table_preparation(...)`
    - `prepare_output_table(...)`
    - `select_display_lines(...)`
    - `prepare_row_cells(...)`
    - `tag_output_column(...)`
    - `cell_work(...)`
    - `deduplicate_parameter_sections(...)`
    - `capture_last_line_number(...)`
    - `prepare_main_output(...)`
    - `prepare_kombi_output(...)`

- `libs/lib4tables_prepare.py`
  - delegiert `prepare4out(...)`, `prepare4out_beforeForLoop_SpaltenZeilenBestimmen(...)`, `prepare4out_LoopBody(...)`, `prepare4out_Tagging(...)` und `cellWork(...)` jetzt an `reta_architecture.table_preparation`
  - bleibt als Legacy-kompatible `Prepare`-Fassade erhalten
  - fiel von ca. 1039 Zeilen auf ca. 836 Zeilen

- `reta_architecture/program_workflow.py`
  - nutzt jetzt `bootstrap_table_preparation()` für:
    - positive/negative Zeilen-Sektionsbereinigung
    - Haupttabellen-Ausgabevorbereitung
    - Kombi-Tabellen-Ausgabevorbereitung

- `reta_architecture/table_generation.py`
  - setzt `lastLineNumber` jetzt über `capture_last_line_number(...)` aus der neuen Table-Preparation-Schicht

- `RetaArchitecture`
  - besitzt jetzt `table_preparation`
  - neuer Bootstrap:
    - `RetaArchitecture.bootstrap_table_preparation(...)`
  - `snapshot()` enthält jetzt `table_preparation`

- `reta_architecture_probe_py.py`
  - neuer Befehl:
    - `table-preparation-json`

- `reta_architecture/package_integrity.py`
  - `reta_architecture/table_preparation.py` ist jetzt Pflichtdatei

## Architektonische Bedeutung

Damit ist `prepare4out(...)` nicht mehr nur eine alte Methode in einer großen Legacy-Klasse, sondern eine explizite Architekturzone:

- **Prägarben:** lokale Zeilen-/Spaltenauswahlen, die noch nicht global gerendert sind
- **Garben:** konsistente globale Ausgabetabelle als verklebte Sektion
- **Morphismen:** Zeilenfilterung, Zellenumbruch, Header-/Tag-Transport
- **Universelle Eigenschaften:** kanonische Vorbereitung der Haupttabelle und Kombi-Tabelle vor dem Join

Der wichtige Punkt ist nicht, dass der alte Algorithmus neu erfunden wurde. Er wurde als eigener Architektur-Knoten freigelegt, während die Legacy-Methodennamen weiterhin funktionieren.

## Geprüft

- `py_compile`: OK
- `tests.test_architecture_refactor`: 32 Tests, OK
- `tests.test_command_parity`: 1 Paritätsmatrix-Test, OK
- `reta_architecture_probe_py.py table-preparation-json`: OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- echter `reta.py`-Smoke-Test für Religionen/sternpolygon: OK

## Stabile Kernwerte

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `Religionen / Hinduismus -> [217]`

## Paketstatus

- Dateien: 267
- fehlende Pflichtdateien: 0
- Runtime-Artefakte: 0
- `csv/religion.csv`: 1043 Zeilen
- `csv/vn-religion.csv`: 1043 Zeilen

## Nächster sinnvoller Block

Der nächste harte Architekturblock ist jetzt nicht mehr `prepare4out`, sondern die großen Concat-/Generated-Column-Algorithmen in `libs/lib4tables_concat.py`. Dort liegen noch viele einzelne Spaltengeneratoren, die als eigene Generator-Morphismen oder Meta-Spalten-Garben sichtbar gemacht werden können.
