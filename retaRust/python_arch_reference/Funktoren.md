# Funktoren im architekturveränderten `reta`

Im aktuellen architekturveränderten `reta` sind die Funktoren in:

```text
reta_architecture/category_theory.py
```

als `FunctorSpec`-Einträge beschrieben.

Der Stand enthält:

```text
62 Funktoren
```

Ein Funktor bedeutet hier:

```text
Ein Funktor übersetzt eine ganze Kategorie strukturerhaltend in eine andere Kategorie.
```

Also nicht nur:

```text
eine Funktion A -> B
```

sondern:

```text
Objekte einer Kategorie -> Objekte einer anderen Kategorie
Morphismen einer Kategorie -> Morphismen einer anderen Kategorie
```

Einfacher gesagt:

```text
Kategorie = Bereich
Morphismus = erlaubter Übergang innerhalb eines Bereichs
Funktor = Architekturübersetzung zwischen Bereichen
Natürliche Transformation = Beweis-/Verträglichkeitsbedingung zwischen zwei solchen Übersetzungen
```

---

# 1. Wichtigste Funktor-Kette

Die zentrale Funktor-Kette des architekturveränderten `reta` ist:

```text
SchemaToTopologyFunctor
        ↓
RawCommandPresheafFunctor / LocalDataPresheafFunctor
        ↓
CanonicalParameterSheafFunctor / GluedSemanticSheafFunctor
        ↓
TableGenerationGluingFunctor
        ↓
GeneratedColumnEndofunctorFamily
        ↓
OutputRenderingFunctorFamily
        ↓
NormalizedOutputFunctor
```

Inhaltlich heißt das:

```text
Schema/Wörter/Parameter
        ↓
offene Reta-Kontexte
        ↓
lokale Prägarben-Sektionen
        ↓
globale Garben-Semantik
        ↓
universelles Gluing
        ↓
globale Tabelle
        ↓
generierte Spalten
        ↓
gerenderte Ausgabe
        ↓
normalisierte Paritätsausgabe
```

Das ist die wichtigste mathematische Pipeline.

---

# 2. Höchste Priorität: Topologie-Funktoren

Diese Funktoren betreffen die offenen Kontexte und die spezialisierten Topologien: Kontext, Zeilen, Ausgabe, Prompt und Completion.

## 2.1 `SchemaToTopologyFunctor`

```text
CanonicalSemanticSheafCategory -> OpenRetaContextCategory
covariant
```

## Aufgabe

Dieser Funktor erzeugt aus der kanonischen Schema-/Words-Struktur die Topologie der offenen Reta-Kontexte.

Er übersetzt:

```text
RetaContextSchema -> RetaContextTopology
ParameterFamilies -> ContextSelection
```

und Morphismen wie:

```text
schema_family_membership -> open_for
main_parameter_cover -> cover_for_main
```

## Bedeutung

Das ist der erste große Strukturfunktor:

```text
Wörter / Parameter / Schema
        ↓
offene Kontexte
```

Beispiel:

```text
Hauptparameter = Religionen
Sprache = deutsch
Scope = spalten
```

wird als offener Kontext lesbar.

## Priorität

```text
Priorität: maximal
```

Ohne diesen Funktor gibt es keine saubere Kontext-Topologie.

---

## 2.2 `RowRangeInputFunctor`

```text
ActivatedRowRangeCategory -> LocalSectionCategory
covariant
```

## Aufgabe

Transportiert aktivierte Zeilenbereichssektionen zurück in die lokale Input-/Prompt-Sektion.

Er übersetzt:

```text
RowRangeExpression -> LocalSection
RowIndexSet -> LocalSection
```

und:

```text
expand_row_range -> restrict
```

## Bedeutung

Dieser Funktor verbindet die Zeilen-Topologie mit der Prägarben-Schicht.

Also:

```text
Zeilensyntax / Zeilenmenge
        ↓
lokale Eingabesektion
```

Beispiel:

```text
1-10
        ↓
RowIndexSet
        ↓
lokale Sektion für Tabellen-/Promptverarbeitung
```

## Priorität

```text
Priorität: sehr hoch
```

Für Tabellenbau und Parallelisierung ist dieser Funktor wichtig.

---

## 2.3 `ArithmeticRowRangeGluingFunctor`

```text
ActivatedRowRangeCategory -> ActivatedArithmeticCategory
covariant
```

## Aufgabe

Verwendet den RowRange-MorphismBundle als Topologie für Teiler-/Faktor-Gluing.

Er übersetzt:

```text
RowIndexSet -> DivisorSection
```

und:

```text
expand_row_range -> glue_divisor_range
```

## Bedeutung

Dieser Funktor verbindet:

```text
Zeilen-Topologie
        ↓
Arithmetik-Topologie
```

Beispiel:

```text
Zeilenmenge {6, 12, 18}
        ↓
Teiler-/Faktorstruktur über diesen Zeilen
```

## Priorität

```text
Priorität: hoch bis sehr hoch
```

Weil viele `reta`-Strukturen zahlen- und zeilenbasiert sind.

---

## 2.4 `OutputRenderingFunctorFamily`

```text
TableSectionCategory -> OutputFormatCategory
covariant
```

## Aufgabe

Renderer-Funktoren von globalen Tabellensektionen in konkrete Ausgabeformate.

Er übersetzt:

```text
Tables -> RenderedOutput
TableStateSections -> OutputModeSpec
```

und:

```text
prepare_output_table -> render
apply_output_mode -> apply_output_mode
```

## Bedeutung

Dieser Funktor verbindet Tabellen-Topologie mit Ausgabe-Topologie.

Also:

```text
globale Tabelle
        ↓
Shell / HTML / CSV / Markdown / BBCode / Emacs
```

## Priorität

```text
Priorität: sehr hoch
```

Die Ausgabe ist das beobachtbare Ergebnis.

---

## 2.5 `NormalizedOutputFunctor`

```text
OutputFormatCategory -> OutputFormatCategory
covariant
```

## Aufgabe

Überführt verschiedene Renderer-Ausgaben in paritätsfähige Normalformen.

Er übersetzt:

```text
RenderedOutput -> NormalizedOutput
OutputSyntax -> OutputSyntax
```

und:

```text
render -> normalize_for_parity
```

## Bedeutung

Dieser Funktor ist für Vergleichbarkeit entscheidend:

```text
alte Ausgabe
neue Ausgabe
        ↓
Normalform
```

## Priorität

```text
Priorität: sehr hoch
```

Ohne ihn ist Legacy-Parität schwer prüfbar.

---

## 2.6 `WordCompletionPromptFunctor`

```text
ActivatedWordCompletionCategory -> LocalSectionCategory
covariant
```

## Aufgabe

Transportiert einfache Completion-Kandidaten zurück in die Prompt-/Input-Kapsel.

Er übersetzt:

```text
CompletionCandidateSection -> PromptCompletionSection
```

und:

```text
iter_word_completions -> prompt_completion
```

## Bedeutung

Dieser Funktor verbindet Word-Completion-Topologie mit lokaler Prompt-Prägarbe.

```text
Cursor-Präfix
        ↓
Completion-Kandidatensektion
        ↓
Prompt-Sektion
```

## Priorität

```text
Priorität: hoch für retaPrompt
```

---

## 2.7 `NestedCompletionPromptFunctor`

```text
ActivatedNestedCompletionCategory -> LocalSectionCategory
covariant
```

## Aufgabe

Transportiert hierarchische Completion-Kandidaten zurück in die Prompt-/Input-Kapsel.

Er übersetzt:

```text
NestedCompletionCandidateSection -> PromptCompletionSection
```

und:

```text
yield_nested_candidates -> prompt_completion
```

## Bedeutung

Dieser Funktor verbindet die Nested-Completion-Topologie mit der Prompt-Prägarbe.

Also:

```text
reta --spalten=
reta --zeilen=
reta --ausgabe=
        ↓
lokale Prompt-Completion-Sektion
```

## Priorität

```text
Priorität: hoch für retaPrompt
```

---

# 3. Höchste Priorität: Prägarben-Funktoren

Prägarben-Funktoren sind im Projekt besonders wichtig, weil sie rohe lokale Daten über offenen Kontexten organisieren.

Wichtig: Prägarben sind normalerweise **kontravariant**. Genau das sieht man hier.

---

## 3.1 `RawCommandPresheafFunctor`

```text
OpenRetaContextCategory -> LocalSectionCategory
contravariant
```

## Aufgabe

Ordnet jedem offenen Kontext lokale, noch nicht kanonisierte Prompt-/Datei-Sektionen zu.

Er übersetzt:

```text
ContextSelection -> LocalSection
PromptText -> PromptStatePresheaf
```

und:

```text
refine -> restrict
open_for -> add_section
```

## Bedeutung

Das ist der wichtigste Rohkommando-Prägarben-Funktor.

Er sagt:

```text
Ein roher Prompt- oder CLI-Ausdruck ist eine lokale Sektion über einem offenen Kontext.
```

Beispiele:

```text
a1
reta --spalten religion
reta --zeilen 1-10
```

sind zunächst keine globale Semantik, sondern lokale Rohsektionen.

## Universelle Bedeutung

Die zugehörige Natürlichkeit lautet:

```text
erst Kontext einschränken, dann kanonisieren
=
erst kanonisieren, dann Kontext einschränken
```

## Priorität

```text
Priorität: maximal für Prompt/CLI
```

---

## 3.2 `LocalDataPresheafFunctor`

```text
OpenRetaContextCategory -> LocalSectionCategory
contravariant
```

## Aufgabe

Modelliert CSVs, Übersetzungen und Assets als lokale Prägarben-Sektionen.

Er übersetzt:

```text
ContextSelection -> FilesystemPresheaf
CsvContext -> LocalSection
```

und:

```text
refine -> restrict
cover_for_main -> add_section
```

## Bedeutung

Dieser Funktor behandelt lokale Dateien als Sektionen.

Also:

```text
CSV-Dateien
i18n-Dateien
Assets
Dokumentation
        ↓
lokale Sektionen über Kontexten
```

Beispiel:

```text
csv/en-religion.csv
```

liegt über einem englischen Religions-/CSV-Kontext.

## Priorität

```text
Priorität: maximal
```

Ohne lokale Daten-Prägarbe gibt es nichts zu kleben.

---

## 3.3 `RowRangeInputFunctor`

```text
ActivatedRowRangeCategory -> LocalSectionCategory
covariant
```

## Aufgabe

Auch dieser Funktor wirkt prägarbenartig, weil er Zeilenbereichsausdrücke in lokale Input-Sektionen zurückführt.

```text
RowRangeExpression -> LocalSection
RowIndexSet -> LocalSection
```

## Bedeutung

Zeilenangaben werden als lokale Sektionen nutzbar.

```text
1-10
primzahlen
vielfache
        ↓
lokale Zeilensektion
```

## Priorität

```text
Priorität: sehr hoch
```

---

## 3.4 `WordCompletionPromptFunctor`

```text
ActivatedWordCompletionCategory -> LocalSectionCategory
covariant
```

## Aufgabe

Macht Word-Completion-Kandidaten zu lokalen Prompt-Sektionen.

## Priorität

```text
Priorität: hoch für retaPrompt
```

---

## 3.5 `NestedCompletionPromptFunctor`

```text
ActivatedNestedCompletionCategory -> LocalSectionCategory
covariant
```

## Aufgabe

Macht Nested-Completion-Kandidaten zu lokalen Prompt-Sektionen.

## Priorität

```text
Priorität: hoch für retaPrompt
```

---

# 4. Höchste Priorität: Garben-Funktoren

Garben-Funktoren kleben lokale Daten zu globaler Semantik.

---

## 4.1 `CanonicalParameterSheafFunctor`

```text
OpenRetaContextCategory -> CanonicalSemanticSheafCategory
contravariant
```

## Aufgabe

Ordnet offenen Kontexten die geklebte kanonische Parametersemantik zu.

Er übersetzt:

```text
ContextSelection -> ParameterSemanticsSheaf
ContextCover -> ParameterSemanticsSheaf
```

und:

```text
refine -> canonicalize_pair
cover_for_main -> column_numbers_for_pair
```

## Bedeutung

Das ist einer der wichtigsten Funktoren überhaupt.

Er macht aus Kontexten globale Semantik:

```text
offener Kontext
        ↓
ParameterSemanticsSheaf
```

Beispiel:

```text
Religionen + a1
        ↓
kanonisches Parameterpaar
        ↓
Spaltennummernmenge
```

## Priorität

```text
Priorität: absolut maximal
```

Wenn dieser Funktor falsch ist, ist die Bedeutung falsch.

---

## 4.2 `GluedSemanticSheafFunctor`

```text
OpenRetaContextCategory -> CanonicalSemanticSheafCategory
contravariant
```

## Aufgabe

Klebt lokale Daten zu global verwendbarer Semantik.

Er übersetzt:

```text
ContextSelection -> SheafBundle
CsvContext -> ParameterSemanticsSheaf
```

und:

```text
refine -> sync_program_semantics
cover_for_main -> merge_parameter_dicts
```

## Bedeutung

Dieser Funktor ist die eigentliche Sheafification-Schicht:

```text
lokale CSV-/i18n-/Prompt-Sektionen
        ↓
global verwendbare Semantik
```

## Priorität

```text
Priorität: maximal
```

Weil hier lokale Daten wirklich zu globaler Semantik werden.

---

## 4.3 `GeneratedColumnEndofunctorFamily`

```text
TableSectionCategory -> TableSectionCategory
covariant
```

## Aufgabe

Familie von Tabellen-Endofunktoren, die abgeleitete Spalten erzeugen.

Er übersetzt:

```text
Tables -> Tables
GeneratedColumnSection -> GeneratedColumnSection
```

und:

```text
concat_love_polygon -> concat_love_polygon
concat_modallogik -> concat_modallogik
create_spalte_gestirn -> create_spalte_gestirn
```

## Bedeutung

Ein Endofunktor bleibt in derselben Kategorie:

```text
Tabelle -> Tabelle
```

Aber mit zusätzlicher Struktur:

```text
Tabelle
        ↓
Tabelle mit generierten Spalten
```

## Priorität

```text
Priorität: sehr hoch
```

Generierte Spalten dürfen kein versteckter globaler Zustand sein.

---

## 4.4 `ExplicitTableStateFunctor`

```text
TableSectionCategory -> TableSectionCategory
covariant
```

## Aufgabe

Projiziert die globale Runtime in explizite, inspizierbare Tabellenzustandssektionen.

Er übersetzt:

```text
Tables -> TableStateSections
GeneratedColumnSection -> GeneratedColumnSection
```

und:

```text
prepare_output_table -> TableStateSections.snapshot
render_table_output -> TableStateSections.snapshot
```

## Bedeutung

Dieser Funktor macht impliziten mutable Tabellenzustand sichtbar.

```text
mutable Tables
        ↓
explizite TableStateSections
```

## Priorität

```text
Priorität: sehr hoch
```

Wichtig für Debugging, Tests, Garbensynchronisierung und Architekturklarheit.

---

# 5. Höchste Priorität: Funktoren universeller Eigenschaften

Diese Funktoren transportieren Gluing, Normalisierung, Synchronisierung und Materialisierung.

---

## 5.1 `TableGenerationGluingFunctor`

```text
UniversalConstructionCategory -> TableSectionCategory
covariant
```

## Aufgabe

Führt Parameter- und Spaltenauswahl über universelles Gluing zur globalen Tabellen-Sektion.

Er übersetzt:

```text
ParameterDictionaryDiagram -> Tables
ColumnBucketDiagram -> TableStateSections
```

und:

```text
merge_parameter_dicts -> create_tables
normalize_column_buckets -> prepare_output_table
```

## Bedeutung

Das ist der zentrale Materialisierungsfunktor:

```text
universelles Gluing
        ↓
globale Tabelle
```

Er macht aus:

```text
Parametersemantik
Spaltennormalisierung
lokalem Gluing
```

eine echte Tabelle.

## Priorität

```text
Priorität: absolut maximal für Runtime
```

---

## 5.2 `NormalizedOutputFunctor`

```text
OutputFormatCategory -> OutputFormatCategory
covariant
```

## Aufgabe

Macht gerenderte Ausgabe vergleichbar.

```text
RenderedOutput -> NormalizedOutput
```

## Universelle Eigenschaft

```text
Verschiedene Ausgabeformen werden auf eine gemeinsame Normalform gebracht.
```

## Priorität

```text
Priorität: sehr hoch
```

---

## 5.3 `GeneratedColumnEndofunctorFamily`

```text
TableSectionCategory -> TableSectionCategory
covariant
```

## Aufgabe

Erzeugt generierte Spalten als Endofunktoren der Tabelle.

## Universelle Eigenschaft

```text
Die Tabelle bleibt Tabelle,
aber mit eindeutig synchronisierbarer Zusatzstruktur.
```

## Priorität

```text
Priorität: sehr hoch
```

---

## 5.4 `RehearsalCoverFunctor`

```text
ArchitectureRehearsalCategory -> ArchitectureRehearsalCategory
covariant
```

## Aufgabe

Klebt lokale Rehearsal-Sektionen zu einem Readiness-Cover.

Er übersetzt:

```text
RehearsalOpenSetSpec -> RehearsalCoverSpec
```

und:

```text
rehearse_step -> cover_wave
```

## Bedeutung

Dieser Funktor ist eine Meta-Garben-Idee:

```text
lokale Trockenläufe
        ↓
globales Readiness-Cover
```

## Priorität

```text
Priorität: hoch für Refactoring-Sicherheit
```

---

## 5.5 `ActivationTransactionFunctor`

```text
ArchitectureActivationCategory -> ArchitectureActivationCategory
covariant
```

## Aufgabe

Klebt lokale Aktivierungseinheiten zu einer Transaktion pro Fenster.

Er übersetzt:

```text
ActivationWindowSpec -> ActivationTransactionSpec
```

und:

```text
activate_move -> commit_transaction
```

## Bedeutung

Dieser Funktor behandelt Aktivierung als universelle Transaktion:

```text
lokale Aktivierungseinheiten
        ↓
globale Commit-Transaktion
```

## Priorität

```text
Priorität: hoch für Projektstabilität
```

---

## 5.6 `ActivationRollbackFunctor`

```text
ArchitectureActivationCategory -> ArchitectureActivationCategory
covariant
```

## Aufgabe

Ordnet jedem Commit-Gate eine Rollback-Sektion zu.

Er übersetzt:

```text
ActivationGateSpec -> ActivationRollbackSpec
```

und:

```text
activate_gate -> rollback
```

## Bedeutung

Jede Aktivierung bekommt eine Rücknahme-Sektion.

```text
Commit-Gate
        ↓
Rollback-Sektion
```

## Priorität

```text
Priorität: hoch für sichere Migration
```

---

# 6. Runtime- und Morphismen-Funktoren

Diese Funktoren setzen die mathematische Architektur in konkrete Programmläufe um.

---

## 6.1 `MutableTableRuntimeFunctor`

```text
TableSectionCategory -> TableSectionCategory
covariant
```

## Aufgabe

Historisch-mutable Sicht der globalen Tabellensektion.

Er übersetzt:

```text
Tables -> Tables
TableOutput -> TableOutput
```

und:

```text
prepare_output_table -> prepare_output_table
render_table_output -> render_table_output
```

## Bedeutung

Dieser Funktor beschreibt den bestehenden Tabellenpfad:

```text
mutable Tabelle
        ↓
vorbereitete mutable Tabelle
        ↓
gerenderte Ausgabe
```

## Priorität

```text
Priorität: hoch
```

Wichtig, weil `reta` historisch mutable Tabellenzustände nutzt.

---

## 6.2 `ExplicitTableStateFunctor`

```text
TableSectionCategory -> TableSectionCategory
covariant
```

## Aufgabe

Macht mutable Tabellenzustände explizit.

## Priorität

```text
Priorität: sehr hoch
```

Er ist der Gegenpol zum mutable Runtime-Funktor.

---

## 6.3 `OutputRenderingFunctorFamily`

```text
TableSectionCategory -> OutputFormatCategory
covariant
```

## Aufgabe

Rendert globale Tabelle in Ausgabeformate.

## Priorität

```text
Priorität: sehr hoch
```

---

## 6.4 `LegacyRuntimeFunctor`

```text
LegacyFacadeCategory -> TableSectionCategory
covariant
```

## Aufgabe

Beschreibt, wie alte Importpfade auf die neue Tabellenarchitektur abgebildet werden.

Er übersetzt:

```text
libs.tableHandling -> Tables
libs.lib4tables_prepare -> TablePreparationBundle
libs.lib4tables_concat -> GeneratedColumnRegistry
```

und:

```text
tableHandling_reexport -> create_tables
prepare_delegation -> prepare_output_table
concat_delegation -> GeneratedColumnEndofunctorFamily
```

## Bedeutung

Alter Runtime-Pfad:

```text
Legacy-Fassade
        ↓
Tabellenkategorie
```

## Priorität

```text
Priorität: hoch für Kompatibilität
```

---

## 6.5 `ArchitectureRuntimeFunctor`

```text
LegacyFacadeCategory -> TableSectionCategory
covariant
```

## Aufgabe

Der neue strukturierte Runtime-Pfad über die Architektur-Fassade.

Er übersetzt:

```text
reta.py Program -> RetaArchitecture
libs.tableHandling -> Tables
libs.lib4tables_prepare -> TablePreparationBundle
libs.lib4tables_concat -> GeneratedColumnRegistry
```

und:

```text
bootstrap_program -> bootstrap_table_runtime
tableHandling_reexport -> create_tables
prepare_delegation -> bootstrap_table_preparation
concat_delegation -> bootstrap_generated_columns
```

## Bedeutung

Neuer Runtime-Pfad:

```text
Program / alte Fassade
        ↓
RetaArchitecture
        ↓
TableSectionCategory
```

## Priorität

```text
Priorität: hoch
```

Wichtig für die Brücke zwischen altem Einstieg und neuer Architektur.

---

# 7. Aktivierte Runtime-Funktoren

Diese Funktoren zeigen, dass vormals alte Legacy-Logik jetzt echte Architekturbesitzer hat.

---

## 7.1 Row-Range-Funktoren

### `RowRangeActivationFunctor`

```text
ArchitectureActivationCategory -> ActivatedRowRangeCategory
```

Aktiviert Row-Range-Logik als realen Architekturbesitz.

Priorität:

```text
sehr hoch
```

---

### `CenterRowRangeCompatibilityFunctor`

```text
LegacyFacadeCategory -> ActivatedRowRangeCategory
```

Liest alte `center`-Zeilenbereichsfunktionen als Fassaden über den neuen `RowRangeMorphismBundle`.

Priorität:

```text
hoch für Kompatibilität
```

---

### `RowRangeInputFunctor`

```text
ActivatedRowRangeCategory -> LocalSectionCategory
```

Transportiert Zeilenbereichssektionen zurück in lokale Input-/Prompt-Sektionen.

Priorität:

```text
sehr hoch
```

---

### `RowRangeValidationFunctor`

```text
ActivatedRowRangeCategory -> ArchitectureValidationCategory
```

Macht aktivierte Row-Range-Migration prüfbar.

Priorität:

```text
hoch
```

---

## 7.2 Arithmetik-Funktoren

### `ArithmeticActivationFunctor`

```text
ArchitectureActivationCategory -> ActivatedArithmeticCategory
```

Aktiviert `center`-Arithmetik als echten Architekturbesitz.

Priorität:

```text
hoch
```

---

### `CenterArithmeticCompatibilityFunctor`

```text
LegacyFacadeCategory -> ActivatedArithmeticCategory
```

Liest alte `center`-Arithmetiknamen als Fassaden über `ArithmeticMorphismBundle`.

Priorität:

```text
hoch für Kompatibilität
```

---

### `ArithmeticRowRangeGluingFunctor`

```text
ActivatedRowRangeCategory -> ActivatedArithmeticCategory
```

Verwendet RowRange-Logik als Topologie für Teiler-/Faktor-Gluing.

Priorität:

```text
sehr hoch
```

---

### `ArithmeticValidationFunctor`

```text
ActivatedArithmeticCategory -> ArchitectureValidationCategory
```

Macht aktivierte Arithmetik-Migration prüfbar.

Priorität:

```text
hoch
```

---

## 7.3 Console-/Help-/Utility-Funktoren

### `ConsoleIOActivationFunctor`

```text
ArchitectureActivationCategory -> ActivatedConsoleIOCategory
```

Aktiviert Console-/Help-/Utility-Logik als Architekturbesitz.

---

### `CenterConsoleIOCompatibilityFunctor`

```text
LegacyFacadeCategory -> ActivatedConsoleIOCategory
```

Liest alte `center`-Console-Namen als Fassaden über `ConsoleIOMorphismBundle`.

---

### `ConsoleIOOutputRenderingFunctor`

```text
ActivatedConsoleIOCategory -> OutputFormatCategory
```

Transportiert Console-Ausgabesektionen in die Output-Rendering-Kategorie.

---

### `ConsoleIOValidationFunctor`

```text
ActivatedConsoleIOCategory -> ArchitectureValidationCategory
```

Macht die Console-/Utility-Migration prüfbar.

Priorität der Gruppe:

```text
mittel bis hoch
```

---

## 7.4 Word-Completion-Funktoren

### `WordCompletionActivationFunctor`

```text
ArchitectureActivationCategory -> ActivatedWordCompletionCategory
```

Aktiviert `word_completerAlx` als reale Architektur-Completion-Logik.

---

### `LegacyWordCompleterCompatibilityFunctor`

```text
LegacyFacadeCategory -> ActivatedWordCompletionCategory
```

Liest den alten `WordCompleter`-Import als Fassade über die Architektur-Completion-Klasse.

---

### `WordCompletionPromptFunctor`

```text
ActivatedWordCompletionCategory -> LocalSectionCategory
```

Transportiert Completion-Kandidaten zurück in die Prompt-/Input-Kapsel.

---

### `WordCompletionValidationFunctor`

```text
ActivatedWordCompletionCategory -> ArchitectureValidationCategory
```

Macht die aktivierte Word-Completion-Migration prüfbar.

Priorität der Gruppe:

```text
hoch für retaPrompt
```

---

## 7.5 Nested-Completion-Funktoren

### `NestedCompletionActivationFunctor`

```text
ArchitectureActivationCategory -> ActivatedNestedCompletionCategory
```

Aktiviert `nestedAlx` als reale Architektur-Completion-Logik.

---

### `LegacyNestedCompleterCompatibilityFunctor`

```text
LegacyFacadeCategory -> ActivatedNestedCompletionCategory
```

Liest den alten `nestedAlx`-Import als Fassade über die Architektur-Nested-Completion-Klasse.

---

### `NestedCompletionPromptFunctor`

```text
ActivatedNestedCompletionCategory -> LocalSectionCategory
```

Transportiert hierarchische Completion-Kandidaten zurück in die Prompt-/Input-Kapsel.

---

### `NestedCompletionValidationFunctor`

```text
ActivatedNestedCompletionCategory -> ArchitectureValidationCategory
```

Macht die aktivierte Nested-Completion-Migration prüfbar.

Priorität der Gruppe:

```text
hoch für retaPrompt
```

---

# 8. Metaarchitektur-Funktoren

Diese Funktoren sind nicht primär Runtime, sondern sichern Architektur, Refactoring, Validierung und Migration.

---

## 8.1 Vertrags- und Validierungsfunktoren

### `CategoryTheoryToContractFunctor`

```text
CommutativeArchitectureContractCategory -> CommutativeArchitectureContractCategory
```

Hebt Kategorien, Funktoren und natürliche Transformationen in Vertragsdiagramme.

---

### `ArchitectureMapToContractFunctor`

```text
CommutativeArchitectureContractCategory -> CommutativeArchitectureContractCategory
```

Macht aus der Architekturkarte explizite Kapselverträge und kommutierende Diagramme.

---

### `ContractToValidationFunctor`

```text
CommutativeArchitectureContractCategory -> ArchitectureValidationCategory
```

Hebt Verträge in ausführbare Validierungschecks.

---

### `WitnessToValidationFunctor`

```text
CommutativeArchitectureContractCategory -> ArchitectureValidationCategory
```

Hebt Witnesses in ausführbare Validierungschecks.

Priorität der Gruppe:

```text
hoch für Architektur-Sicherheit
```

---

## 8.2 Kohärenz-, Trace- und Boundary-Funktoren

### `CoherenceMatrixFunctor`

```text
CommutativeArchitectureContractCategory -> ArchitectureCoherenceCategory
```

Führt Verträge, Witnesses und Kapseln als Kohärenzmatrix zusammen.

---

### `CoherenceToTraceFunctor`

```text
ArchitectureCoherenceCategory -> ArchitectureTraceCategory
```

Macht Kohärenzzeilen als navigierbare Trace-Routen sichtbar.

---

### `LegacyOwnershipTraceFunctor`

```text
LegacyFacadeCategory -> ArchitectureTraceCategory
```

Ordnet alte `reta`-Besitzer ihren neuen Kapselspuren zu.

---

### `CoherenceToBoundaryFunctor`

```text
ArchitectureCoherenceCategory -> ArchitectureBoundaryCategory
```

Projiziert Kohärenz auf konkrete Modul- und Importgrenzen.

---

### `LegacyImportBoundaryFunctor`

```text
LegacyFacadeCategory -> ArchitectureBoundaryCategory
```

Klassifiziert Legacy-Importe als sichtbare Kapselgrenzen.

Priorität der Gruppe:

```text
hoch für Refactoring-Sicherheit
```

---

## 8.3 Impact-Funktoren

### `TraceBoundaryImpactFunctor`

```text
ArchitectureTraceCategory -> ArchitectureImpactCategory
```

Projiziert Trace-Routen auf Impact-Quellen und betroffene Verträge.

---

### `BoundaryImpactFunctor`

```text
ArchitectureBoundaryCategory -> ArchitectureImpactCategory
```

Projiziert reale Importgrenzen auf Impact-Quellen.

---

### `ImpactGateValidationFunctor`

```text
ArchitectureImpactCategory -> ArchitectureCoherenceCategory
```

Führt Impact-Gates in die validierbare Kohärenzschicht zurück.

---

### `MigrationCandidateFunctor`

```text
LegacyFacadeCategory -> ArchitectureImpactCategory
```

Liest Legacy-Owner als guarded Migrationskandidaten.

Priorität der Gruppe:

```text
mittel bis hoch
```

---

## 8.4 Migrationsplan-Funktoren

### `ImpactToMigrationPlanFunctor`

```text
ArchitectureImpactCategory -> ArchitectureMigrationCategory
```

Projiziert Migrationskandidaten auf geordnete Migrationsschritte.

---

### `ImpactGateBindingFunctor`

```text
ArchitectureImpactCategory -> ArchitectureMigrationCategory
```

Projiziert Impact-Gates auf konkrete Gate-Bindings.

---

### `MigrationWaveOrderingFunctor`

```text
ArchitectureMigrationCategory -> ArchitectureMigrationCategory
```

Endofunktor, der geplante Schritte stufenweise/kapselweise ordnet.

---

### `MigrationOrderingCoherenceFunctor`

```text
ArchitectureMigrationCategory -> ArchitectureCoherenceCategory
```

Reflektiert Migrationswellen in die Kohärenzmatrix.

---

### `MigrationGateCoherenceFunctor`

```text
ArchitectureMigrationCategory -> ArchitectureCoherenceCategory
```

Reflektiert Gate-Bindings in die validierbare Kohärenzschicht.

Priorität der Gruppe:

```text
mittel bis hoch
```

---

## 8.5 Rehearsal-Funktoren

### `MigrationStepRehearsalFunctor`

```text
ArchitectureMigrationCategory -> ArchitectureRehearsalCategory
```

Projiziert Migrationsschritte auf Trockenlauf-Moves.

---

### `MigrationGateRehearsalFunctor`

```text
ArchitectureMigrationCategory -> ArchitectureRehearsalCategory
```

Projiziert Gate-Bindings auf Preflight-/Postflight-Rehearsal-Suites.

---

### `RehearsalCoverFunctor`

```text
ArchitectureRehearsalCategory -> ArchitectureRehearsalCategory
```

Klebt lokale Rehearsal-Sektionen zu einem Readiness-Cover.

---

### `RehearsalGateValidationFunctor`

```text
ArchitectureRehearsalCategory -> ArchitectureValidationCategory
```

Reflektiert Gate-Rehearsals in die Validierungsschicht.

---

### `RehearsalReadinessCoherenceFunctor`

```text
ArchitectureRehearsalCategory -> ArchitectureCoherenceCategory
```

Reflektiert Readiness in die Kohärenzmatrix.

Priorität der Gruppe:

```text
hoch für sichere Umbauten
```

---

## 8.6 Activation-Funktoren

### `RehearsalActivationFunctor`

```text
ArchitectureRehearsalCategory -> ArchitectureActivationCategory
```

Projiziert Rehearsal-Moves auf Aktivierungseinheiten.

---

### `GateActivationFunctor`

```text
ArchitectureRehearsalCategory -> ArchitectureActivationCategory
```

Projiziert Gate-Rehearsals auf Commit-/Rollback-Gates.

---

### `ActivationTransactionFunctor`

```text
ArchitectureActivationCategory -> ArchitectureActivationCategory
```

Klebt lokale Aktivierungseinheiten zu einer Transaktion pro Fenster.

---

### `ActivationRollbackFunctor`

```text
ArchitectureActivationCategory -> ArchitectureActivationCategory
```

Ordnet jedem Commit-Gate eine Rollback-Sektion zu.

---

### `ActivationValidationFunctor`

```text
ArchitectureActivationCategory -> ArchitectureValidationCategory
```

Reflektiert Aktivierungsfenster, Gates und Rollback-Sektionen in die Validierung.

---

### `ActivationCoherenceFunctor`

```text
ArchitectureActivationCategory -> ArchitectureCoherenceCategory
```

Reflektiert Aktivierungstransaktionen in die Kohärenzmatrix.

Priorität der Gruppe:

```text
hoch für Projektstabilität
```

---

# 9. Gesamtliste aller 62 Funktoren

| Rang | Funktor | Quelle -> Ziel | Varianz | Hauptrolle |
|---:|---|---|---|---|
| 1 | `SchemaToTopologyFunctor` | `CanonicalSemanticSheafCategory -> OpenRetaContextCategory` | covariant | Schema/Wörter zu offener Kontext-Topologie |
| 2 | `RawCommandPresheafFunctor` | `OpenRetaContextCategory -> LocalSectionCategory` | contravariant | Rohkommandos als lokale Prägarben-Sektionen |
| 3 | `CanonicalParameterSheafFunctor` | `OpenRetaContextCategory -> CanonicalSemanticSheafCategory` | contravariant | offene Kontexte zu kanonischer Parametersemantik |
| 4 | `LocalDataPresheafFunctor` | `OpenRetaContextCategory -> LocalSectionCategory` | contravariant | CSV/i18n/Assets als lokale Prägarbe |
| 5 | `GluedSemanticSheafFunctor` | `OpenRetaContextCategory -> CanonicalSemanticSheafCategory` | contravariant | lokale Daten zu globaler Semantik kleben |
| 6 | `TableGenerationGluingFunctor` | `UniversalConstructionCategory -> TableSectionCategory` | covariant | universelles Gluing zu globaler Tabelle |
| 7 | `GeneratedColumnEndofunctorFamily` | `TableSectionCategory -> TableSectionCategory` | covariant | generierte Spalten als Tabellen-Endofunktoren |
| 8 | `OutputRenderingFunctorFamily` | `TableSectionCategory -> OutputFormatCategory` | covariant | Tabelle zu Ausgabeformaten |
| 9 | `NormalizedOutputFunctor` | `OutputFormatCategory -> OutputFormatCategory` | covariant | gerenderte Ausgabe zu Normalform |
| 10 | `LegacyRuntimeFunctor` | `LegacyFacadeCategory -> TableSectionCategory` | covariant | alte Runtime-Pfade zur Tabellenarchitektur |
| 11 | `ArchitectureRuntimeFunctor` | `LegacyFacadeCategory -> TableSectionCategory` | covariant | neuer Runtime-Pfad über Architektur-Fassade |
| 12 | `MutableTableRuntimeFunctor` | `TableSectionCategory -> TableSectionCategory` | covariant | mutable Tabellenruntime |
| 13 | `ExplicitTableStateFunctor` | `TableSectionCategory -> TableSectionCategory` | covariant | Runtime zu expliziten Tabellenzuständen |
| 14 | `CategoryTheoryToContractFunctor` | `CommutativeArchitectureContractCategory -> CommutativeArchitectureContractCategory` | covariant | Kategorien/Funktoren zu Vertragsdiagrammen |
| 15 | `ArchitectureMapToContractFunctor` | `CommutativeArchitectureContractCategory -> CommutativeArchitectureContractCategory` | covariant | Architekturkarte zu Kapselverträgen |
| 16 | `ContractToValidationFunctor` | `CommutativeArchitectureContractCategory -> ArchitectureValidationCategory` | covariant | Verträge zu Validierungschecks |
| 17 | `WitnessToValidationFunctor` | `CommutativeArchitectureContractCategory -> ArchitectureValidationCategory` | covariant | Witnesses zu Validierungschecks |
| 18 | `CoherenceMatrixFunctor` | `CommutativeArchitectureContractCategory -> ArchitectureCoherenceCategory` | covariant | Verträge/Witnesses zu Kohärenzmatrix |
| 19 | `CoherenceToTraceFunctor` | `ArchitectureCoherenceCategory -> ArchitectureTraceCategory` | covariant | Kohärenz zu Trace-Routen |
| 20 | `LegacyOwnershipTraceFunctor` | `LegacyFacadeCategory -> ArchitectureTraceCategory` | covariant | Legacy-Besitzer zu Architekturspuren |
| 21 | `CoherenceToBoundaryFunctor` | `ArchitectureCoherenceCategory -> ArchitectureBoundaryCategory` | covariant | Kohärenz zu Modulgrenzen |
| 22 | `LegacyImportBoundaryFunctor` | `LegacyFacadeCategory -> ArchitectureBoundaryCategory` | covariant | Legacy-Importe zu Boundary-Graph |
| 23 | `TraceBoundaryImpactFunctor` | `ArchitectureTraceCategory -> ArchitectureImpactCategory` | covariant | Trace zu Impact |
| 24 | `BoundaryImpactFunctor` | `ArchitectureBoundaryCategory -> ArchitectureImpactCategory` | covariant | Importgrenzen zu Impact |
| 25 | `ImpactGateValidationFunctor` | `ArchitectureImpactCategory -> ArchitectureCoherenceCategory` | covariant | Impact-Gates zu Kohärenz |
| 26 | `MigrationCandidateFunctor` | `LegacyFacadeCategory -> ArchitectureImpactCategory` | covariant | Legacy-Owner zu Migrationskandidaten |
| 27 | `ImpactToMigrationPlanFunctor` | `ArchitectureImpactCategory -> ArchitectureMigrationCategory` | covariant | Impact zu Migrationsplan |
| 28 | `ImpactGateBindingFunctor` | `ArchitectureImpactCategory -> ArchitectureMigrationCategory` | covariant | Impact-Gates zu Gate-Bindings |
| 29 | `MigrationWaveOrderingFunctor` | `ArchitectureMigrationCategory -> ArchitectureMigrationCategory` | covariant | Migrationsschritte ordnen |
| 30 | `MigrationOrderingCoherenceFunctor` | `ArchitectureMigrationCategory -> ArchitectureCoherenceCategory` | covariant | Migrationswellen zu Kohärenz |
| 31 | `MigrationGateCoherenceFunctor` | `ArchitectureMigrationCategory -> ArchitectureCoherenceCategory` | covariant | Gate-Bindings zu Kohärenz |
| 32 | `MigrationStepRehearsalFunctor` | `ArchitectureMigrationCategory -> ArchitectureRehearsalCategory` | covariant | Migrationsschritte zu Rehearsals |
| 33 | `MigrationGateRehearsalFunctor` | `ArchitectureMigrationCategory -> ArchitectureRehearsalCategory` | covariant | Gate-Bindings zu Rehearsal-Suites |
| 34 | `RehearsalCoverFunctor` | `ArchitectureRehearsalCategory -> ArchitectureRehearsalCategory` | covariant | lokale Rehearsals zu Readiness-Cover |
| 35 | `RehearsalGateValidationFunctor` | `ArchitectureRehearsalCategory -> ArchitectureValidationCategory` | covariant | Rehearsal-Gates zu Validierung |
| 36 | `RehearsalReadinessCoherenceFunctor` | `ArchitectureRehearsalCategory -> ArchitectureCoherenceCategory` | covariant | Readiness zu Kohärenz |
| 37 | `RehearsalActivationFunctor` | `ArchitectureRehearsalCategory -> ArchitectureActivationCategory` | covariant | Rehearsals zu Aktivierungseinheiten |
| 38 | `GateActivationFunctor` | `ArchitectureRehearsalCategory -> ArchitectureActivationCategory` | covariant | Rehearsal-Gates zu Commit-/Rollback-Gates |
| 39 | `ActivationTransactionFunctor` | `ArchitectureActivationCategory -> ArchitectureActivationCategory` | covariant | Aktivierungen zu Transaktion |
| 40 | `ActivationRollbackFunctor` | `ArchitectureActivationCategory -> ArchitectureActivationCategory` | covariant | Commit-Gate zu Rollback-Sektion |
| 41 | `ActivationValidationFunctor` | `ArchitectureActivationCategory -> ArchitectureValidationCategory` | covariant | Aktivierung zu Validierung |
| 42 | `ActivationCoherenceFunctor` | `ArchitectureActivationCategory -> ArchitectureCoherenceCategory` | covariant | Aktivierung zu Kohärenz |
| 43 | `RowRangeActivationFunctor` | `ArchitectureActivationCategory -> ActivatedRowRangeCategory` | covariant | Row-Range-Logik aktivieren |
| 44 | `CenterRowRangeCompatibilityFunctor` | `LegacyFacadeCategory -> ActivatedRowRangeCategory` | covariant | alte center-Zeilenfunktionen kompatibel halten |
| 45 | `RowRangeInputFunctor` | `ActivatedRowRangeCategory -> LocalSectionCategory` | covariant | Zeilenbereiche zurück in Input-Sektionen |
| 46 | `RowRangeValidationFunctor` | `ActivatedRowRangeCategory -> ArchitectureValidationCategory` | covariant | Row-Range-Migration validieren |
| 47 | `ArithmeticActivationFunctor` | `ArchitectureActivationCategory -> ActivatedArithmeticCategory` | covariant | Arithmetik aktivieren |
| 48 | `CenterArithmeticCompatibilityFunctor` | `LegacyFacadeCategory -> ActivatedArithmeticCategory` | covariant | alte center-Arithmetik kompatibel halten |
| 49 | `ArithmeticRowRangeGluingFunctor` | `ActivatedRowRangeCategory -> ActivatedArithmeticCategory` | covariant | Zeilenbereiche zu Arithmetik-Gluing |
| 50 | `ArithmeticValidationFunctor` | `ActivatedArithmeticCategory -> ArchitectureValidationCategory` | covariant | Arithmetik-Migration validieren |
| 51 | `ConsoleIOActivationFunctor` | `ArchitectureActivationCategory -> ActivatedConsoleIOCategory` | covariant | Console/Help/Utility aktivieren |
| 52 | `CenterConsoleIOCompatibilityFunctor` | `LegacyFacadeCategory -> ActivatedConsoleIOCategory` | covariant | alte center-Console-Fassade kompatibel halten |
| 53 | `ConsoleIOOutputRenderingFunctor` | `ActivatedConsoleIOCategory -> OutputFormatCategory` | covariant | Console-Ausgabe zu Output-Kategorie |
| 54 | `ConsoleIOValidationFunctor` | `ActivatedConsoleIOCategory -> ArchitectureValidationCategory` | covariant | Console-Migration validieren |
| 55 | `WordCompletionActivationFunctor` | `ArchitectureActivationCategory -> ActivatedWordCompletionCategory` | covariant | Word-Completion aktivieren |
| 56 | `LegacyWordCompleterCompatibilityFunctor` | `LegacyFacadeCategory -> ActivatedWordCompletionCategory` | covariant | alte WordCompleter-Fassade kompatibel halten |
| 57 | `WordCompletionPromptFunctor` | `ActivatedWordCompletionCategory -> LocalSectionCategory` | covariant | Word-Completion zu Prompt-Sektion |
| 58 | `WordCompletionValidationFunctor` | `ActivatedWordCompletionCategory -> ArchitectureValidationCategory` | covariant | Word-Completion validieren |
| 59 | `NestedCompletionActivationFunctor` | `ArchitectureActivationCategory -> ActivatedNestedCompletionCategory` | covariant | Nested-Completion aktivieren |
| 60 | `LegacyNestedCompleterCompatibilityFunctor` | `LegacyFacadeCategory -> ActivatedNestedCompletionCategory` | covariant | alte nestedAlx-Fassade kompatibel halten |
| 61 | `NestedCompletionPromptFunctor` | `ActivatedNestedCompletionCategory -> LocalSectionCategory` | covariant | Nested-Completion zu Prompt-Sektion |
| 62 | `NestedCompletionValidationFunctor` | `ActivatedNestedCompletionCategory -> ArchitectureValidationCategory` | covariant | Nested-Completion validieren |

---

# 10. Priorisierung nach mathematischer Bedeutung

## Rang 1: Semantik-Pipeline

```text
SchemaToTopologyFunctor
RawCommandPresheafFunctor
LocalDataPresheafFunctor
CanonicalParameterSheafFunctor
GluedSemanticSheafFunctor
```

Warum?

Hier entsteht Bedeutung:

```text
Schema -> Kontext -> lokale Sektion -> globale Garbe
```

Das ist der mathematische Kern.

---

## Rang 2: Universelles Gluing und Tabelle

```text
TableGenerationGluingFunctor
GeneratedColumnEndofunctorFamily
MutableTableRuntimeFunctor
ExplicitTableStateFunctor
```

Warum?

Hier wird Bedeutung materialisiert:

```text
Semantik -> Tabelle -> generierte Spalten -> expliziter Zustand
```

---

## Rang 3: Ausgabe und Parität

```text
OutputRenderingFunctorFamily
NormalizedOutputFunctor
```

Warum?

Hier wird die Architektur beobachtbar und testbar:

```text
Tabelle -> Ausgabe -> Normalform
```

---

## Rang 4: Zeilen, Arithmetik und Parallelisierungsbasis

```text
RowRangeActivationFunctor
RowRangeInputFunctor
ArithmeticRowRangeGluingFunctor
ArithmeticActivationFunctor
```

Warum?

Zeilen und Arithmetik sind natürliche Chunk-/Parallelisierungsachsen.

```text
Zeilenmenge -> Chunks -> arithmetische Struktur -> Tabelle
```

---

## Rang 5: Prompt und Completion

```text
WordCompletionActivationFunctor
WordCompletionPromptFunctor
NestedCompletionActivationFunctor
NestedCompletionPromptFunctor
```

Warum?

Sie machen `retaPrompt` topologisch sauber:

```text
Prompt-Kontext -> Completion-Kandidaten -> lokale Prompt-Sektion
```

---

## Rang 6: Legacy-Kompatibilität

```text
LegacyRuntimeFunctor
ArchitectureRuntimeFunctor
CenterRowRangeCompatibilityFunctor
CenterArithmeticCompatibilityFunctor
CenterConsoleIOCompatibilityFunctor
LegacyWordCompleterCompatibilityFunctor
LegacyNestedCompleterCompatibilityFunctor
```

Warum?

Sie halten alte Pfade funktoriell kompatibel mit der neuen Architektur.

---

## Rang 7: Metaarchitektur

```text
CategoryTheoryToContractFunctor
ContractToValidationFunctor
CoherenceMatrixFunctor
TraceBoundaryImpactFunctor
ImpactToMigrationPlanFunctor
RehearsalCoverFunctor
ActivationTransactionFunctor
```

Warum?

Sie schützen das Projekt vor Architektur-Rückfall und machen Migrationen prüfbar.

---

# 11. Nach Themen getrennt

## Topologie-Funktoren

```text
SchemaToTopologyFunctor
RowRangeInputFunctor
ArithmeticRowRangeGluingFunctor
OutputRenderingFunctorFamily
NormalizedOutputFunctor
WordCompletionPromptFunctor
NestedCompletionPromptFunctor
```

Sie transportieren oder materialisieren offene Kontexte:

```text
Kontext
Zeilenkontext
Ausgabekontext
Prompt-Kontext
Completion-Kontext
```

---

## Prägarben-Funktoren

```text
RawCommandPresheafFunctor
LocalDataPresheafFunctor
RowRangeInputFunctor
WordCompletionPromptFunctor
NestedCompletionPromptFunctor
```

Sie machen lokale Rohdaten zu lokalen Sektionen:

```text
Prompt
CLI
CSV
i18n
Zeilenausdruck
Completion-Kandidat
```

---

## Garben-Funktoren

```text
CanonicalParameterSheafFunctor
GluedSemanticSheafFunctor
GeneratedColumnEndofunctorFamily
ExplicitTableStateFunctor
```

Sie kleben oder synchronisieren globale Semantik:

```text
Parametersemantik
generierte Spalten
Tabellenzustand
globale Ausgabesektion
```

---

## Universelle-Konstruktions-Funktoren

```text
TableGenerationGluingFunctor
GeneratedColumnEndofunctorFamily
NormalizedOutputFunctor
RehearsalCoverFunctor
ActivationTransactionFunctor
ActivationRollbackFunctor
```

Sie tragen die universellen Eigenschaften:

```text
Gluing
Normalform
Endomorphismus
Readiness-Cover
Commit-Transaktion
Rollback-Sektion
```

---

## Morphismus-/Runtime-Funktoren

```text
MutableTableRuntimeFunctor
ExplicitTableStateFunctor
OutputRenderingFunctorFamily
LegacyRuntimeFunctor
ArchitectureRuntimeFunctor
RowRangeActivationFunctor
ArithmeticActivationFunctor
ConsoleIOActivationFunctor
WordCompletionActivationFunctor
NestedCompletionActivationFunctor
```

Sie setzen Morphismen in echte Runtime-Pfade um.

---

## Validierungs- und Meta-Funktoren

```text
CategoryTheoryToContractFunctor
ArchitectureMapToContractFunctor
ContractToValidationFunctor
WitnessToValidationFunctor
CoherenceMatrixFunctor
CoherenceToTraceFunctor
CoherenceToBoundaryFunctor
TraceBoundaryImpactFunctor
BoundaryImpactFunctor
ImpactGateValidationFunctor
ImpactToMigrationPlanFunctor
RehearsalGateValidationFunctor
ActivationValidationFunctor
ActivationCoherenceFunctor
```

Sie machen Architektur prüfbar.

---

# 12. Die wichtigste Erkenntnis

Die Funktoren sind die **Übersetzungsachsen** der Architektur.

Die Topologien sagen:

```text
wo etwas gilt
```

Die Prägarben sagen:

```text
welche lokalen Daten dort liegen
```

Die Garben sagen:

```text
was daraus global konsistent wird
```

Die Morphismen sagen:

```text
wie Bedeutung transportiert wird
```

Die Funktoren sagen:

```text
wie ganze Architektur-Schichten strukturerhaltend ineinander übersetzt werden
```

Und die natürlichen Transformationen sagen:

```text
warum verschiedene Pfade trotzdem dasselbe Ergebnis liefern müssen
```

Die wichtigste Funktorformel des Projekts ist:

```text
OpenRetaContextCategory
        ↓ RawCommandPresheafFunctor / LocalDataPresheafFunctor
LocalSectionCategory
        ↓ PresheafToSheafGluingTransformation
CanonicalSemanticSheafCategory
        ↓ TableGenerationGluingFunctor
TableSectionCategory
        ↓ OutputRenderingFunctorFamily
OutputFormatCategory
        ↓ NormalizedOutputFunctor
NormalizedOutput
```

In normalen Worten:

```text
Rohdaten werden lokal gesammelt.
Kontexte schränken sie ein.
Garben kleben sie zu globaler Semantik.
Universelle Konstruktionen materialisieren sie als Tabelle.
Output-Funktoren rendern sie.
Normalisierungs-Funktoren machen sie vergleichbar.
Legacy- und Validierungs-Funktoren sichern, dass alte und neue Pfade kommutieren.
```

Das ist die funktorielle Architektur des architekturveränderten `reta`.
