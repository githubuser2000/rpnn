# Stage 23 – Zahlen-/Wrapping-Morphismen weiter aus dem Legacy-Kern gezogen

Stage 23 baut direkt auf Stage 22 auf. Es wurde kein Neuaufbau aus dem Original gemacht.

## Neuer Architekturbaustein: `reta_architecture/table_wrapping.py`

Die historisch in `libs/lib4tables_prepare.py` liegende Textumbruch- und Breitenlogik ist jetzt eine eigene Architekturschicht.

Enthalten sind unter anderem:

- `Wraptype`
- `TextWrapRuntime`
- `TableWrappingBundle`
- `refresh_textwrap_runtime(...)`
- `textwrap_runtime(...)`
- `set_shell_rows_amount(...)`
- `get_shell_rows_amount(...)`
- `set_wrapping_type(...)`
- `get_wrapping_type(...)`
- `chunks(...)`
- `split_more_if_not_small(...)`
- `alxwrap(...)`
- `wrap_cell_text(...)`
- `width_for_row(...)`
- `bootstrap_table_wrapping(...)`

`libs/lib4tables_prepare.py` bleibt als Legacy-kompatible Fassade erhalten, delegiert aber die alte Wrapping-/Width-Logik an diese neue Schicht.

## Neuer Architekturbaustein: `reta_architecture/number_theory.py`

Die kleine, aber tief verwendete Zahlenlogik ist jetzt als eigene Morphismen-Schicht sichtbar.

Enthalten sind:

- `moonNumber(...)`
- `primFak(...)`
- `divisorGenerator(...)`
- `primRepeat(...)`
- `primCreativity(...)`
- `primMultiple(...)`
- `isPrimMultiple(...)`
- `couldBePrimeNumberPrimzahlkreuz(...)`
- `couldBePrimeNumberPrimzahlkreuz_fuer_innen(...)`
- `couldBePrimeNumberPrimzahlkreuz_fuer_aussen(...)`
- `NumberTheoryBundle`
- `bootstrap_number_theory(...)`

Diese Schicht ist bewusst dependency-light: keine CLI-, Renderer-, i18n- oder Tabellenimporte. Damit ist sie ein sauberer Morphismenkern statt weiter Teil von `lib4tables.py`.

## Umgehängte Architekturmodule

Diese Architekturmodule verwenden die neue Zahlenmorphismen-Schicht jetzt direkt:

- `reta_architecture/row_filtering.py`
- `reta_architecture/generated_columns.py`
- `reta_architecture/meta_columns.py`
- `reta_architecture/table_output.py`

Damit hängt die Architektur für Prim-/Mond-/Vielfachheitslogik nicht mehr am Legacy-Modul `lib4tables.py`. `lib4tables.py` bleibt für ältere Importpfade und Ausgabe-Syntaxklassen erhalten.

## Architektur-Fassade erweitert

`RetaArchitecture` trägt jetzt zusätzlich:

- `table_wrapping`
- `number_theory`

Neue Bootstrap-Methoden:

- `RetaArchitecture.bootstrap_table_wrapping(...)`
- `RetaArchitecture.bootstrap_number_theory(...)`

`snapshot()` enthält jetzt auch:

- `table_wrapping`
- `number_theory`

## Probe-Werkzeug erweitert

`reta_architecture_probe_py.py` kennt jetzt zusätzlich:

```bash
python -B -S reta_architecture_probe_py.py table-wrapping-json
python -B -S reta_architecture_probe_py.py number-theory-json
```

## Tests erweitert

Neu bzw. erweitert:

- Regressionstest für `TableWrappingBundle`
- Regressionstest für `NumberTheoryBundle`
- Prüfung, dass `row_filtering.py` die Zahlenfunktionen aus `reta_architecture.number_theory` bezieht
- Paket-Manifest prüft jetzt auch:
  - `reta_architecture/table_wrapping.py`
  - `reta_architecture/number_theory.py`

## Geprüft

- `py_compile` über alle Python-Dateien: OK
- `tests.test_architecture_refactor`: **43 Tests, OK**
- `reta_architecture_probe_py.py table-wrapping-json`: OK
- `reta_architecture_probe_py.py number-theory-json`: OK
- `reta_architecture_probe_py.py package-integrity-json`: OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- Smoke-Test `Religionen/sternpolygon`: OK

Zusätzlich gegen Stage 22 verglichen:

- Shell-Ausgabe `Religionen/sternpolygon`: byte-identisch
- Markdown-Ausgabe `Religionen/sternpolygon`: byte-identisch
- Bruch-/CSV-Gluing `--gebrochenuniversum=5`: byte-identisch

## Paketstatus

- fehlende Pflichtdateien: **0**
- Runtime-Artefakte: **0**
- `csv/religion.csv`: **1043 Zeilen**
- `csv/vn-religion.csv`: **1043 Zeilen**

## Bewertung

Der nächste sinnvolle Block ist jetzt `libs/lib4tables.py`: Dort liegen noch Ausgabe-Syntaxklassen und die alten Zahlenfunktionen als Legacy-Export. Die neuere Architektur sollte diese künftig entweder über `output_semantics`/`table_output` oder über `number_theory` vollständig kapseln.
