# Architecture Refactor Stage 33

Stage 33 ist die Impact- und Migration-Gate-Schicht der neuen Reta-Architektur.

Bis Stage 32 konnte man eine Komponente durch die Architektur verfolgen:

```text
alte reta-Komponente
  → neue Kapsel
  → Kategorie
  → Funktor / natürliche Transformation
  → kommutierendes Diagramm
  → Refactor-Gesetz
  → Witness
  → Validation / Coherence
  → Python-Boundary
```

Stage 33 beantwortet jetzt die nächste Frage:

```text
Was wird betroffen, wenn diese Komponente später bewegt oder verändert wird?
```

## Gesamtposition

```text
RetaArchitectureRoot
└─ CategoricalMetaCapsule
   ├─ CategoryTheoryBundle
   ├─ ArchitectureMapBundle
   ├─ ArchitectureContractsBundle
   ├─ ArchitectureWitnessBundle
   ├─ ArchitectureValidationBundle
   ├─ ArchitectureCoherenceBundle
   ├─ ArchitectureTraceBundle
   ├─ ArchitectureBoundariesBundle
   └─ ArchitectureImpactBundle
```

## Stufenlogik

```text
Stage 27  Kategorien, Funktoren, natürliche Transformationen
Stage 28  Kapselkarte und Gesamtarchitektur
Stage 29  kommutierende Diagramme und Refactor-Gesetze
Stage 30  Witnesses mit Datei-/Test-/Probe-Ankern
Stage 31  Validation und Coherence
Stage 32  Traces und Modulgrenzen
Stage 33  Impact-Routen und Migration-Gates
```

## Was von reta was wird

```text
reta.py / retaPrompt.py / libs/*
  → Legacy- oder Kompatibilitätsflächen
  → MigrationCandidateSpec
  → RegressionGateSpec

reta_architecture/*
  → neue Kapselbesitzer
  → ImpactSourceSpec
  → ImpactContractSpec

csv/* / i18n/*
  → lokale Sektionen, Topologie- und Semantikquellen
  → ImpactSourceSpec für Prägarben-/Garbenpfade
```

## Keine Verhaltensänderung

Stage 33 ist eine Architektur-Metaschicht. Sie verschiebt keine fachliche Logik und ändert keine Ausgabeabsicht. Spätere Stages können diese Schicht benutzen, um konkrete Code-Extraktionen kontrollierter durchzuführen.
