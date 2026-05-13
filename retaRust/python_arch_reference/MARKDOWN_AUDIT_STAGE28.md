# Markdown Audit – Stage 28

## Gelesene Historienbasis

Aus dem Stage-27-Paket waren vor Stage 28 insgesamt **58 Markdown-Dateien** vorhanden.

Wichtige Familien:

- `ARCHITECTURE_REFACTOR.md`
- `ARCHITECTURE_REFACTOR_STAGE18.md`
- `ARCHITECTURE_REFACTOR_STAGE22.md`
- `ARCHITECTURE_REFACTOR_STAGE23.md`
- `ARCHITECTURE_REFACTOR_STAGE26.md`
- `ARCHITECTURE_REFACTOR_STAGE27.md`
- `CONTINUATION_CHANGES.md`
- `STAGE3_CHANGES.md` bis `STAGE27_CHANGES.md`
- `PACKAGE_AUDIT_STAGE11.md` bis `PACKAGE_AUDIT_STAGE27.md`
- `readme-reta*.md`
- `readme-retaPrompt*.md`
- `readme-startFiles.md`
- entsprechende Dateien unter `doc/`

## Hochgeladenes `retaPyNewArch.tar.bz2`

Das neu hochgeladene tar.bz2 enthielt in dieser Umgebung **0 Markdown-Dateien**. Es enthielt im Wesentlichen CSVs und wenige Hilfsskripte.

Deshalb wurde Stage 28 aus dem Stage-27-Paket aufgebaut und die Abweichung im neuen `MarkdownAuditSpec` dokumentiert.

## Konsequenz für Stage 28

Stage 28 baut nicht auf einer unbekannten externen Absprache auf, sondern auf der im Paket vorhandenen Markdown-Historie und der vorhandenen Stage-27-Architektur:

```text
Topologie
Morphismen
universelle Eigenschaften
Prägarben
Garben
math Kategorien
Funktoren
natürliche Transformationen
```

Neu hinzu kommt die explizite Gesamtkarte:

```text
Kapseln
Enthält-Beziehungen
Datenflüsse
Legacy-zu-Neu-Mappings
Stage-Schritte
Textdiagramm
Mermaid-Diagramm
```
