# Universelle Eigenschaften zu den Topologien und Morphismen im architekturveränderten `reta`

Die **universellen Eigenschaften** im architekturveränderten `reta` sind der Kern der mathematischen Architektur. Sie beantworten nicht nur:

```text
Welche Topologien und Morphismen gibt es?
```

sondern:

```text
Warum ergibt sich aus lokalen Teilen eindeutig eine globale Bedeutung, Tabelle oder Ausgabe?
```

Oder einfacher:

```text
Topologie = Wo gilt etwas?
Morphismus = Wie wird etwas übertragen?
Universelle Eigenschaft = Warum ist das Ergebnis der kanonische / eindeutig richtige Klebepunkt?
```

Wichtig: Im Code sind diese universellen Eigenschaften nicht als formale mathematische Beweise implementiert, sondern als **Architekturgesetze, Gluing-Knoten, Normalisierungen, Natürlichkeitsbedingungen und kommutierende Diagramme**.

Im aktuellen Stand gibt es dafür besonders:

```text
reta_architecture/universal.py
reta_architecture/category_theory.py
reta_architecture/presheaves.py
reta_architecture/sheaves.py
reta_architecture/table_generation.py
reta_architecture/program_workflow.py
reta_architecture/architecture_contracts.py
reta_architecture/architecture_validation.py
reta_architecture/architecture_coherence.py
```

Die explizite zentrale Kategorie heißt:

```text
UniversalConstructionCategory
```

Darin liegen die wichtigsten universellen Runtime-Konstruktionen:

```text
merge_parameter_dicts
normalize_column_buckets
sync_tables
```

Dazu kommen die Natürlichkeitsbedingungen, also die kommutierenden Verträglichkeitsgesetze zwischen alten und neuen Pfaden, Prompt und Semantik, Prägarbe und Garbe, Tabelle und Ausgabe.

---

# A. Universelle Eigenschaften der Topologien

## Priorität 1: Kontext-Topologie

Topologie:

```text
RetaContextTopology
ContextSelection
```

Datei:

```text
reta_architecture/topology.py
```

Die zentrale universelle Eigenschaft ist:

```text
Ein Reta-Kontext ist der kleinste eindeutig bestimmte offene Bereich,
in dem eine Kombination aus Sprache, Parametern, Zeilen, Ausgabe, Tags,
Kombinationen und Scope gilt.
```

Die Kontext-Topologie ist im Kern eine Produkt-/Schnitt-Topologie über Dimensionen:

```text
language
main_parameters
sub_parameters
row_parameters
output_modes
tag_names
combination_parameters
scopes
```

Ein Kontext wie:

```text
Sprache = deutsch
Hauptparameter = Religionen
Scope = spalten
```

ist eine offene Menge im Bedeutungsraum von `reta`.

Die wichtigste universelle Eigenschaft dieser Topologie ist der **größte gemeinsame Unterkontext**:

```text
refine(U, V) = U ∩ V
```

Das heißt:

```text
Wenn ein Ausdruck in Kontext U gilt
und zusätzlich in Kontext V gelesen werden soll,
dann ist U ∩ V der eindeutig beste engere Kontext.
```

Beispiel:

```text
U = alle Religionen-Kontexte
V = alle Spalten-Kontexte

U ∩ V = Religionen im Spalten-Scope
```

Priorität: **maximal**.

Warum?  
Weil alle späteren Bedeutungen davon abhängen, dass `reta` sauber weiß, **wo** ein Wort oder Parameter gilt.

---

## Priorität 2: Basisöffnungen

Morphismen:

```text
open_for
basis_open_sets
```

Universelle Eigenschaft:

```text
open_for(dimension, values) erzeugt die kleinste Basisöffnung,
die genau diese Dimensionswerte zulässt und alles andere offen lässt.
```

Beispiel:

```python
topology.open_for("main_parameters", ["Religionen"])
```

bedeutet:

```text
kleinster offener Kontext, in dem Hauptparameter = Religionen gilt
```

Oder:

```python
topology.open_for("output_modes", ["html"])
```

bedeutet:

```text
kleinster offener Kontext für HTML-Ausgabe
```

Das ist wichtig, weil daraus größere Kontexte gebaut werden.

Priorität: **sehr hoch**.

---

## Priorität 3: Leerer Kontext als Widerspruchsobjekt

Topologische Situation:

```text
ContextSelection.is_empty()
```

Universelle Eigenschaft:

```text
Wenn zwei Einschränkungen unvereinbar sind, entsteht der leere Kontext.
```

Beispiel:

```text
Sprache = deutsch
∩ Sprache = englisch
```

kann leer werden, wenn der Kontext nur einen exklusiven Sprachwert zulässt.

Der leere Kontext ist hier das universelle Signal:

```text
Diese Bedeutungen lassen sich nicht gemeinsam lesen.
```

Priorität: **sehr hoch**.

Warum?  
Ohne leeren Kontext würden widersprüchliche Befehle als scheinbar gültige Mischzustände weiterlaufen.

---

## Priorität 4: Überdeckung eines Hauptparameters

Morphism:

```text
cover_for_main
```

Universelle Eigenschaft:

```text
Ein Hauptparameter wird durch lokale offene Kontexte überdeckt,
die zusammen den operativen Bedeutungsraum dieses Parameters ergeben.
```

Beispiel:

```text
Religionen
```

wird sinngemäß überdeckt durch:

```text
Hauptparameter-Kontext Religionen
+
Spalten-Scope
```

Das ist wichtig für Prägarben und Garben:

```text
Lokale Sektionen über dieser Überdeckung können später global geklebt werden.
```

Priorität: **sehr hoch**.

---

## Priorität 5: Hauptparameter-Topologie

Dimension:

```text
main_parameters
```

Universelle Eigenschaft:

```text
Alle Aliase eines Hauptparameters besitzen einen kanonischen Repräsentanten.
```

Beispiel:

```text
religion
Religion
Religionen
religions
```

werden auf dieselbe Bedeutung gebracht:

```text
Religionen
```

Die Hauptparameter-Topologie hat daher eine Quotienten-Eigenschaft:

```text
Viele lokale Namen werden zu einem kanonischen Bedeutungsobjekt identifiziert.
```

Das ist fast wie:

```text
Rohwörter / Aliasrelation = kanonische Hauptparameter
```

Priorität: **maximal bis sehr hoch**.

---

## Priorität 6: Nebenparameter-Topologie

Dimension:

```text
sub_parameters
```

Universelle Eigenschaft:

```text
Ein lokaler Nebenparameter wird nur zusammen mit seinem Hauptparameter eindeutig.
```

Beispiel:

```text
a1
absicht 1
```

ist nicht bloß ein freies Wort, sondern muss in einem Hauptkontext gelesen werden:

```text
Religionen + a1
Galaxie + a1
Universum + a1
```

Die universelle Eigenschaft ist:

```text
Das kanonische Parameterpaar ist der eindeutig richtige Klebepunkt
aus Hauptparameter und Nebenparameter.
```

Also:

```text
(Hauptparameter-Alias, Nebenparameter-Alias)
        ↓
kanonisches Parameterpaar
        ↓
Spaltenmenge
```

Priorität: **maximal**.

Das ist eine der wichtigsten Semantik-Eigenschaften im ganzen Projekt.

---

## Priorität 7: Zeilenparameter-Topologie

Dimension:

```text
row_parameters
```

Dateien:

```text
reta_architecture/row_ranges.py
reta_architecture/row_filtering.py
reta_architecture/arithmetic.py
```

Universelle Eigenschaft:

```text
Ein Zeilenausdruck besitzt eine kanonische expandierte Zeilenmenge.
```

Beispiele:

```text
1-5
```

wird zu:

```text
{1, 2, 3, 4, 5}
```

oder:

```text
primzahlen
```

wird zu:

```text
Menge der Primzahl-Zeilen
```

oder:

```text
vielfachevonzahlen
```

wird zu einer passenden Vielfachenmenge.

Die Eigenschaft lautet:

```text
Alle syntaktischen Zeilenbeschreibungen werden auf genau die Zeilenmenge
abgebildet, die später Tabelle und Parallelisierung verwenden dürfen.
```

Priorität: **sehr hoch**.

Für Performance ist diese Schicht besonders wichtig, weil Zeilenmengen gut in Chunks zerlegbar sind.

---

## Priorität 8: Ausgabe-Topologie

Dimension:

```text
output_modes
```

Dateien:

```text
reta_architecture/output_semantics.py
reta_architecture/output_syntax.py
reta_architecture/table_output.py
```

Universelle Eigenschaft:

```text
Verschiedene konkrete Renderer erzeugen Ausgabeformen,
die nach Normalisierung dieselbe semantische Ausgabe repräsentieren müssen.
```

Beispiel:

```text
Shell-Ausgabe
HTML-Ausgabe
CSV-Ausgabe
Markdown-Ausgabe
```

sind äußerlich verschieden, aber sie sollen auf eine gemeinsame Aussage zurückführbar sein:

```text
NormalizedOutput
```

Das ist eine Quotienten-Eigenschaft:

```text
RenderedOutput / erlaubte Formatunterschiede = NormalizedOutput
```

Priorität: **hoch**.

Warum?  
Weil sichtbare Ausgabe sonst schwer testbar und schwer vergleichbar wäre.

---

## Priorität 9: Tag-Topologie

Dimension:

```text
tag_names
```

Datei:

```text
reta_architecture/tag_schema.py
```

Universelle Eigenschaft:

```text
Tags klassifizieren Spalten so, dass jede Tag-Kombination
eine eindeutig bestimmte Spaltensektion auswählt.
```

Beispiele für Tags:

```text
sternPolygon
gleichfoermigesPolygon
keinPolygon
galaxie
universum
keinParaOdMetaP
gebrRat
```

Eine Tag-Kombination wirkt wie eine Faser über der Spaltenmenge:

```text
Tag-Kombination -> Spaltennummern
```

Die universelle Eigenschaft ist:

```text
Die Tag-Auswahl ist der kanonische Klassifikator für Spaltenrollen.
```

Priorität: **hoch bis sehr hoch**.

---

## Priorität 10: Kombinations-Topologie

Dimension:

```text
combination_parameters
```

Dateien:

```text
reta_architecture/combi_join.py
reta_architecture/concat_csv.py
reta_architecture/generated_columns.py
```

Universelle Eigenschaft:

```text
Lokale Kombi-Relationen werden in eine globale relationale Tabellensektion geklebt.
```

Beispielhafte Bereiche:

```text
Galaxie
Universum
Kombi-CSV
Relationstabellen
Join-Semantik
```

Die Eigenschaft ist pushout-/join-artig:

```text
lokale Kombi-Sektion A
lokale Kombi-Sektion B
gemeinsamer Tabellenkontext
        ↓
globale Kombi-Tabelle
```

Priorität: **mittel bis hoch**.

---

## Priorität 11: Scope-Topologie

Dimension:

```text
scopes
```

Beispiele:

```text
zeilen
spalten
kombination
ausgabe
help
debug
nichts
```

Universelle Eigenschaft:

```text
Ein roher Ausdruck wird durch den Scope in seine richtige Lesart gezwungen.
```

Beispiel:

```text
primzahlen
```

kann als Zeilenparameter Sinn ergeben.

Aber:

```text
html
```

gehört eher in den Ausgabe-Scope.

Die Scope-Topologie ist also ein Klassifikator:

```text
rohes Token -> operativer Befehlsraum
```

Priorität: **sehr hoch für Bedienung**, semantisch etwas unter Haupt-/Nebenparameter.

---

## Priorität 12: Prägarben-Topologie

Datei:

```text
reta_architecture/presheaves.py
```

Universelle Eigenschaft:

```text
Lokale Daten können entlang kleinerer Kontexte eingeschränkt werden.
```

Das ist die klassische Prägarben-Eigenschaft:

```text
U größer als V
Sektion über U
        ↓ restrict
Sektion über V
```

Beispiele:

```text
alle CSV-Sektionen
-> nur englische CSV-Sektionen

alle Prompt-Sektionen
-> aktueller Spaltenkontext

alle i18n-Sektionen
-> konkrete Sprache
```

Wichtig:

```text
Prägarben liefern lokale Daten, aber noch keine garantierte globale Wahrheit.
```

Priorität: **sehr hoch**.

---

## Priorität 13: Garben-Topologie

Datei:

```text
reta_architecture/sheaves.py
```

Universelle Eigenschaft:

```text
Kompatible lokale Sektionen kleben eindeutig zu einer globalen Sektion.
```

Das ist die eigentliche Garben-Eigenschaft.

Sinngemäß:

```text
lokale Sektion über U
lokale Sektion über V
beide stimmen auf U ∩ V überein
        ↓
eindeutige globale Sektion über U ∪ V
```

Im Projekt bedeutet das:

```text
lokale CSV-Daten
lokale i18n-Daten
lokale Parameter-Aliase
lokale Prompt-Zustände
        ↓
global verwendbare Parametersemantik
```

Priorität: **maximal**.

---

## Priorität 14: Prompt-Topologie

Dateien:

```text
reta_architecture/prompt_language.py
reta_architecture/prompt_execution.py
reta_architecture/prompt_interaction.py
reta_architecture/completion_word.py
reta_architecture/completion_nested.py
```

Universelle Eigenschaft:

```text
Der aktuelle Cursor-Kontext ist die kleinste Prompt-Umgebung,
die für Completion oder Befehlsausführung ausreicht.
```

Beispiel:

```text
reta --spalten reli|
```

Der relevante offene Kontext ist nicht der ganze Prompt, sondern der Keim am Cursor:

```text
CursorPrefixOpenSet = "reli"
```

Für Nested Completion gilt zusätzlich:

```text
PromptDocument -> NestedCompletionOpenSet
```

Also:

```text
Wo steht der Cursor?
Nach welchem Parameter?
Nach welchem "="?
In welchem Scope?
```

Priorität: **hoch für retaPrompt**.

---

## Priorität 15: Tabellen-Topologie

Dateien:

```text
reta_architecture/table_runtime.py
reta_architecture/table_state.py
reta_architecture/table_generation.py
reta_architecture/table_preparation.py
reta_architecture/table_output.py
```

Universelle Eigenschaft:

```text
Die globale Tabelle ist das kanonische materialisierte Objekt
aus Parametersemantik, Zeilenauswahl, Spaltenauswahl, CSV-Sektionen,
generierten Spalten und Ausgabezustand.
```

Oder kurz:

```text
Semantik + Zeilen + Spalten + CSV + Generatoren
        ↓
globale Tabelle
```

Diese Tabelle ist der zentrale Klebepunkt der Runtime.

Priorität: **sehr hoch**.

---

## Priorität 16: Rehearsal-/Activation-Topologie

Dateien:

```text
reta_architecture/architecture_rehearsal.py
reta_architecture/architecture_activation.py
```

Universelle Eigenschaft:

```text
Lokale Migrationsmoves kleben nur dann zu globaler Readiness,
wenn alle Gates bestehen und Rollback-Sektionen vorhanden sind.
```

Das ist die Meta-Garbe des Refactorings.

Beispiel:

```text
lokaler Move A
lokaler Move B
lokaler Move C
Gate-Suiten
Rollback-Sektionen
        ↓
ActivationTransaction
```

Priorität: **mittel für Runtime**, **hoch für Refactoring-Sicherheit**.

---

# B. Universelle Eigenschaften der Morphismen

Jetzt getrennt von den Topologien: Hier geht es um die universellen Eigenschaften der konkreten Übergänge.

---

## Priorität 1: `refine`

Morphism:

```text
ContextSelection -> ContextSelection
```

Universelle Eigenschaft:

```text
refine ist der größte gemeinsame Unterkontext zweier offener Kontexte.
```

Mathematisch:

```text
refine(U, V) = U ∩ V
```

Bedeutung:

```text
Jeder Kontext, der sowohl in U als auch in V liegt,
faktorisiert eindeutig durch U ∩ V.
```

Praktisch:

```text
Erst Sprache einschränken und dann Scope einschränken
ergibt denselben Kontext wie erst Scope und dann Sprache.
```

Priorität: **maximal**.

---

## Priorität 2: `open_for`

Morphism:

```text
RetaContextTopology -> ContextSelection
```

Universelle Eigenschaft:

```text
open_for erzeugt die kleinste Basisöffnung für eine gegebene Dimension.
```

Beispiel:

```text
open_for("language", ["de"])
```

ist der kleinste offene Kontext für Deutsch.

Jede stärkere Einschränkung mit Deutsch muss durch diese Basisöffnung laufen.

Priorität: **sehr hoch**.

---

## Priorität 3: `cover_for_main`

Morphism:

```text
RetaContextTopology -> ContextCover
```

Universelle Eigenschaft:

```text
cover_for_main erzeugt eine kanonische Überdeckung eines Hauptparameters.
```

Diese Überdeckung ist die Voraussetzung dafür, dass lokale Parameterdaten später geklebt werden können.

Priorität: **sehr hoch**.

---

## Priorität 4: `restrict`

Morphism:

```text
FilesystemPresheaf -> LocalSection
```

Universelle Eigenschaft:

```text
restrict ist der kanonische Restriktionsmorphismus der Prägarbe.
```

Wenn gilt:

```text
V ⊆ U
```

dann gibt es:

```text
Sektion(U) -> Sektion(V)
```

Praktisch:

```text
Alle Daten
-> nur Daten im aktuellen Kontext
```

Priorität: **maximal für Prägarben**.

---

## Priorität 5: `add_section`

Morphism:

```text
LocalSection -> FilesystemPresheaf
```

Universelle Eigenschaft:

```text
Eine lokale Sektion wird an genau dem offenen Kontext registriert,
über dem sie gültig ist.
```

Das ist die Einbettung lokaler Rohdaten in die Prägarbe.

Priorität: **hoch**.

---

## Priorität 6: `update_prompt_state`

Morphism:

```text
PromptStatePresheaf -> LocalSection
```

Universelle Eigenschaft:

```text
Der aktuelle Prompt-Zustand wird als lokale Sektion über dem aktuellen Prompt-Kontext dargestellt.
```

Praktisch:

```text
Nutzer tippt "a1"
        ↓
lokale Prompt-Sektion
```

Priorität: **hoch für retaPrompt**.

---

## Priorität 7: `canonicalize_pair`

Morphism:

```text
Alias-Paar -> kanonisches Parameterpaar
```

Universelle Eigenschaft:

```text
canonicalize_pair ist der Quotientenmorphismus von Alias-Semantik
zu kanonischer Parametersemantik.
```

Viele Alias-Pfade werden identifiziert:

```text
("religion", "a1")
("Religion", "absicht 1")
("Religionen", "Absicht_1")
```

führen zum selben kanonischen Paar.

Die universelle Eigenschaft:

```text
Jede semantische Auswertung, die Aliasgleichheit respektiert,
faktorisiert eindeutig über das kanonische Parameterpaar.
```

Einfacher:

```text
Wenn zwei Wörter dasselbe bedeuten, muss der weitere Tabellenbau
denselben Pfad nehmen.
```

Priorität: **maximal**.

---

## Priorität 8: `column_numbers_for_pair`

Morphism:

```text
CanonicalParameterPair -> ColumnNumberSet
```

Universelle Eigenschaft:

```text
Die Spaltenmenge ist die kanonische Darstellung eines Parameterpaars in der Tabelle.
```

Also:

```text
Bedeutung -> Spalten
```

Jede spätere Tabellenoperation, die diesen Parameter meint, muss durch diese Spaltenmenge laufen.

Priorität: **maximal**.

---

## Priorität 9: `merge_parameter_dicts`

Datei:

```text
reta_architecture/universal.py
```

Morphism:

```text
ParameterDictionaryDiagram -> ParameterSemanticsSheaf
```

Universelle Eigenschaft im Code ausdrücklich:

```text
pushout-artiges Zusammenkleben lokaler Parameterdaten
```

Das bedeutet:

```text
lokales Dictionary A
lokales Dictionary B
gemeinsame Alias-/Parameterstruktur
        ↓
gemergtes globales Parameterdictionary
```

Die Eigenschaft lautet:

```text
Das Ergebnis ist der kanonische gemeinsame Zielpunkt,
in dem beide lokalen Dictionary-Sektionen kompatibel zusammengeführt sind.
```

Priorität: **maximal**.

Das ist einer der explizitesten universellen Knoten im Projekt.

---

## Priorität 10: `normalize_column_buckets`

Datei:

```text
reta_architecture/universal.py
```

Morphism:

```text
ColumnBucketDiagram -> NormalizedColumnBuckets
```

Universelle Eigenschaft:

```text
normalize_column_buckets erzeugt die kanonische Normalform
aus positiven und negativen Spaltenauswahlen.
```

Beispiel:

```text
positive Auswahl: {1, 2, 3, 4}
negative Auswahl: {3}
```

wird zu:

```text
{1, 2, 4}
```

Die universelle Eigenschaft:

```text
Jeder weitere Tabellenpfad, der widerspruchsfreie Spalten braucht,
faktorisiert über diese Normalform.
```

Oder einfach:

```text
Erst widersprüchliche Auswahl bereinigen,
dann Tabelle bauen.
```

Priorität: **maximal bis sehr hoch**.

---

## Priorität 11: `sync_tables`

Datei:

```text
reta_architecture/universal.py
```

Morphism:

```text
Tables -> SheafBundle
```

Universelle Eigenschaft:

```text
sync_tables ist der kanonische Synchronisationsmorphismus
von materialisierter Tabelle zurück in die Garbenmetadaten.
```

Er synchronisiert unter anderem:

```text
generierte Spalten
Ausgabe-Sektionen
Tabellenmetadaten
HTML-/Output-Referenzen
```

Die Eigenschaft:

```text
Runtime-Tabelle und Garbenbeschreibung bleiben dasselbe Objekt
aus zwei Perspektiven.
```

Priorität: **sehr hoch**.

---

## Priorität 12: `PresheafToSheafGluingTransformation`

Natürliche Transformation:

```text
LocalDataPresheafFunctor => GluedSemanticSheafFunctor
```

Universelle Eigenschaft:

```text
Lokale Sektionen über einer Überdeckung kleben zu derselben globalen Semantik,
unabhängig von der kompatiblen Reihenfolge der Restriktionen.
```

Das ist die eigentliche Sheafification-Eigenschaft des Projekts.

Schema:

```text
lokale CSV-Sektion
lokale i18n-Sektion
lokale Prompt-Sektion
        ↓ restrict
kompatible lokale Sektionen
        ↓ glue
globale Semantik
```

Priorität: **maximal**.

---

## Priorität 13: `RawToCanonicalParameterTransformation`

Natürliche Transformation:

```text
RawCommandPresheafFunctor => CanonicalParameterSheafFunctor
```

Universelle Eigenschaft:

```text
Kontext zuerst einschränken und dann kanonisieren
ergibt dieselbe Semantik wie zuerst kanonisieren
und danach auf den kleineren Kontext einschränken.
```

Als Diagramm:

```text
RawCommand
   | restrict
   v
RestrictedRawCommand
   | canonicalize
   v
CanonicalMeaning
```

ist äquivalent zu:

```text
RawCommand
   | canonicalize
   v
CanonicalMeaning
   | restrict
   v
RestrictedCanonicalMeaning
```

Priorität: **maximal**.

Das ist extrem wichtig für Prompt und CLI.

---

## Priorität 14: `TableGenerationGluingTransformation`

Natürliche Transformation:

```text
CanonicalParameterSheafFunctor => TableGenerationGluingFunctor
```

Universelle Eigenschaft:

```text
Äquivalente Alias-/Kontextpfade erzeugen dieselbe globale Tabellensektion.
```

Das heißt:

```text
Wenn zwei Nutzereingaben semantisch gleich sind,
müssen sie nach Kanonisierung, Spaltenauswahl und Tabellenbau
dieselbe globale Tabelle ergeben.
```

Priorität: **maximal**.

---

## Priorität 15: `expand_row_range`

Morphism:

```text
RowRangeExpression -> RowIndexSet
```

Universelle Eigenschaft:

```text
Ein Zeilenbereichsausdruck besitzt eine kanonische freie Zeilenmenge.
```

Beispiel:

```text
1-5
```

wird zu:

```text
{1, 2, 3, 4, 5}
```

Jede spätere Operation auf diesen Zeilen muss nicht mehr die Syntax interpretieren, sondern kann über die Menge laufen.

Priorität: **sehr hoch**.

---

## Priorität 16: `validate_row_range`

Morphism:

```text
RowRangeExpression -> RowRangeSyntax
```

Universelle Eigenschaft:

```text
Nur gültige Zeilenbereichsausdrücke dürfen in die Zeilenmengen-Konstruktion eintreten.
```

Das ist eine Gate-Eigenschaft:

```text
RawText
  -> gültiger RowRange-Ausdruck
  -> Zeilenmenge
```

Priorität: **hoch**.

---

## Priorität 17: `filter_original_lines`

Morphism:

```text
ParameterSection -> RowSet
```

Universelle Eigenschaft:

```text
Die sichtbaren Tabellenzeilen sind die kanonische Projektion
der Zeilenparameter auf die Originaltabellenzeilen.
```

Also:

```text
Zeilenparameter
        ↓
konkrete RowSet-Auswahl
```

Priorität: **sehr hoch**.

---

## Priorität 18: `factor_pairs`

Morphism:

```text
ArithmeticExpression -> FactorPairSet
```

Universelle Eigenschaft:

```text
Eine Zahl wird auf ihre kanonischen Faktorpaare abgebildet.
```

Beispiel:

```text
12
```

wird zu:

```text
(1,12), (2,6), (3,4)
```

Das ist eine arithmetische Darstellungs-Eigenschaft.

Priorität: **hoch**.

---

## Priorität 19: `prime_factorize`

Morphism:

```text
ArithmeticExpression -> PrimeFactorSection
```

Universelle Eigenschaft:

```text
Eine Zahl besitzt eine kanonische Primfaktorzerlegung.
```

Beispiel:

```text
12 -> 2 * 2 * 3
```

Das ist mathematisch stärker als viele andere Eigenschaften, weil die Primfaktorzerlegung eindeutig ist.

Priorität: **hoch bis sehr hoch**.

---

## Priorität 20: `glue_divisor_range`

Morphism:

```text
RowIndexSet -> DivisorSection
```

Universelle Eigenschaft:

```text
Zeilenbereich und Arithmetik werden zu einer gemeinsamen Divisor-Sektion geklebt.
```

Also:

```text
Zeilenmenge
        ↓
Teiler-/Faktorstruktur über diesen Zeilen
```

Priorität: **hoch**.

---

## Priorität 21: `prepare_output_table`

Morphism:

```text
Tables -> Tables
```

Universelle Eigenschaft:

```text
Die vorbereitete Tabelle ist die kanonische ausgabefähige Form
der globalen Tabelle.
```

Dabei werden unter anderem verarbeitet:

```text
Zeilenauswahl
Spaltenauswahl
Zellenvorbereitung
Wrapping
Breiten
Ausgabezustand
```

Nach der Prozessparallelisierung gilt zusätzlich:

```text
Lokale Chunk-Vorbereitungen kleben deterministisch
in Originalreihenfolge zur selben vorbereiteten Tabelle.
```

Das ist eine wichtige neue universelle Eigenschaft:

```text
parallel vorbereiten + seriell kleben
=
seriell vorbereiten
```

Priorität: **sehr hoch**.

---

## Priorität 22: `readConcatCsv`

Morphism:

```text
LocalCsvSection -> Tables
```

Universelle Eigenschaft:

```text
Lokale CSV-Sektionen werden in die globale Tabellensektion eingeklebt.
```

Also:

```text
CSV-Datei
        ↓
lokale Sektion
        ↓
globale Tabelle
```

Priorität: **hoch**.

---

## Priorität 23: `render_table_output` / `render`

Morphism:

```text
Tables -> RenderedOutput
```

Universelle Eigenschaft:

```text
Eine ausgabefähige Tabelle besitzt eine kanonische Darstellung
im gewählten Ausgabeformat.
```

Beispiel:

```text
Tabelle + shell    -> Shell-Ausgabe
Tabelle + html     -> HTML-Ausgabe
Tabelle + csv      -> CSV-Ausgabe
Tabelle + markdown -> Markdown-Ausgabe
```

Priorität: **hoch**.

---

## Priorität 24: `normalize_for_parity`

Morphism:

```text
RenderedOutput -> NormalizedOutput
```

Universelle Eigenschaft:

```text
Unterschiedliche syntaktische Ausgabeformen werden auf eine vergleichbare Normalform gebracht.
```

Das ist für Tests und Legacy-Parität entscheidend.

Also:

```text
alte Ausgabe
neue Ausgabe
        ↓ normalize
vergleichbare semantische Ausgabe
```

Priorität: **hoch**.

---

## Priorität 25: `split_command_words`

Morphism:

```text
CommandText -> CommandWordSection
```

Universelle Eigenschaft:

```text
Ein Befehlstext wird in die eindeutig weiterverarbeitbare Wortsektion zerlegt.
```

Beispiel:

```text
reta --spalten religion --zeilen 1-10
```

wird zu einer strukturierten Wortfolge.

Priorität: **hoch für Prompt/CLI**.

---

## Priorität 26: `expand_shorthand`

Morphism:

```text
PromptShorthand -> ExpandedCommand
```

Universelle Eigenschaft:

```text
Ein Kurzkommando besitzt eine kanonische Langform.
```

Beispiel:

```text
a1
```

wird zu einem ausführbaren `reta`-Befehl expandiert.

Die Eigenschaft ist:

```text
Alle weiteren Semantikpfade laufen über die expandierte Langform,
nicht über die rohe Abkürzung.
```

Priorität: **hoch für retaPrompt**.

---

## Priorität 27: `restrict_to_cursor_prefix`

Morphism:

```text
Document -> CursorPrefixOpenSet
```

Universelle Eigenschaft:

```text
Für einfache Completion reicht der kleinste Cursor-Präfix-Kontext.
```

Beispiel:

```text
reta --spalten reli|
```

wird zu:

```text
reli
```

Alles, was nicht für diesen Präfix relevant ist, wird ausgeblendet.

Priorität: **mittel bis hoch**.

---

## Priorität 28: `select_nested_open_set`

Morphism:

```text
PromptDocument -> NestedCompletionOpenSet
```

Universelle Eigenschaft:

```text
Der Prompt-Zustand besitzt eine kanonische aktuelle Completion-Situation.
```

Beispiele:

```text
reta --spalten=
reta --zeilen=
reta --ausgabe=
```

sind verschiedene offene Completion-Kontexte.

Priorität: **hoch für retaPrompt**.

---

## Priorität 29: `glue_equality_value_options`

Morphism:

```text
NestedOptionSection -> NestedOptionSection
```

Universelle Eigenschaft:

```text
Werteoptionen aus Runtime-Vokabular, i18n und Parameterlisten
kleben zu einer lokalen Completion-Wertsektion.
```

Also:

```text
Zeilenwerte
Spaltenwerte
Ausgabewerte
Kombiwerte
        ↓
Completion-Optionen
```

Priorität: **mittel bis hoch**.

---

## Priorität 30: `yield_nested_candidates`

Morphism:

```text
NestedOptionSection -> NestedCompletionCandidateSection
```

Universelle Eigenschaft:

```text
Aus einer lokalen Optionensektion entsteht die kanonische Kandidatensektion
für prompt_toolkit.
```

Priorität: **mittel bis hoch**.

---

## Priorität 31: Generierte-Spalten-Endomorphismen

Morphismen:

```text
concat_love_polygon
concat_modallogik
create_spalte_gestirn
```

Universelle Eigenschaft:

```text
Eine generierte Spalte ist ein Endomorphismus der globalen Tabelle.
```

Also:

```text
Tables -> Tables
```

Wichtig ist:

```text
Die Tabelle bleibt Tabelle,
aber mit zusätzlicher abgeleiteter Struktur.
```

Dazu passt die Natürlichkeitsbedingung:

```text
GeneratedColumnsSheafSyncTransformation
```

Sie sagt:

```text
Direkter Zugriff auf generierte Spalten
und Synchronisierung über die Garbe
müssen dieselbe GeneratedColumnSection ergeben.
```

Priorität: **mittel bis hoch**.

---

## Priorität 32: Legacy-Fassaden-Morphismen

Morphismen:

```text
bootstrap_program
tableHandling_reexport
prepare_delegation
concat_delegation
delegate_center_wrappers
delegate_center_arithmetic
delegate_center_console_io
delegate_word_completer
delegate_nested_completer
```

Universelle Eigenschaft:

```text
Alter Pfad und neuer Architekturpfad müssen kommutieren.
```

Beispiel:

```text
center.py -> alte Funktion -> Ergebnis
```

muss beobachtbar dasselbe sein wie:

```text
reta_architecture -> neuer Morphismus -> Ergebnis
```

Das ist die Eigenschaft der:

```text
LegacyToArchitectureTransformation
```

Sie sagt:

```text
Jeder repräsentative alte Aufrufpfad und der entsprechende neue Architekturpfad
müssen beobachtbar gleiche Ausgabe liefern.
```

Priorität: **hoch für Kompatibilität**, aber nicht mehr Kernsemantik.

---

## Priorität 33: Architektur-Validierungs-Morphismen

Morphismen:

```text
bootstrap_architecture_contracts
validate_contract_references
bootstrap_architecture_validation
validate
```

Universelle Eigenschaft:

```text
Architekturbegriffe, Kapseln, Kategorien, Funktoren,
natürliche Transformationen und Witnesses werden zu einem gemeinsamen
prüfbaren Validierungsobjekt geklebt.
```

Das bedeutet:

```text
Kategorie-Theorie
Architekturkarte
Verträge
Witnesses
Paketstruktur
        ↓
ArchitectureValidationBundle
```

Priorität: **mittel für Runtime**, **hoch für Refactoring-Sicherheit**.

---

## Priorität 34: Migrations-, Rehearsal- und Activation-Morphismen

Morphismen:

```text
plan
order
bind_gate
preserve_invariant
rehearse_step
rehearse_gate
cover_wave
activate_move
activate_gate
rollback
commit_transaction
```

Universelle Eigenschaft:

```text
Ein Refactoring darf nur dann global aktiviert werden,
wenn lokale Moves, Gates, Invarianten und Rollbacks zu einer
konsistenten Transaktion kleben.
```

Besonders wichtig:

```text
cover_wave
commit_transaction
```

`cover_wave`:

```text
lokale Rehearsal-Moves -> globale Readiness
```

`commit_transaction`:

```text
lokale Aktivierungen -> geschützte globale Transaktion
```

Priorität: **mittel für Programmverhalten**, **hoch für Projektstabilität**.

---

# C. Die wichtigsten universellen Eigenschaften als Rangliste

## 1. Kontextschnitt / Meet

```text
refine(U, V) = U ∩ V
```

Warum wichtig:

```text
Alle Bedeutungen hängen davon ab, dass Kontexte sauber verfeinert werden.
```

---

## 2. Alias-Quotient

```text
Rohalias -> kanonischer Parameter
```

Warum wichtig:

```text
Viele Schreibweisen müssen dieselbe Bedeutung ergeben.
```

---

## 3. Kanonisches Parameterpaar

```text
Hauptparameter + Nebenparameter -> CanonicalParameterPair
```

Warum wichtig:

```text
Das ist der zentrale semantische Klebepunkt.
```

---

## 4. Spaltenrepräsentation

```text
CanonicalParameterPair -> ColumnNumberSet
```

Warum wichtig:

```text
Bedeutung muss in konkrete Tabellenstruktur übersetzt werden.
```

---

## 5. Prägarben-Restriktion

```text
Sektion(U) -> Sektion(V), wenn V ⊆ U
```

Warum wichtig:

```text
Lokale Daten müssen in kleineren Kontexten lesbar sein.
```

---

## 6. Garben-Gluing

```text
kompatible lokale Sektionen -> eindeutige globale Sektion
```

Warum wichtig:

```text
Aus CSV, i18n, Prompt und Parametern entsteht globale Semantik.
```

---

## 7. Pushout-artiger Parameter-Merge

```text
merge_parameter_dicts
```

Warum wichtig:

```text
Lokale Parameter-Dictionaries werden zu globaler Parametersemantik.
```

---

## 8. Spalten-Normalform

```text
normalize_column_buckets
```

Warum wichtig:

```text
Positive und negative Spaltenauswahlen müssen widerspruchsfrei werden.
```

---

## 9. Zeilenmengen-Expansion

```text
RowRangeExpression -> RowIndexSet
```

Warum wichtig:

```text
Zeilensyntax wird zur ausführbaren Zeilenmenge.
```

---

## 10. Tabellen-Gluing

```text
Semantik + Zeilen + Spalten + CSV + generierte Spalten -> Tables
```

Warum wichtig:

```text
Die Tabelle ist das globale Runtime-Objekt.
```

---

## 11. Parallel-Chunk-Gluing

```text
parallel vorbereitete Chunks -> deterministische Gesamttabelle
```

Warum wichtig:

```text
Die neue PyPy3-Prozessparallelisierung darf die Ausgabe nicht verändern.
```

---

## 12. Output-Normalisierung

```text
RenderedOutput -> NormalizedOutput
```

Warum wichtig:

```text
Alte und neue Ausgabe müssen vergleichbar bleiben.
```

---

## 13. Legacy-Kommutativität

```text
alter Pfad = neuer Architekturpfad
```

Warum wichtig:

```text
Der Umbau darf bestehende Aufrufe nicht brechen.
```

---

## 14. Rehearsal-/Activation-Gluing

```text
lokale Moves + Gates + Rollbacks -> globale Transaktion
```

Warum wichtig:

```text
Weitere Umbauten bleiben kontrollierbar.
```

---

# D. Nach Topologie getrennte Zuordnung

| Topologie | Universelle Eigenschaft | Wichtigste Morphismen |
|---|---|---|
| Kontext-Topologie | Schnitt/Meet offener Kontexte | `refine`, `open_for`, `cover_for_main` |
| Sprach-Topologie | Sprachsektionen werden lokal eingeschränkt und global geklebt | `restrict`, `merge_parameter_dicts` |
| Hauptparameter-Topologie | Aliase haben kanonischen Repräsentanten | `resolve_main_alias`, `canonicalize_pair` |
| Nebenparameter-Topologie | Nebenparameter wird erst im Hauptkontext eindeutig | `resolve_parameter_alias`, `canonicalize_pair` |
| Zeilen-Topologie | Zeilenausdruck hat kanonische Zeilenmenge | `expand_row_range`, `filter_original_lines` |
| Ausgabe-Topologie | Renderer-Ausgaben besitzen gemeinsame Normalform | `render`, `normalize_for_parity` |
| Tag-Topologie | Tag-Kombination klassifiziert Spaltensektion | `sync_from_tables`, generierte Spaltenmorphismen |
| Kombi-Topologie | lokale Kombi-Relationen kleben zur globalen Relationstabelle | `readConcatCsv`, `prepare_output_table` |
| Scope-Topologie | roher Ausdruck wird in richtige Lesart klassifiziert | `split_command_words`, `expand_shorthand` |
| Prägarben-Topologie | lokale Sektionen können eingeschränkt werden | `add_section`, `restrict`, `update_prompt_state` |
| Garben-Topologie | kompatible lokale Sektionen kleben eindeutig global | `merge_parameter_dicts`, `sync_tables` |
| Prompt-Topologie | Cursor-Kontext ist kleinster relevanter Completion-Kontext | `restrict_to_cursor_prefix`, `select_nested_open_set` |
| Tabellen-Topologie | globale Tabelle ist materialisierter Klebepunkt | `prepare_output_table`, `readConcatCsv`, `render_table_output` |
| Rehearsal-/Activation-Topologie | lokale Moves kleben zu Readiness und Transaktion | `cover_wave`, `commit_transaction` |

---

# E. Nach Morphismusgruppen getrennte Zuordnung

| Morphismusgruppe | Universelle Eigenschaft | Priorität |
|---|---|---:|
| Kontextmorphismen | kleinste Öffnung, Schnitt, Überdeckung | 1 |
| Alias-/Parametermorphismen | Quotient roher Aliase zu kanonischer Semantik | 2 |
| Prägarbenmorphismen | Restriktion lokaler Sektionen | 3 |
| Garben-/Gluing-Morphismen | eindeutige globale Sektion aus kompatiblen lokalen Daten | 4 |
| Spaltenmorphismen | kanonische Spaltenmenge aus Bedeutung | 5 |
| Zeilenmorphismen | kanonische Zeilenmenge aus Zeilensyntax | 6 |
| Tabellenmorphismen | globale Tabelle als materialisiertes Objekt | 7 |
| Parallelisierungsmorphismen | Chunk-Ergebnis klebt deterministisch zur seriellen Ausgabe | 8 |
| Ausgabemorphismen | Renderer-Ausgaben normalisieren auf gemeinsame Semantik | 9 |
| Promptmorphismen | Rohtext/Kurztext wird kanonischer Befehl | 10 |
| Completionmorphismen | Cursor-/Nested-Kontext erzeugt Kandidatensektion | 11 |
| Arithmetikmorphismen | Zahlen werden kanonisch faktorisiert/strukturiert | 12 |
| Legacy-Morphismen | alter und neuer Pfad kommutieren | 13 |
| Metaarchitektur-Morphismen | Verträge, Gates, Witnesses kleben zu Validierung | 14 |
| Activation-Morphismen | lokale Aktivierungen kleben zu Transaktion mit Rollback | 15 |

---

# F. Die zentrale Architekturformel

Die universellen Eigenschaften des architekturveränderten `reta` lassen sich so zusammenfassen:

```text
Lokale Rohdaten
  CSV, i18n, Prompt, Aliase, Zeilenausdrücke
        ↓
Kontext-Restriktion
        ↓
kanonische Semantik
        ↓
Garbe / Gluing
        ↓
Spalten- und Zeilenmengen
        ↓
globale Tabelle
        ↓
generierte Spalten und Ausgabevorbereitung
        ↓
gerenderte Ausgabe
        ↓
Normalisierung / Parität
```

Mathematisch:

```text
Die Prägarben lokaler Daten werden über Kontextrestriktionen eingeschränkt.
Kompatible lokale Sektionen werden durch Garben-Gluing zu globaler Semantik.
Diese globale Semantik wird über universelle Tabellenkonstruktionen materialisiert.
Renderer und Legacy-Pfade werden durch natürliche Transformationen
auf dieselbe beobachtbare Ausgabe gezwungen.
```

Praktisch:

```text
Egal ob der Nutzer über Prompt, CLI, Alias, alte Legacy-Fassade,
CSV-Sektion oder neue Architektur kommt:
Wenn die Bedeutung dieselbe ist, muss die resultierende Tabelle
und die normalisierte Ausgabe dieselbe sein.
```

Das ist die wichtigste universelle Eigenschaft des ganzen Umbaus.
