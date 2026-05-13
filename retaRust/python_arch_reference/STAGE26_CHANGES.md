# Stage 26 – Explizite Tabellenzustands-Sektionen

Dieser Schritt baut direkt auf **Stage 25** auf. Es wurde nicht neu angefangen; der vorhandene Architekturpfad bleibt erhalten.

## Ziel

Nach Stage 25 lag `Tables` bereits in `reta_architecture.table_runtime`. Der nächste Engpass war, dass der mutable Tabellenzustand selbst weiterhin direkt in `Tables.__init__` erzeugt wurde. Stage 26 zieht diesen Zustand in eine eigene Architekturschicht.

## Neue Schicht

### `reta_architecture/table_state.py`

Neue explizite State-/Sektionen-Schicht für die globale Tabellensektion:

- `GeneratedColumnSection`
  - besitzt `parameters`
  - besitzt `tags`
  - entspricht dem alten Paar `generatedSpaltenParameter` / `generatedSpaltenParameter_Tags`
- `TableDisplayState`
  - besitzt Display-Flags:
    - `keine_ueberschriften`
    - `keine_leeren_inhalte`
    - `spalte_gestirn`
  - besitzt die gemeinsame `religion_numbers`-Liste
- `TableStateSections`
  - bündelt:
    - `highest_rows`
    - `display`
    - `generated_columns`
    - `row_display_to_original`
    - `generated_rows`
- `TableStateBundle`
  - Factory und Snapshot-Schicht für die Tabellenzustände
- `bootstrap_table_state(...)`
  - neuer Architektur-Bootstrap

## Änderungen an `reta_architecture/table_runtime.py`

- `Tables` erzeugt seinen Zustand jetzt über `TableStateBundle`.
- Legacy-Attribute bleiben kompatibel:
  - `generatedSpaltenParameter`
  - `generatedSpaltenParameter_Tags`
  - `rowNumDisplay2rowNumOrig`
  - `religionNumbers`
- Diese Attribute zeigen jetzt auf die expliziten Sektionen aus `table_state`.
- `keineUeberschriften`, `keineleereninhalte` und `spaltegGestirn` sind jetzt Properties über `TableDisplayState`.
- `Tables.tableStateSnapshot` macht den aktuellen State inspizierbar.
- `TableRuntimeBundle` enthält jetzt ein `table_state`-Bundle und erzeugt `Tables` über diese Schicht.

## Änderungen an `reta.py`

- `reta.py` importiert `Tables` nicht mehr direkt.
- `Program.__init__` erzeugt Tabellen jetzt über:

```python
self.architecture.bootstrap_table_runtime().create_tables(...)
```

Damit hängt auch der konkrete Programmlauf an der expliziten Architektur-Fassade, nicht mehr direkt an einer Runtime-Klasse.

## Änderungen an `RetaArchitecture`

- neue Eigenschaft:
  - `table_state`
- neuer Bootstrap:
  - `bootstrap_table_state(...)`
- `bootstrap_table_runtime(...)` reicht die State-Schicht weiter.
- `snapshot()` enthält jetzt `table_state`.

## Neues Probe-Kommando

```bash
python -B -S reta_architecture_probe_py.py table-state-json
```

## Tests

Neue Regressionen prüfen:

- `TableStateBundle` ist explizit vorhanden.
- `RetaArchitecture.snapshot()` enthält `table_state`.
- `Tables` nutzt die State-Sektionen tatsächlich.
- `generatedSpaltenParameter` und `generatedSpaltenParameter_Tags` zeigen auf die neue `GeneratedColumnSection`.
- Display-Flags werden über `TableDisplayState` gespiegelt.
- `reta.py` erzeugt Tabellen über `bootstrap_table_runtime().create_tables(...)`.

## Geprüft

- `py_compile`: OK
- `tests.test_architecture_refactor`: **46 Tests, OK**
- `reta_architecture_probe_py.py table-state-json`: OK
- `reta_architecture_probe_py.py table-runtime-json`: OK
- `reta_architecture_probe_py.py package-integrity-json`: OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- Smoke-Test `Religionen/sternpolygon`: OK

Zusätzlich gegen **Stage 25** verglichen:

- Shell-Ausgabe `Religionen/sternpolygon`: byte-identisch
- Markdown-Ausgabe `Religionen/sternpolygon`: byte-identisch
- Bruch-/CSV-Gluing `--gebrochenuniversum=5`: byte-identisch

## Wichtiger Hinweis

Die große Original-vs-Refactor-Paritätsmatrix wurde in diesem Schritt nicht als finale Aussage verwendet. Sie lief in dieser Umgebung wieder schwerfällig bei einem Bruch-/CSV-Fall. Für die direkte Nicht-Regression dieses Schritts wurde deshalb Stage 25 als Baseline genutzt.

## Architekturgewinn

Stage 26 ist kein großer Monolith-Split wie frühere Schritte, aber ein wichtiger Ownership-Schritt:

Vorher:

```text
Tables = Runtime-Klasse + mutable Zustandscontainer
```

Jetzt:

```text
TableStateBundle / TableStateSections = explizite mutable Sektionen
Tables = legacy-kompatible globale Tabellensektion über diesen Sektionen
```

Damit wird die nächste Aufteilung von `Tables` in Tabellenzustand, Outputzustand, Generated-Column-State und Kombi-State einfacher und weniger riskant.
