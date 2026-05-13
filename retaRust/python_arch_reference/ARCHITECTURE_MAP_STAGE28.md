# Reta Stage 28 Architecture Map

Diese Datei ist die menschenlesbare Fassung von `reta_architecture/architecture_map.py`.

## Gesamtbild

```text
Legacy-Oberflächen
    reta.py / retaPrompt.py / libs/*
        │
        ▼
RetaArchitectureRoot
        │
        ├─ SchemaTopologyCapsule
        │      i18n-Kontext, Matrix, Runtime → Schema → offene Kontexte
        │
        ├─ LocalSectionCapsule
        │      CSVs, Docs, Übersetzungen, Prompt-State als Prägarben-Sektionen
        │
        ├─ SemanticSheafCapsule
        │      lokale Sektionen → kanonische Parametersemantik / Garben
        │
        ├─ InputPromptCapsule
        │      CLI/Prompt-Rohtext → Tokens → kanonische Parameter
        │
        ├─ WorkflowGluingCapsule
        │      Parameter-Runtime + Spaltenauswahl + Tabellen-Gluing
        │
        ├─ TableCoreCapsule
        │      globale Tabellensektion + explizite TableStateSections
        │
        ├─ GeneratedRelationCapsule
        │      Generated Columns, Meta Columns, CSV-Bruch-Gluing, Kombi Join
        │
        ├─ OutputRenderingCapsule
        │      Output-Syntax, Output-Semantik, TableOutput
        │
        ├─ CompatibilityCapsule
        │      Legacy-Fassaden, Paket-Audit, Paritätstests
        │
        └─ CategoricalMetaCapsule
               Kategorien, Funktoren, natürliche Transformationen, Kapselkarte
```

## Kategorialer Lesepfad

```text
Open(Context)^op
    └─ LocalSection Presheaves
          └─ Sheafification / Gluing
                └─ Canonical Semantic Sheaves
                      └─ Universal Workflow Construction
                            └─ Global Table Section
                                  ├─ Generated-column Endofunctors
                                  ├─ Row/Prepare/Wrapping Morphisms
                                  └─ Output Rendering Functors
```

## Kommutierende Refactor-Pfade

Die wichtigsten natürlichen Transformationen aus Stage 27 erscheinen in Stage 28 als Flüsse:

| Natürliche Transformation | In der Karte zwischen |
|---|---|
| `RawToCanonicalParameterTransformation` | `InputPromptCapsule → SemanticSheafCapsule` |
| `PresheafToSheafGluingTransformation` | `LocalSectionCapsule → SemanticSheafCapsule` |
| `TableGenerationGluingTransformation` | `SemanticSheafCapsule → WorkflowGluingCapsule` |
| `GeneratedColumnsSheafSyncTransformation` | `GeneratedRelationCapsule ↔ TableCoreCapsule` |
| `TableRuntimeToStateSectionsTransformation` | `TableCoreCapsule → CategoricalMetaCapsule` |
| `RenderedOutputNormalizationTransformation` | `OutputRenderingCapsule → CompatibilityCapsule` |
| `LegacyToArchitectureTransformation` | `CompatibilityCapsule → RetaArchitectureRoot` |

## Praktische Regel

Neue Arbeit sollte nicht mehr fragen:

```text
In welche alte Datei schreibe ich das?
```

sondern:

```text
Welche Kapsel besitzt diese Semantik?
Ist es Topologie, lokale Sektion, Garbe, Morphismus, universelles Gluing,
Tabellenzustand, Renderer, Funktor oder natürliche Transformation?
```
