# Natürliche Transformationen im architekturveränderten `reta`

Im aktuellen architekturveränderten `reta` gibt es **29 natürliche Transformationen** in:

```text
reta_architecture/category_theory.py
```

Eine natürliche Transformation ist hier eine **Verträglichkeitsbedingung zwischen zwei Funktoren**.

Einfach gesagt:

```text
Kategorie = ein Architekturraum
Morphismus = Übergang innerhalb eines Architekturraums
Funktor = Übersetzung eines ganzen Architekturraums in einen anderen
natürliche Transformation = Garantie, dass zwei Übersetzungswege dasselbe Ergebnis liefern
```

Oder noch praktischer:

```text
Wenn ich links herum durch die Architektur gehe
und rechts herum durch die Architektur gehe,
muss am Ende dasselbe beobachtbare Ergebnis herauskommen.
```

Im Projekt sind diese natürlichen Transformationen keine abstrakte Deko. Sie sind die Architekturgesetze, die erzwingen:

```text
Rohtext und kanonische Semantik passen zusammen.
Prägarben und Garben kleben korrekt.
Tabellenbau folgt aus Semantik.
Output bleibt vergleichbar.
Legacy-Pfade und neue Architekturpfade bleiben äquivalent.
Migrationen laufen nur mit Gates, Rollback und Validierung.
```

---

# 1. Gesamtpriorität

Die 29 natürlichen Transformationen lassen sich in fünf Gruppen einteilen:

| Priorität | Gruppe | Transformationen | Bedeutung |
|---:|---|---|---|
| 1 | Semantik, Prägarbe, Garbe, Tabelle | 1–7 | Kernarchitektur von `reta` |
| 2 | Verträge, Validierung, Kohärenz | 8–13 | schützt die mathematische Architektur |
| 3 | Migration, Rehearsal, Activation | 14–19 | schützt weitere Umbauten |
| 4 | aktivierte Runtime-Kapseln | 20–29 | RowRange, Arithmetik, Console, Completion |
| 5 | reine Kompatibilität | Teile von 7, 20, 22, 24, 26, 28 | alte Pfade bleiben lauffähig |

Die wichtigsten Transformationen überhaupt sind:

```text
RawToCanonicalParameterTransformation
PresheafToSheafGluingTransformation
TableGenerationGluingTransformation
GeneratedColumnsSheafSyncTransformation
TableRuntimeToStateSectionsTransformation
RenderedOutputNormalizationTransformation
LegacyToArchitectureTransformation
```

Warum?  
Weil sie direkt die Hauptpipeline absichern:

```text
Rohtext / Datei / CSV / Prompt
        ↓
lokale Prägarbe
        ↓
kanonische Garbe
        ↓
globale Tabelle
        ↓
Ausgabe
        ↓
Normalform / Parität
```

---

# 2. Höchste Priorität: natürliche Transformationen der Prägarben und Garben

## 2.1 `RawToCanonicalParameterTransformation`

```text
RawCommandPresheafFunctor
    =>
CanonicalParameterSheafFunctor
```

## Bereich

```text
Rohkommando-Prägarbe -> kanonische Parametergarbe
```

## Komponenten

```text
PromptText     -> PromptMorphisms.split_command_words
RawAlias       -> AliasMorphisms.canonical_main / canonical_sub
ParameterPair  -> ParameterSemanticsSheaf.canonicalize_pair
```

## Natürlichkeitsbedingung

```text
Kontext zuerst einschränken und dann kanonisieren
=
zuerst kanonisieren und danach auf den kleineren Kontext einschränken
```

## Bedeutung

Diese Transformation ist eine der wichtigsten überhaupt.

Sie sagt:

```text
Ob ein Nutzer "a1", "absicht 1", "religion a1" oder einen längeren Prompt-Befehl eingibt:
Wenn die Bedeutung dieselbe ist, muss die kanonische Parametersemantik dieselbe sein.
```

Beispiel:

```text
Prompt-Rohtext
        ↓ split_command_words
Rohparameter
        ↓ canonicalize_pair
kanonisches Parameterpaar
```

muss dasselbe ergeben wie:

```text
Prompt-Rohtext
        ↓ Kontext einschränken
lokale Prompt-Sektion
        ↓ kanonisieren
kanonisches Parameterpaar
```

## Zugehörige Topologien

```text
Prompt-Topologie
Scope-Topologie
Hauptparameter-Topologie
Nebenparameter-Topologie
Kontext-Topologie
```

## Zugehörige Morphismen

```text
split_command_words
canonicalize_pair
resolve_main_alias
resolve_parameter_alias
restrict
refine
```

## Priorität

```text
Priorität: absolut maximal
```

Ohne diese Transformation wäre nicht garantiert, dass Prompt, CLI und kanonische Semantik dasselbe meinen.

---

## 2.2 `PresheafToSheafGluingTransformation`

```text
LocalDataPresheafFunctor
    =>
GluedSemanticSheafFunctor
```

## Bereich

```text
lokale Datenprägarbe -> geklebte Semantikgarbe
```

## Komponenten

```text
CsvSections         -> Presheaf.restrict -> merge_parameter_dicts
TranslationSections -> SheafBundle.from_repo
PromptSections      -> PromptStatePresheaf.update -> sync_program_semantics
```

## Natürlichkeitsbedingung

```text
Lokale Sektionen über einer Überdeckung kleben zu derselben globalen Semantik,
unabhängig davon, in welcher kompatiblen Reihenfolge die Restriktionen gelesen werden.
```

## Bedeutung

Das ist die eigentliche Garben-Transformation.

Sie sagt:

```text
Lokale CSV-Dateien,
lokale i18n-Daten,
lokale Prompt-Zustände
und lokale Parametersektionen
müssen zu einer global konsistenten Semantik kleben.
```

Schema:

```text
lokale Daten
        ↓ restrict
kompatible lokale Sektionen
        ↓ glue
globale Semantik
```

## Zugehörige Topologien

```text
Kontext-Topologie
Sprach-Topologie
Prägarben-Topologie
Garben-Topologie
Parameter-Topologie
```

## Zugehörige Morphismen

```text
add_section
restrict
merge_parameter_dicts
sync_program_semantics
refine
cover_for_main
```

## Universelle Eigenschaft

```text
Kompatible lokale Sektionen kleben eindeutig zur globalen Sektion.
```

## Priorität

```text
Priorität: absolut maximal
```

Das ist der Kern von „Prägarbe -> Garbe“ im Projekt.

---

## 2.3 `TableGenerationGluingTransformation`

```text
CanonicalParameterSheafFunctor
    =>
TableGenerationGluingFunctor
```

## Bereich

```text
kanonische Semantikgarbe -> universeller Tabellenbau
```

## Komponenten

```text
ParameterSemanticsSheaf -> ColumnSelectionBundle
ColumnBuckets           -> normalize_column_buckets
ProgramWorkflow         -> ProgramWorkflowBundle.run
```

## Natürlichkeitsbedingung

```text
Kanonische Parametersemantik, Spaltenauswahl und Tabellenbau bilden ein kommutatives Workflow-Diagramm:
äquivalente Alias-/Kontextpfade erzeugen dieselbe globale Tabellensektion.
```

## Bedeutung

Diese Transformation sagt:

```text
Wenn zwei Eingaben semantisch dasselbe bedeuten,
dann müssen sie dieselbe globale Tabelle erzeugen.
```

Beispiel:

```text
religion + a1
Religionen + absicht 1
```

müssen über:

```text
canonicalize_pair
column_numbers_for_pair
normalize_column_buckets
table_generation
```

zur selben Tabelle führen.

## Zugehörige Topologien

```text
Hauptparameter-Topologie
Nebenparameter-Topologie
Spalten-Topologie
Tabellen-Topologie
```

## Zugehörige Morphismen

```text
canonicalize_pair
column_numbers_for_pair
normalize_column_buckets
prepare_output_table
ProgramWorkflowBundle.run
```

## Universelle Eigenschaft

```text
Kanonische Semantik materialisiert sich eindeutig als globale Tabelle.
```

## Priorität

```text
Priorität: absolut maximal für Runtime
```

Das ist die Brücke von Bedeutung zu Tabelle.

---

# 3. Sehr hohe Priorität: Tabellen-, Output- und Legacy-Transformationen

## 3.1 `GeneratedColumnsSheafSyncTransformation`

```text
GeneratedColumnEndofunctorFamily
    =>
ExplicitTableStateFunctor
```

## Bereich

```text
generierte Tabellen-Endofunktoren -> expliziter Tabellenzustand
```

## Komponenten

```text
GeneratedColumnRegistry -> GeneratedColumnSection.parameters
GeneratedColumnTags     -> GeneratedColumnSection.tags
GeneratedColumnsSheaf   -> sync_generated_columns_from_tables
```

## Natürlichkeitsbedingung

```text
Ein generierter Spalten-Endofunktor und die anschließende State-/Sheaf-Synchronisierung
kommutieren mit dem direkten Zugriff auf die explizite GeneratedColumnSection.
```

## Bedeutung

Generierte Spalten dürfen kein versteckter globaler Zustand sein.

Diese Transformation sagt:

```text
Wenn eine Tabelle generierte Spalten bekommt,
müssen diese Spalten sowohl im Runtime-Zustand
als auch in der Garben-Metadaten-Schicht dieselbe Struktur haben.
```

## Zugehörige Topologien

```text
Tag-Topologie
Tabellen-Topologie
Garben-Topologie
Generated-Column-Topologie
```

## Zugehörige Morphismen

```text
concat_love_polygon
concat_modallogik
create_spalte_gestirn
sync_generated_columns_from_tables
sync_tables
```

## Universelle Eigenschaft

```text
Tabelle -> generierte Tabelle
und
Tabelle -> Garbenmetadaten
müssen kommutieren.
```

## Priorität

```text
Priorität: sehr hoch
```

---

## 3.2 `TableRuntimeToStateSectionsTransformation`

```text
MutableTableRuntimeFunctor
    =>
ExplicitTableStateFunctor
```

## Bereich

```text
mutable Tabellenruntime -> explizite Tabellenzustandssektionen
```

## Komponenten

```text
Tables.generatedSpaltenParameter      -> TableStateSections.generated_columns.parameters
Tables.generatedSpaltenParameter_Tags -> TableStateSections.generated_columns.tags
Tables.rowNumDisplay2rowNumOrig       -> TableStateSections.row_display_to_original
Tables.religionNumbers                -> TableDisplayState.religion_numbers
```

## Natürlichkeitsbedingung

```text
Alte mutable Tabellenattribute und neue explizite Zustandssektionen referenzieren dieselben Objekte;
Mutation über einen Pfad ist über den anderen Pfad sichtbar.
```

## Bedeutung

Historisch hatte `reta` viel mutable Tabellenlogik.

Diese Transformation sagt:

```text
Der alte mutable Tabellenzustand
und der neue explizite TableState
dürfen nicht auseinanderlaufen.
```

## Zugehörige Topologien

```text
Tabellen-Topologie
State-Topologie
Generated-Column-Topologie
Zeilen-Topologie
```

## Zugehörige Morphismen

```text
prepare_output_table
render_table_output
TableStateSections.snapshot
sync_tables
```

## Priorität

```text
Priorität: sehr hoch
```

Diese Transformation ist wichtig, weil sie verhindert, dass alte Mutable-State-Reste die neue Architektur untergraben.

---

## 3.3 `RenderedOutputNormalizationTransformation`

```text
OutputRenderingFunctorFamily
    =>
NormalizedOutputFunctor
```

## Bereich

```text
gerenderte Ausgabe -> normalisierte Ausgabe
```

## Komponenten

```text
html     -> HTML normalisieren
markdown -> Markdown-Text vergleichen
shell    -> Shell-Text vergleichen
csv      -> CSV-Text vergleichen
```

## Natürlichkeitsbedingung

```text
Renderer-Ausgaben dürfen syntaktische Formatdetails haben,
müssen nach zulässiger Normalisierung aber dieselbe semantische Paritätsaussage ergeben.
```

## Bedeutung

Diese Transformation macht Ausgabe testbar.

Beispiel:

```text
alte Shell-Ausgabe
neue Shell-Ausgabe
```

oder:

```text
HTML-Ausgabe
Markdown-Ausgabe
CSV-Ausgabe
```

dürfen formatbedingt verschieden sein, aber nach Normalisierung muss die semantische Ausgabe stabil sein.

## Zugehörige Topologien

```text
Ausgabe-Topologie
Tabellen-Topologie
Paritäts-Topologie
```

## Zugehörige Morphismen

```text
render
render_table_output
normalize_for_parity
apply_output_mode
```

## Priorität

```text
Priorität: sehr hoch
```

Das ist die sichtbare Korrektheitsgarantie.

---

## 3.4 `LegacyToArchitectureTransformation`

```text
LegacyRuntimeFunctor
    =>
ArchitectureRuntimeFunctor
```

## Bereich

```text
alte Runtime-Fassade -> neue Architektur-Runtime
```

## Komponenten

```text
reta.py                 -> RetaArchitecture.bootstrap
libs.tableHandling      -> reta_architecture.table_runtime
libs.lib4tables_prepare -> reta_architecture.table_preparation / row_filtering / table_wrapping
libs.lib4tables_concat  -> reta_architecture.generated_columns / concat_csv / combi_join
```

## Natürlichkeitsbedingung

```text
Jeder repräsentative alte Aufrufpfad und der entsprechende neue Architekturpfad
müssen beobachtbar gleiche Ausgabe liefern.
```

## Bedeutung

Das ist die zentrale Refactor-Invariante.

Sie sagt:

```text
Alte Benutzung darf nicht brechen,
nur weil die Architektur jetzt sauberer ist.
```

Beispiel:

```text
alter Pfad über lib4tables_prepare
```

muss beobachtbar dasselbe ergeben wie:

```text
neuer Pfad über reta_architecture.table_preparation
```

## Zugehörige Topologien

```text
Legacy-Topologie
Tabellen-Topologie
Ausgabe-Topologie
Kompatibilitäts-Topologie
```

## Zugehörige Morphismen

```text
bootstrap_program
tableHandling_reexport
prepare_delegation
concat_delegation
render_table_output
normalize_for_parity
```

## Priorität

```text
Priorität: sehr hoch
```

Für neue Semantik nicht der Kern, aber für sichere Migration entscheidend.

---

# 4. Architekturvertrag-, Validierungs- und Kohärenz-Transformationen

## 4.1 `ContractedNaturalityTransformation`

```text
CategoryTheoryToContractFunctor
    =>
ArchitectureMapToContractFunctor
```

## Bereich

```text
Kategorie-Theorie -> Architekturverträge
```

## Komponenten

```text
NaturalTransformationSpec -> CommutativeDiagramSpec
ArchitectureCapsuleSpec   -> CapsuleContractSpec
RefactorInvariant         -> RefactorLawSpec
ReferenceValidation       -> ContractValidationSpec
```

## Natürlichkeitsbedingung

```text
Die aus Kategorie-Theorie und Kapselkarte abgeleiteten Vertragsdiagramme
referenzieren dieselben bekannten Kapseln, Kategorien, Funktoren
und natürlichen Transformationen.
```

## Bedeutung

Diese Transformation macht mathematische Begriffe zu konkreten Architekturverträgen.

```text
Natürliche Transformation
        ↓
kommutierendes Vertragsdiagramm
```

## Priorität

```text
Priorität: hoch für Architektur-Sicherheit
```

---

## 4.2 `ContractWitnessValidationTransformation`

```text
ContractToValidationFunctor
    =>
WitnessToValidationFunctor
```

## Bereich

```text
Architekturverträge -> Witness-/Validierungsschicht
```

## Komponenten

```text
CommutativeDiagramSpec     -> DiagramWitnessCoverageCheck
CapsuleContractSpec        -> CapsuleContractCoverageCheck
RefactorLawSpec            -> RefactorLawObligationCoverageCheck
NaturalTransformationSpec  -> NaturalTransformationWitnessCoverageCheck
RepositoryManifest         -> PackageIntegrityValidationCheck
```

## Natürlichkeitsbedingung

```text
Direkte Vertragsvalidierung und Validierung über konkrete Witnesses
müssen denselben Gesamtstatus liefern.
```

## Bedeutung

Diese Transformation verhindert reine Papierarchitektur.

Sie sagt:

```text
Ein Vertrag zählt nur, wenn es auch Witnesses / Prüfungen dafür gibt.
```

## Priorität

```text
Priorität: hoch
```

---

## 4.3 `CoherenceToTraceTransformation`

```text
CoherenceMatrixFunctor
    =>
CoherenceToTraceFunctor
```

## Bereich

```text
Kohärenzmatrix -> Trace-Navigation
```

## Komponenten

```text
Capsule     -> CapsuleTraceSpec
LegacyOwner -> RetaComponentTraceSpec
```

## Natürlichkeitsbedingung

```text
Kohärenz und Trace-Navigation führen für jede Kapsel
zum selben Diagramm-/Witness-Vertrag.
```

## Bedeutung

Sie macht Kohärenz navigierbar:

```text
Welche alte Datei?
Welche neue Kapsel?
Welcher Vertrag?
Welcher Witness?
```

## Priorität

```text
Priorität: mittel bis hoch
```

---

## 4.4 `CoherenceBoundaryValidationTransformation`

```text
CoherenceToBoundaryFunctor
    =>
LegacyImportBoundaryFunctor
```

## Bereich

```text
Kohärenzmatrix -> reale Python-Importgrenzen
```

## Komponenten

```text
Module -> ModuleOwnershipSpec
Import -> ImportEdgeSpec
```

## Natürlichkeitsbedingung

```text
Kapselgrenzen aus Kohärenz und reale Python-Importe
werden zu demselben Boundary-Graphen klassifiziert.
```

## Bedeutung

Sie verhindert, dass die Architekturkarte etwas behauptet, während reale Imports etwas anderes tun.

## Priorität

```text
Priorität: mittel bis hoch
```

---

## 4.5 `TraceBoundaryImpactTransformation`

```text
TraceBoundaryImpactFunctor
    =>
BoundaryImpactFunctor
```

## Bereich

```text
Trace + Boundary -> Impact
```

## Komponenten

```text
Owner  -> ImpactSourceSpec
Import -> ImpactSourceSpec
```

## Natürlichkeitsbedingung

```text
Impact aus Trace-Route und Impact aus Boundary-Importgraph
führen zu derselben betroffenen Kapsel-/Diagramm-/Gate-Lesart.
```

## Bedeutung

Wenn eine Datei oder ein Import geändert wird, muss klar sein:

```text
Welche Kapsel ist betroffen?
Welche Verträge sind betroffen?
Welche Gates müssen laufen?
```

## Priorität

```text
Priorität: mittel bis hoch
```

---

## 4.6 `ImpactGateValidationTransformation`

```text
MigrationCandidateFunctor
    =>
ImpactGateValidationFunctor
```

## Bereich

```text
Migrationskandidat -> Gate-Validierung
```

## Komponenten

```text
Candidate -> RegressionGateSpec
Gate      -> ImpactValidationSpec
```

## Natürlichkeitsbedingung

```text
Migrationskandidaten und Gate-Validierung kommutieren:
ein späterer Move ist nur zulässig, wenn seine Impact-Gates bestehen.
```

## Bedeutung

Diese Transformation sagt:

```text
Keine Migration ohne passende Gates.
```

## Priorität

```text
Priorität: hoch für sichere Weiterentwicklung
```

---

# 5. Migration-, Rehearsal- und Activation-Transformationen

## 5.1 `ImpactGateMigrationTransformation`

```text
ImpactToMigrationPlanFunctor
    =>
ImpactGateBindingFunctor
```

## Bereich

```text
Impact -> Migrationsplan und Gate-Binding
```

## Komponenten

```text
Candidate -> MigrationStepSpec
Gate      -> MigrationGateBindingSpec
```

## Natürlichkeitsbedingung

```text
Der direkte Pfad Impact-Kandidat -> Migrationsschritt
und der Pfad Impact-Gate -> Gate-Binding
beschreiben denselben erlaubten späteren Move.
```

## Bedeutung

Ein geplanter Migrationsschritt und seine Tests/Gates müssen zusammengehören.

## Priorität

```text
Priorität: mittel bis hoch
```

---

## 5.2 `MigrationPlanCoherenceTransformation`

```text
MigrationOrderingCoherenceFunctor
    =>
MigrationGateCoherenceFunctor
```

## Bereich

```text
Migrationswellen -> Kohärenz und Gates
```

## Komponenten

```text
Wave        -> MigrationInvariantSpec
GateBinding -> MigrationGateBindingSpec
```

## Natürlichkeitsbedingung

```text
Wellenordnung und Gate-Kohärenz kommutieren:
eine geplante Extraktion ist nur kohärent,
wenn ihre Gates und Invarianten dieselbe Welle schützen.
```

## Bedeutung

Diese Transformation schützt Migrationswellen gegen falsche Reihenfolge.

## Priorität

```text
Priorität: mittel bis hoch
```

---

## 5.3 `MigrationRehearsalNaturalityTransformation`

```text
MigrationStepRehearsalFunctor
    =>
MigrationGateRehearsalFunctor
```

## Bereich

```text
Migration -> Rehearsal
```

## Komponenten

```text
Step -> RehearsalMoveSpec
Gate -> GateRehearsalSpec
```

## Natürlichkeitsbedingung

```text
Migrationsschritt und Gate-Binding führen zum selben trockenlaufgeschützten Move.
```

## Bedeutung

Bevor eine Änderung wirklich aktiviert wird, muss sie als Rehearsal nachvollziehbar sein.

## Priorität

```text
Priorität: hoch für sichere Umbauten
```

---

## 5.4 `RehearsalReadinessValidationTransformation`

```text
RehearsalCoverFunctor
    =>
RehearsalGateValidationFunctor
```

## Bereich

```text
Rehearsal-Cover -> Validierung
```

## Komponenten

```text
Cover     -> RehearsalCoverSpec
GateSuite -> ArchitectureValidationCheckSpec
```

## Natürlichkeitsbedingung

```text
Readiness-Cover und Gate-Validierung kommutieren:
lokale Gate-Suiten kleben zur gleichen globalen Readiness-Aussage.
```

## Bedeutung

Ein Rehearsal ist nur global bereit, wenn alle lokalen Gates zusammenpassen.

## Universelle Eigenschaft

```text
lokale Rehearsals
        ↓
globales Readiness-Cover
```

## Priorität

```text
Priorität: hoch
```

---

## 5.5 `RehearsalActivationNaturalityTransformation`

```text
RehearsalActivationFunctor
    =>
GateActivationFunctor
```

## Bereich

```text
Rehearsal -> Activation
```

## Komponenten

```text
Move -> ActivationUnitSpec
Gate -> ActivationGateSpec
```

## Natürlichkeitsbedingung

```text
Aktivierung über den Rehearsal-Move und Aktivierung über die Gate-Suite
beschreiben denselben commit-geschützten Umschlag.
```

## Bedeutung

Es darf keine Aktivierung geben, die nicht durch Gate-Rehearsal gedeckt ist.

## Priorität

```text
Priorität: hoch
```

---

## 5.6 `ActivationRollbackValidationTransformation`

```text
ActivationTransactionFunctor
    =>
ActivationValidationFunctor
```

## Bereich

```text
Activation-Transaktion -> Validierung und Rollback
```

## Komponenten

```text
Transaction -> ActivationTransactionSpec
Rollback    -> ActivationRollbackSpec
```

## Natürlichkeitsbedingung

```text
Transaktionsgluing und Validierung kommutieren nur,
wenn Rollback-Sektionen für alle lokalen Aktivierungen existieren.
```

## Bedeutung

Das ist eine starke Sicherheitsbedingung:

```text
Kein Commit ohne Rollback-Sektion.
```

## Universelle Eigenschaft

```text
lokale Aktivierungen + Gates + Rollbacks
        ↓
geschützte globale Transaktion
```

## Priorität

```text
Priorität: hoch für Projektstabilität
```

---

# 6. Aktivierte Runtime-Transformationen

Diese Transformationen verbinden alte Fassaden mit neuen Architektur-Bundles.

---

## 6.1 `CenterRowRangeToArchitectureTransformation`

```text
CenterRowRangeCompatibilityFunctor
    =>
RowRangeActivationFunctor
```

## Bereich

```text
alte center-Zeilenfunktionen -> RowRangeMorphismBundle
```

## Komponenten

```text
BereichToNumbers2        -> range_to_numbers
isZeilenAngabe           -> is_row_range
strAsGeneratorToListOfNumStrs -> str_as_generator_to_set
```

## Natürlichkeitsbedingung

```text
Erst über center.py aufrufen und dann expandieren
ergibt dieselbe Zeilenmenge wie direkt über RowRangeMorphismBundle expandieren.
```

## Bedeutung

Alte Zeilenbereichsfunktionen bleiben kompatibel, aber die Architektur besitzt die Logik.

## Zugehörige Topologien

```text
Zeilen-Topologie
Legacy-Topologie
```

## Priorität

```text
Priorität: hoch
```

---

## 6.2 `RowRangeValidationTransformation`

```text
RowRangeInputFunctor
    =>
RowRangeValidationFunctor
```

## Bereich

```text
RowRange-Input -> Architekturvalidierung
```

## Komponenten

```text
RowRangeMorphismBundle -> row-ranges-json
RowIndexSet            -> architecture-validation-json
```

## Natürlichkeitsbedingung

```text
Row-Range-Ausdruck einschränken, expandieren und validieren
kommutiert mit direkter Architekturvalidierung.
```

## Bedeutung

Zeilenbereichslogik wird nicht nur ausgeführt, sondern als Architekturteil validiert.

## Priorität

```text
Priorität: hoch
```

---

## 6.3 `CenterArithmeticToArchitectureTransformation`

```text
CenterArithmeticCompatibilityFunctor
    =>
ArithmeticActivationFunctor
```

## Bereich

```text
alte center-Arithmetik -> ArithmeticMorphismBundle
```

## Komponenten

```text
multiples    -> factor_pairs
teiler       -> divisor_range
primfaktoren -> prime_factors
primRepeat   -> prime_repeat_legacy
textHatZiffer -> has_digit
```

## Natürlichkeitsbedingung

```text
Erst über center.py aufrufen und dann arithmetisch expandieren
ergibt dasselbe Ergebnis wie der direkte ArithmeticMorphismBundle-Pfad.
```

## Bedeutung

Alte arithmetische Hilfsfunktionen bleiben beobachtbar kompatibel.

## Zugehörige Topologien

```text
Arithmetik-Topologie
Zeilen-Topologie
Legacy-Topologie
```

## Priorität

```text
Priorität: hoch
```

---

## 6.4 `ArithmeticRowRangeGluingTransformation`

```text
ArithmeticRowRangeGluingFunctor
    =>
ArithmeticValidationFunctor
```

## Bereich

```text
Zeilenbereiche -> Arithmetik-Gluing -> Validierung
```

## Komponenten

```text
RowIndexSet              -> DivisorSection
ArithmeticMorphismBundle -> arithmetic-json
```

## Natürlichkeitsbedingung

```text
Row-Range-Expansion und arithmetisches Teiler-Gluing
kommutieren mit der Architekturvalidierung.
```

## Bedeutung

Zeilenbereiche und Arithmetik dürfen nicht getrennt auseinanderlaufen.

Beispiel:

```text
Zeilenmenge
        ↓
Teiler-/Faktorstruktur
        ↓
Validierung
```

muss konsistent sein.

## Priorität

```text
Priorität: hoch
```

---

## 6.5 `CenterConsoleIOToArchitectureTransformation`

```text
CenterConsoleIOCompatibilityFunctor
    =>
ConsoleIOActivationFunctor
```

## Bereich

```text
alte center-Console-/Help-Funktionen -> ConsoleIOMorphismBundle
```

## Komponenten

```text
cliout            -> cli_output
getTextWrapThings -> get_text_wrap_things
retaHilfe         -> reta_help_text
unique_everseen   -> unique_everseen
```

## Natürlichkeitsbedingung

```text
Erst über center.py aufrufen und dann rendern/zerlegen
ergibt dieselbe sichtbare Ausgabe bzw. endliche Hilfssektion
wie der direkte ConsoleIOMorphismBundle-Pfad.
```

## Bedeutung

Hilfetexte, CLI-Ausgabe und Wrapping bleiben kompatibel.

## Priorität

```text
Priorität: mittel bis hoch
```

---

## 6.6 `ConsoleIOOutputValidationTransformation`

```text
ConsoleIOOutputRenderingFunctor
    =>
ConsoleIOValidationFunctor
```

## Bereich

```text
Console-Ausgabe -> Output-Rendering / Validierung
```

## Komponenten

```text
ConsoleIOMorphismBundle -> console-io-json
ConsoleOutputSection    -> architecture-validation-json
```

## Natürlichkeitsbedingung

```text
Console-Output rendern und Console-Output validieren
kommutieren mit der bestehenden Output-Rendering-Kategorie.
```

## Bedeutung

Console-/Help-Ausgabe wird als Architektursektion prüfbar.

## Priorität

```text
Priorität: mittel bis hoch
```

---

## 6.7 `WordCompleterToArchitectureTransformation`

```text
LegacyWordCompleterCompatibilityFunctor
    =>
WordCompletionActivationFunctor
```

## Bereich

```text
alte WordCompleter-Fassade -> WordCompletionMorphismBundle
```

## Komponenten

```text
WordCompleter    -> ArchitectureWordCompleter
get_completions  -> iter_word_completions
```

## Natürlichkeitsbedingung

```text
Erst über libs.word_completerAlx.WordCompleter instanziieren
und dann Completion-Kandidaten erzeugen
ergibt dieselbe Kandidatensektion
wie der direkte WordCompletionMorphismBundle-Pfad.
```

## Bedeutung

Einfache Prompt-Completion bleibt kompatibel.

## Zugehörige Topologien

```text
Prompt-Topologie
Completion-Topologie
CursorPrefix-Topologie
Legacy-Topologie
```

## Priorität

```text
Priorität: hoch für retaPrompt
```

---

## 6.8 `WordCompletionValidationTransformation`

```text
WordCompletionPromptFunctor
    =>
WordCompletionValidationFunctor
```

## Bereich

```text
Word-Completion -> Prompt-Sektion / Validierung
```

## Komponenten

```text
CompletionCandidateSection -> word-completion-json
WordCompletionMorphismBundle -> architecture-validation-json
```

## Natürlichkeitsbedingung

```text
Prompt-Completion und Word-Completion-Validierung
kommutieren über derselben Completion-Kandidatensektion.
```

## Bedeutung

Completion ist nicht nur UI-Verhalten, sondern validierter Architekturteil.

## Priorität

```text
Priorität: hoch für retaPrompt
```

---

## 6.9 `NestedCompleterToArchitectureTransformation`

```text
LegacyNestedCompleterCompatibilityFunctor
    =>
NestedCompletionActivationFunctor
```

## Bereich

```text
alte nestedAlx-Fassade -> NestedCompletionMorphismBundle
```

## Komponenten

```text
NestedCompleter  -> ArchitectureNestedCompleter
ComplSitua       -> ComplSitua
get_completions  -> yield_nested_candidates
```

## Natürlichkeitsbedingung

```text
Erst über libs.nestedAlx.NestedCompleter instanziieren
und dann hierarchisch completieren
ergibt dieselbe Kandidatensektion
wie der direkte NestedCompletionMorphismBundle-Pfad.
```

## Bedeutung

Die komplexe hierarchische Completion bleibt kompatibel.

Beispiele:

```text
reta --spalten=
reta --zeilen=
reta --ausgabe=
```

müssen über alte und neue Completion-Pfade dieselben Kandidaten liefern.

## Priorität

```text
Priorität: hoch bis sehr hoch für retaPrompt
```

---

## 6.10 `NestedCompletionValidationTransformation`

```text
NestedCompletionPromptFunctor
    =>
NestedCompletionValidationFunctor
```

## Bereich

```text
Nested Completion -> Prompt-Sektion / Validierung
```

## Komponenten

```text
NestedCompletionCandidateSection -> nested-completion-json
NestedCompletionMorphismBundle   -> architecture-validation-json
```

## Natürlichkeitsbedingung

```text
Nested Prompt Completion und Nested-Completion-Validierung
kommutieren über derselben Completion-Kandidatensektion.
```

## Bedeutung

Hierarchische Completion wird als prüfbare Architektursektion behandelt.

## Priorität

```text
Priorität: hoch für retaPrompt
```

---

# 7. Alle 29 natürlichen Transformationen priorisiert

| Rang | Natürliche Transformation | Funktoren | Hauptrolle |
|---:|---|---|---|
| 1 | `RawToCanonicalParameterTransformation` | `RawCommandPresheafFunctor => CanonicalParameterSheafFunctor` | Rohtext wird kanonische Semantik |
| 2 | `PresheafToSheafGluingTransformation` | `LocalDataPresheafFunctor => GluedSemanticSheafFunctor` | lokale Sektionen kleben global |
| 3 | `TableGenerationGluingTransformation` | `CanonicalParameterSheafFunctor => TableGenerationGluingFunctor` | Semantik wird Tabelle |
| 4 | `GeneratedColumnsSheafSyncTransformation` | `GeneratedColumnEndofunctorFamily => ExplicitTableStateFunctor` | generierte Spalten synchronisieren mit Garbe |
| 5 | `TableRuntimeToStateSectionsTransformation` | `MutableTableRuntimeFunctor => ExplicitTableStateFunctor` | mutable Runtime wird expliziter State |
| 6 | `RenderedOutputNormalizationTransformation` | `OutputRenderingFunctorFamily => NormalizedOutputFunctor` | Ausgabe wird vergleichbare Normalform |
| 7 | `LegacyToArchitectureTransformation` | `LegacyRuntimeFunctor => ArchitectureRuntimeFunctor` | alte und neue Runtime kommutieren |
| 8 | `ContractedNaturalityTransformation` | `CategoryTheoryToContractFunctor => ArchitectureMapToContractFunctor` | Kategorie-Theorie wird Vertrag |
| 9 | `ContractWitnessValidationTransformation` | `ContractToValidationFunctor => WitnessToValidationFunctor` | Verträge und Witnesses validieren gleich |
| 10 | `CoherenceToTraceTransformation` | `CoherenceMatrixFunctor => CoherenceToTraceFunctor` | Kohärenz wird Trace |
| 11 | `CoherenceBoundaryValidationTransformation` | `CoherenceToBoundaryFunctor => LegacyImportBoundaryFunctor` | Kohärenz und reale Imports stimmen |
| 12 | `TraceBoundaryImpactTransformation` | `TraceBoundaryImpactFunctor => BoundaryImpactFunctor` | Trace-Impact und Boundary-Impact stimmen |
| 13 | `ImpactGateValidationTransformation` | `MigrationCandidateFunctor => ImpactGateValidationFunctor` | Migration nur mit Gates |
| 14 | `ImpactGateMigrationTransformation` | `ImpactToMigrationPlanFunctor => ImpactGateBindingFunctor` | Migrationsplan und Gate-Binding stimmen |
| 15 | `MigrationPlanCoherenceTransformation` | `MigrationOrderingCoherenceFunctor => MigrationGateCoherenceFunctor` | Wellenordnung und Gates stimmen |
| 16 | `MigrationRehearsalNaturalityTransformation` | `MigrationStepRehearsalFunctor => MigrationGateRehearsalFunctor` | Migration und Gate-Rehearsal stimmen |
| 17 | `RehearsalReadinessValidationTransformation` | `RehearsalCoverFunctor => RehearsalGateValidationFunctor` | Readiness-Cover und Validierung stimmen |
| 18 | `RehearsalActivationNaturalityTransformation` | `RehearsalActivationFunctor => GateActivationFunctor` | Aktivierung über Move und Gate stimmt |
| 19 | `ActivationRollbackValidationTransformation` | `ActivationTransactionFunctor => ActivationValidationFunctor` | Commit nur mit Rollback |
| 20 | `CenterRowRangeToArchitectureTransformation` | `CenterRowRangeCompatibilityFunctor => RowRangeActivationFunctor` | alte Zeilen-API und neue RowRange-API stimmen |
| 21 | `RowRangeValidationTransformation` | `RowRangeInputFunctor => RowRangeValidationFunctor` | RowRange-Ausführung und Validierung stimmen |
| 22 | `CenterArithmeticToArchitectureTransformation` | `CenterArithmeticCompatibilityFunctor => ArithmeticActivationFunctor` | alte Arithmetik und neue Arithmetik stimmen |
| 23 | `ArithmeticRowRangeGluingTransformation` | `ArithmeticRowRangeGluingFunctor => ArithmeticValidationFunctor` | Zeilen und Arithmetik kleben validierbar |
| 24 | `CenterConsoleIOToArchitectureTransformation` | `CenterConsoleIOCompatibilityFunctor => ConsoleIOActivationFunctor` | alte Console-API und neue Console-API stimmen |
| 25 | `ConsoleIOOutputValidationTransformation` | `ConsoleIOOutputRenderingFunctor => ConsoleIOValidationFunctor` | Console-Output und Validierung stimmen |
| 26 | `WordCompleterToArchitectureTransformation` | `LegacyWordCompleterCompatibilityFunctor => WordCompletionActivationFunctor` | alte WordCompletion und neue stimmen |
| 27 | `WordCompletionValidationTransformation` | `WordCompletionPromptFunctor => WordCompletionValidationFunctor` | WordCompletion und Validierung stimmen |
| 28 | `NestedCompleterToArchitectureTransformation` | `LegacyNestedCompleterCompatibilityFunctor => NestedCompletionActivationFunctor` | alte NestedCompletion und neue stimmen |
| 29 | `NestedCompletionValidationTransformation` | `NestedCompletionPromptFunctor => NestedCompletionValidationFunctor` | NestedCompletion und Validierung stimmen |

---

# 8. Nach Topologien getrennt

## Kontext-, Parameter- und Scope-Topologie

Wichtigste natürliche Transformationen:

```text
RawToCanonicalParameterTransformation
PresheafToSheafGluingTransformation
TableGenerationGluingTransformation
```

Sie sichern:

```text
Rohtext
        ↓
Kontext
        ↓
kanonische Parametersemantik
        ↓
Spalten und Tabelle
```

Priorität:

```text
maximal
```

---

## Prägarben-Topologie

Wichtigste Transformationen:

```text
RawToCanonicalParameterTransformation
PresheafToSheafGluingTransformation
RowRangeInputFunctor-bezogene Transformationen
WordCompletionValidationTransformation
NestedCompletionValidationTransformation
```

Sie sichern:

```text
lokale Sektionen
restrict
Prompt-Zustand
CSV/i18n-Sektionen
Completion-Sektionen
```

Priorität:

```text
maximal bis hoch
```

---

## Garben-Topologie

Wichtigste Transformationen:

```text
PresheafToSheafGluingTransformation
CanonicalParameterSheafFunctor-bezogene Transformationen
GeneratedColumnsSheafSyncTransformation
TableRuntimeToStateSectionsTransformation
```

Sie sichern:

```text
globale Semantik
generierte Spalten
expliziter Tabellenzustand
Garbensynchronisierung
```

Priorität:

```text
maximal
```

---

## Zeilen-Topologie

Wichtigste Transformationen:

```text
CenterRowRangeToArchitectureTransformation
RowRangeValidationTransformation
ArithmeticRowRangeGluingTransformation
```

Sie sichern:

```text
alte Zeilenfunktionen
neue RowRange-Morphismen
Zeilenmenge
Arithmetik-Gluing
Validierung
```

Priorität:

```text
hoch bis sehr hoch
```

---

## Arithmetik-Topologie

Wichtigste Transformationen:

```text
CenterArithmeticToArchitectureTransformation
ArithmeticRowRangeGluingTransformation
```

Sie sichern:

```text
Teiler
Primfaktoren
Vielfache
Ziffernprüfung
Zeilen-Arithmetik-Gluing
```

Priorität:

```text
hoch
```

---

## Ausgabe-Topologie

Wichtigste Transformationen:

```text
RenderedOutputNormalizationTransformation
ConsoleIOOutputValidationTransformation
```

Sie sichern:

```text
gerenderte Ausgabe
Normalform
Parität
Console-/Help-Ausgabe
```

Priorität:

```text
sehr hoch
```

---

## Prompt- und Completion-Topologie

Wichtigste Transformationen:

```text
WordCompleterToArchitectureTransformation
WordCompletionValidationTransformation
NestedCompleterToArchitectureTransformation
NestedCompletionValidationTransformation
RawToCanonicalParameterTransformation
```

Sie sichern:

```text
Prompt-Rohtext
Word Completion
Nested Completion
Kurzbefehle
kanonische Befehlssemantik
```

Priorität:

```text
hoch bis sehr hoch für retaPrompt
```

---

## Legacy-Topologie

Wichtigste Transformationen:

```text
LegacyToArchitectureTransformation
CenterRowRangeToArchitectureTransformation
CenterArithmeticToArchitectureTransformation
CenterConsoleIOToArchitectureTransformation
WordCompleterToArchitectureTransformation
NestedCompleterToArchitectureTransformation
```

Sie sichern:

```text
alte Pfade
neue Architekturpfade
beobachtbare Gleichheit
```

Priorität:

```text
hoch für Kompatibilität
```

---

## Metaarchitektur-, Migration- und Activation-Topologie

Wichtigste Transformationen:

```text
ContractedNaturalityTransformation
ContractWitnessValidationTransformation
CoherenceToTraceTransformation
CoherenceBoundaryValidationTransformation
TraceBoundaryImpactTransformation
ImpactGateValidationTransformation
ImpactGateMigrationTransformation
MigrationPlanCoherenceTransformation
MigrationRehearsalNaturalityTransformation
RehearsalReadinessValidationTransformation
RehearsalActivationNaturalityTransformation
ActivationRollbackValidationTransformation
```

Sie sichern:

```text
Verträge
Witnesses
Trace
Boundary
Impact
Migration
Rehearsal
Activation
Rollback
```

Priorität:

```text
hoch für Projektstabilität
```

---

# 9. Nach Prägarben, Garben, universellen Eigenschaften und Morphismen getrennt

## Prägarben-bezogene natürliche Transformationen

```text
RawToCanonicalParameterTransformation
PresheafToSheafGluingTransformation
RowRangeValidationTransformation
WordCompletionValidationTransformation
NestedCompletionValidationTransformation
```

Sie sichern:

```text
lokale Rohdaten
Prompt-Sektionen
CSV-Sektionen
Zeilensektionen
Completion-Sektionen
```

Kernbedingung:

```text
Restriktion auf kleinere Kontexte muss mit der späteren Auswertung kommutieren.
```

---

## Garben-bezogene natürliche Transformationen

```text
PresheafToSheafGluingTransformation
TableGenerationGluingTransformation
GeneratedColumnsSheafSyncTransformation
TableRuntimeToStateSectionsTransformation
```

Sie sichern:

```text
lokale Sektionen kleben global
kanonische Semantik erzeugt Tabelle
generierte Spalten werden synchronisiert
mutable Runtime und expliziter Garbenzustand bleiben gleich
```

Kernbedingung:

```text
Geklebte globale Semantik darf nicht vom Pfad abhängen.
```

---

## Universelle-Eigenschaft-bezogene natürliche Transformationen

```text
PresheafToSheafGluingTransformation
TableGenerationGluingTransformation
RenderedOutputNormalizationTransformation
RehearsalReadinessValidationTransformation
ActivationRollbackValidationTransformation
```

Sie sichern:

```text
Gluing
Normalform
Tabellenmaterialisierung
Readiness-Cover
Rollback-geschützte Transaktion
```

Kernbedingung:

```text
Der universelle Zielpunkt ist eindeutig und pfadunabhängig.
```

---

## Morphismus-bezogene natürliche Transformationen

```text
RawToCanonicalParameterTransformation
CenterRowRangeToArchitectureTransformation
CenterArithmeticToArchitectureTransformation
CenterConsoleIOToArchitectureTransformation
WordCompleterToArchitectureTransformation
NestedCompleterToArchitectureTransformation
```

Sie sichern:

```text
alte Morphismen
neue Morphismen
direkter Pfad
indirekter Pfad
```

Kernbedingung:

```text
Die Morphismen kommutieren beobachtbar.
```

---

## Funktor-bezogene natürliche Transformationen

Eigentlich sind alle 29 funktorbezogen, aber die zentralsten sind:

```text
RawCommandPresheafFunctor => CanonicalParameterSheafFunctor
LocalDataPresheafFunctor => GluedSemanticSheafFunctor
CanonicalParameterSheafFunctor => TableGenerationGluingFunctor
OutputRenderingFunctorFamily => NormalizedOutputFunctor
LegacyRuntimeFunctor => ArchitectureRuntimeFunctor
```

Kernbedingung:

```text
Zwei ganze Architekturübersetzungen liefern denselben Zielzustand.
```

---

# 10. Wichtigste Erkenntnis

Die natürlichen Transformationen sind im architekturveränderten `reta` die **Kommutativitätsgesetze**.

Sie sagen nicht einfach:

```text
Es gibt mehrere Wege.
```

Sie sagen:

```text
Mehrere Wege müssen dasselbe Ergebnis liefern.
```

Die wichtigste Kette ist:

```text
RawCommandPresheafFunctor
        => RawToCanonicalParameterTransformation
CanonicalParameterSheafFunctor
        => TableGenerationGluingTransformation
TableGenerationGluingFunctor
        => OutputRenderingFunctorFamily
NormalizedOutputFunctor
```

In normalen Worten:

```text
Rohtext wird zu kanonischer Semantik.
Kanonische Semantik wird zu Tabelle.
Tabelle wird zu Ausgabe.
Ausgabe wird normalisiert.
Alte und neue Pfade müssen dabei beobachtbar gleich bleiben.
```

Das ist die zentrale Natürlichkeit des Projekts:

```text
Egal ob ein Nutzer über Prompt, CLI, Alias, CSV, alte Legacy-Fassade
oder neue Architektur kommt:
Wenn die Bedeutung dieselbe ist,
müssen Tabelle und normalisierte Ausgabe dieselbe sein.
```

Das ist die mathematische Sicherung des Architekturumbaus.
