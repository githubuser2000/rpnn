# Architecture Impact Stage 33

Stage 33 führt `ArchitectureImpactBundle` ein.

## Zweck

Die Architektur kennt bereits Kapseln, Kategorien, Funktoren, natürliche Transformationen, Verträge, Witnesses, Validation, Coherence, Traces und Boundaries. Stage 33 verbindet diese Ebenen zu einer Impact-Rechnung.

```text
Trace + Boundary
  → ImpactSourceSpec
  → ImpactContractSpec
  → RegressionGateSpec
  → MigrationCandidateSpec
  → ImpactValidationSpec
```

## Datentypen

### ImpactSourceSpec

Eine alte oder neue reta-Komponente, deren Änderungen durch die Architektur verfolgt werden können.

Enthält unter anderem:

```text
source
source_kind
capsules
categories
functors
natural_transformations
diagrams
laws
boundary_edges
route_hops
```

### ImpactContractSpec

Die betroffenen Architekturverträge für eine Impact-Quelle.

```text
affected_capsules
affected_diagrams
affected_laws
affected_natural_transformations
required_gates
required_probes
```

### RegressionGateSpec

Ein konkretes Gate, das spätere Umbauten schützen muss.

Beispiele:

```text
CategoryTheoryProbeGate
ArchitectureMapProbeGate
ArchitectureContractsProbeGate
ArchitectureWitnessProbeGate
ArchitectureCoherenceProbeGate
ArchitectureTraceProbeGate
ArchitectureBoundaryProbeGate
ArchitectureImpactSelfGate
ArchitectureRegressionGate
CommandParityGate
```

### MigrationCandidateSpec

Ein bewachter Kandidat für spätere Extraktion oder Wartung. Das ist noch kein Move, sondern eine kontrollierte Umbauabsicht.

## Natürliche Transformationen

```text
TraceBoundaryImpactTransformation
```

verbindet die Trace-Lesart und die Boundary-Lesart derselben Komponente.

```text
ImpactGateValidationTransformation
```

verbindet Migrationskandidaten mit prüfbaren Gates.

## Kommutierende Diagramme

```text
TraceBoundaryImpactSquare
ImpactGateValidationSquare
```

Diese Diagramme machen die Stage-33-Idee prüfbar: Wer später Code bewegt, soll nicht an den Kapsel- und Natürlichkeitsverträgen vorbei arbeiten.
