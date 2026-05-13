# Stage 31 – Ausführbare Validierung und Kohärenzmatrix

Stage 31 baut direkt auf Stage 30 auf.

Vorher war die Architektur bereits sichtbar als:

```text
Stage 27  Kategorien, Funktoren, natürliche Transformationen
Stage 28  Gesamtarchitekturkarte mit Kapseln, Containment und Flüssen
Stage 29  kommutierende Diagramme, Kapselverträge und Refactor-Gesetze
Stage 30  Witness-Matrix mit Datei-, Probe- und Test-Ankern
```

Stage 31 zieht diese Ebenen jetzt zusammen. Die neue Frage lautet:

```text
Sind Kategorie-Theorie, Architekturkarte, Verträge, Witnesses und Paketbaum
untereinander kohärent?
```

## Neue Schichten

### `reta_architecture/architecture_validation.py`

Diese Datei führt ausführbare Architekturchecks ein:

- Kategorien referenzieren bekannte Objekte und Morphismen.
- Funktoren referenzieren bekannte Quell- und Zielkategorien.
- Natürliche Transformationen referenzieren bekannte Funktoren.
- Die Architekturkarte steht auf Stage 31.
- Flüsse und Containment-Kanten referenzieren bekannte Kapseln oder registrierte Meta-Bundles.
- Verträge und Witnesses decken die erwarteten Diagramme, Gesetze und Transformationen ab.
- Das Paket enthält die Pflichtdateien und keine Runtime-Artefakte.
- Die Stage-31-Markdown-Historie ist vorhanden.

Neue Probe:

```bash
python -B -S reta_architecture_probe_py.py architecture-validation-json
python -B -S reta_architecture_probe_py.py architecture-validation-md
```

### `reta_architecture/architecture_coherence.py`

Diese Datei ergänzt eine Kohärenzmatrix:

```text
Kapsel
  → Kategorie
  → Funktor / natürliche Transformation
  → kommutierendes Diagramm
  → Refactor-Gesetz
  → Witness
```

Neue Probe:

```bash
python -B -S reta_architecture_probe_py.py architecture-coherence-json
python -B -S reta_architecture_probe_py.py architecture-coherence-md
```

## Paradigma nach Stage 31

| Begriff | Stage-31-Lesart |
|---|---|
| Topologie | Kapseln und Kontexte müssen als stabile offene Strukturen durch die Karte laufen. |
| Morphismus | Jeder Architekturfluss wird als konkrete Übergangskante geprüft. |
| Universelle Eigenschaft | Workflow-/Gluing-Knoten bleiben als Refactor-Gesetze geschützt. |
| Prägarbe | Lokale CSV-/Prompt-/Dokument-Sektionen bleiben über Witnesses rückverfolgbar. |
| Garbe | Geklebte Semantik muss mit Verträgen und Witnesses zusammenpassen. |
| Kategorie | Kategorien werden nicht nur benannt, sondern auf Referenzkonsistenz geprüft. |
| Funktor | Architekturflüsse werden als Funktoren oder natürliche Transformationen klassifiziert. |
| natürliche Transformation | Natürlichkeitsquadrate werden über Diagramme, Gesetze und Witnesses kohärent gehalten. |

## Architekturgewinn

Vor Stage 31:

```text
Die Architektur kannte ihre mathematischen Objekte, Kapseln, Verträge und Witnesses.
```

Nach Stage 31:

```text
Die Architektur kann sich selbst auf Kohärenz prüfen.
```

Das ist wichtig für spätere Extraktionsstufen: Ein neuer Umbau darf nicht nur Tests bestehen, sondern muss auch zeigen, welche Kapsel, welcher Funktor, welche natürliche Transformation, welches Diagramm und welcher Witness betroffen sind.

## Verhalten

Stage 31 ist metadata- und prüfungsorientiert. Es ist keine Änderung am CLI-, Prompt-, Tabellen- oder Ausgabe-Verhalten beabsichtigt.
