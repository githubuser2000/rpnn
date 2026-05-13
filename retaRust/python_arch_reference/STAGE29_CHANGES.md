# Stage 29 – Architekturverträge: kommutierende Diagramme, Kapselgrenzen und Refactor-Gesetze

Stage 29 baut direkt auf Stage 28 auf. Stage 27 hatte Kategorien, Funktoren und natürliche Transformationen explizit gemacht. Stage 28 hatte die Gesamtarchitektur als Kapsel- und Datenflusskarte dargestellt. Stage 29 macht daraus jetzt **prüfbare Architekturverträge**.

Die neue Schicht beantwortet nicht nur:

```text
Was ist Topologie, Garbe, Morphismus, Funktor, natürliche Transformation?
```

sondern:

```text
Welche Pfade müssen kommutieren?
Welche Kapsel besitzt welche Semantik?
Welche Gesetze schützen spätere Refactor-Stages?
```

## Neue Schicht

### `reta_architecture/architecture_contracts.py`

Neue Architektur-Metadaten:

- `DiagramArrowSpec`
- `CommutativeDiagramSpec`
- `CapsuleContractSpec`
- `RefactorLawSpec`
- `ContractValidationSpec`
- `Stage29ArchitecturePlan`
- `ArchitectureContractsBundle`
- `bootstrap_architecture_contracts(...)`

Diese Schicht ändert kein CLI-, Prompt-, Tabellen- oder Ausgabe-Verhalten. Sie macht die vorhandene neue Architektur als Verträge inspizierbar und validiert ihre Referenzen gegen `CategoryTheoryBundle` und `ArchitectureMapBundle`.

## Registrierte kommutierende Diagramme

Stage 29 registriert acht Diagramme:

| Diagramm | Bedeutung |
|---|---|
| `RawCommandNaturalitySquare` | Raw-CLI/Prompt-Text, Kontextrestriktion und Kanonisierung müssen verträglich sein. |
| `PresheafSheafGluingSquare` | Lokale Sektionen einschränken und dann kleben entspricht erst kleben und dann einschränken. |
| `UniversalWorkflowTableSquare` | Kanonische Semantik, Workflow-Gluing und Tabellenbau führen zur gleichen globalen Tabellensektion. |
| `GeneratedColumnStateSyncSquare` | Generated-column-Endofunktoren, TableStateSections und GeneratedColumnsSheaf bleiben synchron. |
| `RuntimeStateProjectionSquare` | Mutable `Tables`-Legacy-Attribute und explizite `TableStateSections` dürfen nicht divergieren. |
| `RenderedOutputParitySquare` | Architektur-Renderer und Legacy-Renderer müssen nach Normalisierung dieselbe beobachtbare Ausgabe liefern. |
| `LegacyArchitectureCompatibilitySquare` | Legacy-Oberfläche und Architektur-Oberfläche bleiben natürlich transformiert und paritätsfähig. |
| `ArchitectureMapContractReflectionTriangle` | Kategorie-Theorie und Kapselkarte spiegeln sich in validierten Stage-29-Verträgen. |

## Kapselverträge

Stage 29 ergänzt elf Kapselverträge, je einen für:

```text
RetaArchitectureRoot
SchemaTopologyCapsule
LocalSectionCapsule
SemanticSheafCapsule
InputPromptCapsule
WorkflowGluingCapsule
TableCoreCapsule
GeneratedRelationCapsule
OutputRenderingCapsule
CompatibilityCapsule
CategoricalMetaCapsule
```

Jeder Vertrag hält fest:

```text
owns
accepts
produces
must_not_own
primary_category
primary_functor_or_transformation
protected_by
implementation_anchors
stage_span
```

Damit wird praktisch klar, was in was von reta steckt und wo neue Arbeit einsortiert werden soll.

## Refactor-Gesetze

Stage 29 hält neun Gesetze fest:

| Gesetz | Rolle |
|---|---|
| `ContextRefinementCompositionLaw` | Topologische Verfeinerungen müssen komponierbar bleiben. |
| `PresheafRestrictionLaw` | Prägarben-Restriktionen müssen mit Kontextverfeinerung verträglich sein. |
| `SheafGluingUniquenessLaw` | Kompatible lokale Sektionen sollen eine kanonische globale Semantik ergeben. |
| `RawCanonicalNaturalityLaw` | Raw-Kommandos und kanonische Semantik bleiben natürlich transformiert. |
| `WorkflowUniversalConstructionLaw` | Tabellenbau läuft über die universelle Workflow-Konstruktion. |
| `GeneratedColumnStateSyncLaw` | Generated-Endofunktoren synchronisieren expliziten Zustand und Sheaf-Metadaten. |
| `RuntimeStateProjectionLaw` | Mutable Runtime und explizite StateSections zeigen denselben Zustand. |
| `OutputNormalizationNaturalityLaw` | Renderer-Normalisierung ist der Paritäts-Naturality-Pfad. |
| `LegacyCompatibilityNaturalityLaw` | Alte Fassaden kommutieren mit der neuen Architektur. |

## Architektur-Integration

`RetaArchitecture` besitzt jetzt zusätzlich:

```python
architecture_contracts: ArchitectureContractsBundle
```

Neuer Bootstrap:

```python
RetaArchitecture.bootstrap_architecture_contracts(...)
```

`snapshot()` enthält jetzt:

```python
"architecture_contracts": ...
```

Die Stage-28-Karte wurde fortgeschrieben:

- `CategoricalMetaCapsule` enthält jetzt `ArchitectureContractsBundle`.
- `ArchitectureMapBundle.snapshot()["stage"]` steht jetzt auf `29`.
- Die Stage-Liste enthält jetzt `Stage 29`.
- Ein neuer Fluss beschreibt `CategoricalMetaCapsule → CompatibilityCapsule` als Architekturgesetz-/Paritätspfad.

## Probe-Werkzeug

Neue Probes:

```bash
python -B -S reta_architecture_probe_py.py architecture-contracts-json
python -B -S reta_architecture_probe_py.py architecture-contracts-md
```

`architecture-contracts-json` enthält:

- kommutierende Diagramme
- Kapselverträge
- Refactor-Gesetze
- Referenzvalidierung gegen Kategorie-Theorie und Kapselkarte
- Stage-29-Plan

`architecture-contracts-md` gibt die Vertragsstruktur als Markdown/Textdiagramm und Mermaid aus.

## Architekturgewinn

Vor Stage 29:

```text
Die Architekturkarte zeigte, wo Dinge liegen und wie Daten fließen.
```

Nach Stage 29:

```text
Die Architektur sagt zusätzlich, welche Pfade kommutieren müssen, welche Kapselgrenzen gelten und welche Gesetze weitere Extraktionen schützen.
```

Das ist der praktische Sinn der natürlichen Transformationen im Projekt: Sie sind jetzt nicht nur mathematische Namen, sondern explizite Refactor-Verträge.
