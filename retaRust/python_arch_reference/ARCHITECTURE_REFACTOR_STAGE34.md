# Architecture Refactor Stage 34 – Guarded Migration Plan

Stage 34 ist der nächste Schritt nach der Stage-33-Impact-Schicht. Die Architektur weiß jetzt nicht nur, welche reta-Komponenten betroffen wären, sondern auch, in welcher Reihenfolge sie später bewegt werden dürften und welche Gates vorher bestehen müssen.

## Ausgangslage vor Stage 34

Bis Stage 33 waren sichtbar:

- Topologie und Kontextöffnungen
- Prägarben und lokale Sektionen
- Garben und geklebte Semantik
- Morphismen und universelle Konstruktionen
- Kategorien, Funktoren und natürliche Transformationen
- Architekturkarte, Verträge, Witnesses, Validation, Coherence
- Traces, Boundaries und Impact-Gates

Stage 33 endete bei guarded Migration Candidates. Diese Kandidaten waren noch keine geordnete Umbauplanung.

## Stage-34-Ziel

Stage 34 führt `ArchitectureMigrationBundle` ein. Es liest die Stage-33-Kandidaten als Objekte der `ArchitectureImpactCategory` und projiziert sie über Funktoren in die neue `ArchitectureMigrationCategory`.

```text
ArchitectureImpactCategory
  --ImpactToMigrationPlanFunctor-->
ArchitectureMigrationCategory
```

Die Migrationsplanung ist bewusst konservativ. Sie führt keine Migration aus. Sie sagt nur:

```text
Diese Kapsel kann später in dieser Welle angefasst werden,
aber nur wenn diese Gates, Diagramme, Gesetze und natürlichen Transformationen gültig bleiben.
```

## Wellenmodell

Die Stage-34-Wellen sind nach Kapseln und Risiko geordnet:

```text
M0 Meta-/Probe-/Dokumentationsschicht
M1 Schema-/Topologie-/Prägarbenflächen
M2 Input-/Prompt-Flächen
M3 Workflow-/Universal-Gluing-Flächen
M4 TableCore-Flächen
M5 GeneratedRelation-Flächen
M6 Output-/Compatibility-Flächen
```

Die genaue Anzahl der Schritte ergibt sich aus den Stage-33-Impact-Kandidaten.

## Natürliche Transformationen

`ImpactGateMigrationTransformation` stellt sicher, dass der Kandidatenpfad und der Gate-Binding-Pfad denselben erlaubten späteren Move beschreiben.

`MigrationPlanCoherenceTransformation` stellt sicher, dass Wellenordnung und Gate-Kohärenz dieselbe Invariante schützen.

## Keine Verhaltensänderung

Stage 34 ist metadata-only. Das beobachtbare Verhalten von `reta.py`, `retaPrompt.py`, Tabellenaufbau, Generated Columns und Ausgabeformaten soll unverändert bleiben.
