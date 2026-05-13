# Stage 24 – Ausgabe-Syntax aus `lib4tables.py` herausgezogen

Stage 24 baut direkt auf Stage 23 auf. Es wurde kein Neuaufbau aus dem Original gemacht.

## Neuer Architekturbaustein: `reta_architecture/output_syntax.py`

Die konkreten Renderer-Syntaxklassen leben jetzt nicht mehr in `libs/lib4tables.py`, sondern in einer eigenen Architekturschicht:

- `NichtsSyntax`
- `OutputSyntax`
- `csvSyntax`
- `emacsSyntax`
- `markdownSyntax`
- `bbCodeSyntax`
- `htmlSyntax`
- `OutputSyntaxBundle`
- `bootstrap_output_syntax(...)`

Damit ist die Ausgabe-Syntax jetzt als explizite Renderer-Morphismenschicht sichtbar. Die alte Datei `libs/lib4tables.py` ist kein semantischer Besitzer dieser Klassen mehr.

## `libs/lib4tables.py` ist jetzt eine dünne Fassade

`libs/lib4tables.py` fiel von ca. **550 Zeilen** auf ca. **59 Zeilen**.

Es exportiert nur noch kompatibel weiter:

- Ausgabe-Syntaxklassen aus `reta_architecture.output_syntax`
- Zahlenmorphismen aus `reta_architecture.number_theory`
- `math` für alte Legacy-Importpfade

Damit bleiben alte Imports wie `from lib4tables import htmlSyntax, primCreativity` gültig, aber die Semantik gehört nicht mehr dem Legacy-Modul.

## Architektur-Fassade erweitert

`RetaArchitecture` besitzt jetzt zusätzlich:

- `output_syntax`

Neue Bootstrap-Methode:

- `RetaArchitecture.bootstrap_output_syntax(...)`

`snapshot()` enthält jetzt:

- `output_syntax`

## Output-Semantik hängt nicht mehr an `lib4tables.py`

`reta_architecture/output_semantics.py` importiert die Syntaxklassen jetzt direkt aus `reta_architecture.output_syntax`, nicht mehr aus dem Legacy-Modul.

Zusätzlich wurden Architekturmodule entkoppelt:

- `reta_architecture/table_output.py` nutzt jetzt `reta_architecture.output_syntax`
- `reta_architecture/combi_join.py` nutzt jetzt `reta_architecture.output_syntax`

Der alte Architekturpfad `architecture -> lib4tables -> architecture` wurde damit vermieden.

## Probe-Werkzeug erweitert

`reta_architecture_probe_py.py` kennt jetzt zusätzlich:

```bash
python -B -S reta_architecture_probe_py.py output-syntax-json
```

## Paket-Manifest erweitert

`reta_architecture/package_integrity.py` behandelt jetzt auch als Pflichtdatei:

- `reta_architecture/output_syntax.py`

## Tests erweitert

Neu bzw. erweitert:

- Regressionstest für `OutputSyntaxBundle`
- Test, dass `lib4tables.py` nur noch Fassade für Output-Syntax und Number-Theory ist
- Paket-Manifest prüft `reta_architecture/output_syntax.py`
- volle Unittest-Discovery läuft weiterhin stabil

## Geprüft

- `py_compile` über alle Python-Dateien: OK
- `python -B -S -m unittest tests.test_architecture_refactor -v`: **45 Tests, OK**
- `python -B -S -m unittest tests.test_command_parity -v`: **1 Paritätsmatrix-Test, OK**
- `python -B -S -m unittest -q`: **46 Tests, OK**
- `reta_architecture_probe_py.py output-syntax-json`: OK
- `reta_architecture_probe_py.py output-json`: OK
- `reta_architecture_probe_py.py package-integrity-json`: OK

Die Command-Parity-Matrix vergleicht weiterhin repräsentativ gegen das Originalarchiv:

- Shell-Ausgabe `Religionen/sternpolygon`
- Markdown-Ausgabe `Religionen/sternpolygon`
- HTML-Ausgabe `Religionen/sternpolygon` nach Normalisierung
- Bruch-/CSV-Gluing `--gebrochenuniversum=5`

## Paketstatus

- fehlende Pflichtdateien: **0**
- Runtime-Artefakte im finalen Baum: **0**
- `csv/religion.csv`: **1043 Zeilen**
- `csv/vn-religion.csv`: **1043 Zeilen**

## Nächster sinnvoller Block

Der nächste Block ist nicht mehr `lib4tables.py`, sondern die verbliebene breite Kompatibilitäts- und Legacy-Verkabelung in:

- `libs/tableHandling.py`
- `libs/lib4tables_prepare.py`
- `libs/lib4tables_concat.py`

Besonders sinnvoll wäre als nächstes eine saubere Tabellen-Sektionsschicht, die `Tables` als globale Tabelle/Garbe modelliert und die restlichen Legacy-Fassaden weiter verdünnt.
