# Stage 22: Zeilenfilter-/Bereichsmorphismen aus `lib4tables_prepare.py` herausgezogen

Stage 22 baut direkt auf Stage 21 auf. Der nächste tiefe Restblock war die Zeilenlogik in `libs/lib4tables_prepare.py`: Bereichsausdrücke, Zählungsgruppen, Mond/Sonne/Planet-Filter, Prim-Multiple, Potenzen und Positionsfilter wurden aus der alten `Prepare`-Klasse in eine explizite Architekturschicht verschoben.

## Neu

- `reta_architecture/row_filtering.py`
  - neue explizite Morphismenschicht für Zeilenauswahl und Bereichsfilter
  - enthält jetzt:
    - `RowFilteringBundle`
    - `bootstrap_row_filtering(...)`
    - `parameters_cmd_with_some_bereich(...)`
    - `filter_original_lines(...)`
    - `set_zaehlungen(...)`
    - `moonsun(...)`
    - `delete_doubles_in_sets(...)`
    - `from_until(...)`
    - `zeile_which_zaehlung(...)`

## Umgebaut

- `libs/lib4tables_prepare.py`
  - delegiert die alten Zeilenfilter-Methoden jetzt an `reta_architecture.row_filtering`
  - bleibt als Legacy-kompatible `Prepare`-Fassade erhalten
  - fiel von ca. **836 Zeilen** auf ca. **362 Zeilen**

- `RetaArchitecture`
  - neuer Lazy-Bootstrap:
    - `bootstrap_row_filtering(...)`
  - `snapshot()` enthält jetzt:
    - `row_filtering`

- `reta_architecture_probe_py.py`
  - neuer Befehl:
    - `row-filtering-json`

- `reta_architecture/package_integrity.py`
  - `reta_architecture/row_filtering.py` ist jetzt Pflichtdatei

- `tests/test_architecture_refactor.py`
  - neue Regressionen für die Row-Filtering-Schicht
  - prüft, dass `Prepare` an `row_filtering` delegiert

## Geprüft

- `py_compile`: OK
- `python -B -S -m unittest tests.test_architecture_refactor -v`: **41 Tests, OK**
- `reta_architecture_probe_py.py row-filtering-json`: OK
- `reta_architecture_probe_py.py package-integrity-json`: OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- Smoke-Tests des neuen Stands:
  - Shell `Religionen/sternpolygon`: OK
  - Markdown `Religionen/sternpolygon`: OK
  - HTML `Religionen/sternpolygon`: OK
  - Bruch-/CSV-Gluing `--gebrochenuniversum=5`: OK
  - Meta-Spalten `--universummetakonkret=meta`: OK

## Parität gegen Stage 21

Direkt gegen Stage 21 verglichen:

- Shell-Ausgabe `Religionen/sternpolygon`: byte-identisch
- Markdown-Ausgabe `Religionen/sternpolygon`: byte-identisch
- HTML-Ausgabe `Religionen/sternpolygon`: identisch nach derselben HTML-Normalisierung wie bisher
- Bruch-/CSV-Gluing `--gebrochenuniversum=5`: byte-identisch
- Meta-Spalten `--universummetakonkret=meta`: byte-identisch

## Nicht behauptet

Die große Original-vs-Refactor-Paritätsmatrix wurde in diesem Schritt nicht als finale Aussage verwendet. Stattdessen wurde die relevante Nicht-Regression gegen den direkten Vorgänger **Stage 21** geprüft und zusätzlich die Architekturtest-Suite erweitert.

## Nächster sinnvoller Block

Nach Stage 22 ist `lib4tables_prepare.py` deutlich dünner. Der nächste sinnvolle Architekturblock ist eine weitere Zerlegung der verbliebenen Text-/Wrapping- und Breitenlogik oder alternativ die Verdünnung von `Tables` zu einer fast reinen globalen Tabellen-Sektionsfassade.
