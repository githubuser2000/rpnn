# Stage 32 – Architektur-Traces und Modulgrenzen

Stage 32 baut direkt auf Stage 31 auf.

Bis Stage 31 war die neue Architektur bereits als überprüfbarer Stack vorhanden:

```text
Stage 27  Kategorien, Funktoren, natürliche Transformationen
Stage 28  Gesamtarchitekturkarte mit Kapseln, Containment und Flüssen
Stage 29  kommutierende Diagramme, Kapselverträge und Refactor-Gesetze
Stage 30  Witness-Matrix mit Datei-, Probe- und Test-Ankern
Stage 31  Validation und Coherence über alle Metaebenen
```

Stage 32 ergänzt zwei praktische Navigationsebenen:

```text
ArchitectureTraceBundle
ArchitectureBoundariesBundle
```

Die neue Frage lautet:

```text
Wie verfolgt man ein konkretes reta-Teil stufenweise durch Kapsel,
Kategorie, Funktor, natürliche Transformation, Vertrag, Witness und Check?

Und wo liegen die realen Modulgrenzen zwischen den Kapseln?
```

## Neue Dateien

```text
reta_architecture/architecture_traces.py
reta_architecture/architecture_boundaries.py
```

## Neue Probes

```bash
python -B -S reta_architecture_probe_py.py architecture-traces-json
python -B -S reta_architecture_probe_py.py architecture-traces-md
python -B -S reta_architecture_probe_py.py architecture-boundaries-json
python -B -S reta_architecture_probe_py.py architecture-boundaries-md
```

## Paradigma nach Stage 32

| Begriff | Stage-32-Lesart |
|---|---|
| Topologie | Kontext- und Kapselräume werden über Traces auffindbar. |
| Morphismus | Architekturflüsse und Importkanten werden als gerichtete Übergänge sichtbar. |
| Universelle Eigenschaft | Gluing-Knoten bleiben über Trace-Routen und Boundary-Checks verankert. |
| Prägarbe | lokale Datei-/Prompt-/CSV-Sektionen sind als Komponentenrouten sichtbar. |
| Garbe | geklebte Semantik bleibt an Semantik-Kapsel und Verträge rückgebunden. |
| Kategorie | Kategorien sind in Trace- und Boundary-Routen auffindbar. |
| Funktor | Funktoren werden als Routen zwischen Kapseln und Kategorien verfolgt. |
| natürliche Transformation | Natürlichkeitsverträge werden als explizite Trace-Ziele navigierbar. |

## Architekturgewinn

Vor Stage 32 konnte die Architektur sich validieren und Kohärenz herstellen.

Nach Stage 32 kann man sie praktisch navigieren:

```text
reta-Komponente → Kapsel → Kategorie/Funktor/Transformation → Diagramm → Gesetz → Witness → Check
```

Zusätzlich wird die reale Modulstruktur als Boundary-Graph sichtbar:

```text
Python-Modul → Besitzer-Kapsel → Importkante → Kapselgrenze → Boundary-Check
```

Stage 32 bleibt metadata- und prüfungsorientiert. CLI-, Prompt-, Tabellen- und Ausgabe-Verhalten sollen unverändert bleiben.
