# Topologien im architekturveränderten `reta`

Im architekturveränderten `reta` gibt es **eine zentrale, explizite Topologie** und mehrere **darauf aufbauende topologische Schichten**.

Die wichtigste Unterscheidung ist:

**Streng im Code heißt die eigentliche Topologie: `RetaContextTopology`.**  
Sie liegt in:

```text
reta_architecture/topology.py
```

Aber architektonisch entstehen daraus mehrere „Topologien“ im weiteren Sinn:

```text
Kontext-Topologie
Parameter-Topologie
Zeilen-Topologie
Ausgabe-Topologie
Prompt-Topologie
Tabellen-Topologie
Rehearsal-/Aktivierungs-Topologie
```

---

# Priorität 1: Die zentrale Reta-Kontext-Topologie

Das ist die wichtigste Topologie im ganzen Umbau.

Dateien:

```text
reta_architecture/topology.py
reta_architecture/schema.py
i18n/words_context.py
i18n/words_matrix.py
i18n/words_runtime.py
```

Der Kern ist:

```python
RetaContextTopology
ContextSelection
ContextDimension
```

Diese Topologie beschreibt nicht einzelne Tabellenzellen, sondern **Kontexte**, in denen `reta` läuft.

Also nicht:

```text
Zeile 17, Spalte 42, Inhalt X
```

sondern eher:

```text
Sprache = deutsch
Hauptparameter = Religionen
Nebenparameter = Absicht_13_ist_Helfen
Zeilenbereich = primzahlen
Ausgabe = shell
Scope = spalten
```

Das ist die richtige Architekturentscheidung. Die Topologie liegt über dem **Bedeutungsraum von `reta`**, nicht über rohen Daten.

---

## Was ist in dieser Topologie drin?

`ContextSelection` hat acht Dimensionen:

```python
language
main_parameters
sub_parameters
row_parameters
output_modes
tag_names
combination_parameters
scopes
```

Das sind die eigentlichen offenen Koordinaten des Systems.

---

## 1.1 Sprach-Topologie

Dimension:

```python
language
```

Quelle:

```text
i18n/words_context.py -> sprachen, sprachen2
```

Sie enthält Sprachkontexte wie:

```text
de
en
vn
cn
kr
```

mit Aliasen wie:

```text
deutsch, german
english, englisch
vietnamesisch, vietnamese, tiếngviệt
chinesisch, chinese, 中國人
koreanisch, korean, 한국인
```

Funktion:

Ein Befehl, eine CSV-Datei, eine Übersetzungsdatei oder ein Prompt-Zustand kann auf einen Sprachkontext eingeschränkt werden.

Beispielhaft:

```text
U = alle Kontexte
V = nur englische Kontexte
```

Dann ist `V` eine offene Teilmenge von `U`.

Priorität: **hoch**, aber nicht die höchste. Sprache ist wichtig für Lokalisierung, aber sie trägt nicht die mathematische Hauptsemantik.

---

## 1.2 Hauptparameter-Topologie

Dimension:

```python
main_parameters
```

Quelle:

```text
i18n/words_context.py -> ParametersMain
```

Das ist eine der wichtigsten Dimensionen.

Darin liegen die großen Themenfamilien von `reta`, zum Beispiel:

```text
Religionen
Galaxie
Größenordnung
Universum
Multiversum
Wirtschaft
Menschliches
Pro_Contra
Licht
Bedeutung
Symbole
Eigenschaften_n
Eigenschaften_1/n
Inkrementieren
Operationen
Primzahlwirkung
Planet
Grundstrukturen
Herrschaft
MetaMetaPhysik_und_MetaChemie
alles
```

Diese Hauptparameter sind die großen offenen Gebiete der Reta-Semantik.

Wenn du im Prompt etwa `a1` oder Absichten, Religionen, Galaxie, Universum usw. ansprichst, dann bewegst du dich meistens zuerst in dieser Hauptparameter-Topologie.

Priorität: **sehr hoch**.  
Das ist zusammen mit den Nebenparametern die semantische Hauptachse.

---

## 1.3 Nebenparameter-Topologie

Dimension:

```python
sub_parameters
```

Quelle:

```text
i18n/words_matrix.py -> paraNdataMatrix
```

Das ist wahrscheinlich die inhaltlich reichste Topologie.

Hier liegen die konkreten Unterbegriffe, also zum Beispiel nicht nur:

```text
Religionen
```

sondern darunter konkrete Einträge, Aliase und Spaltenzuordnungen.

Die Struktur kommt aus Einträgen der Form:

```python
(
    Hauptparameter-Aliase,
    Nebenparameter-Aliase,
    Spaltenmengen
)
```

Also sinngemäß:

```text
Hauptgebiet + lokaler Begriff -> konkrete Spaltennummern
```

Beispielhaft aus dem Matrixaufbau:

```text
Religionen / religion / ...
Galaxie / galaxie / kreis / kreise / ...
Universum / kugel / kugeln / ...
Herrschaft / macht / ...
```

Diese Nebenparameter werden später in der Garbe zur kanonischen Semantik verklebt:

```text
Alias -> kanonischer Parameter -> Spaltenmenge
```

Priorität: **höchste Priorität**.  
Wenn man wissen will, was `reta` wirklich „weiß“, liegt sehr viel davon hier.

---

## 1.4 Zeilenparameter-Topologie

Dimension:

```python
row_parameters
```

Quelle:

```text
i18n/words_context.py -> zeilenParas
```

Darin liegen Zeilenauswahl- und Zahlenbereichskontexte, zum Beispiel:

```text
alles
gestern
heute
morgen
zeit
zaehlung
primzahlen
primzahlvielfache
vielfachevonzahlen
potenzenvonzahlen
oberesmaximum
hoehemaximal
sonne
mond
planet
schwarzesonne
typ
aussenerste
innenerste
aussenalle
innenalle
invertieren
```

Diese Topologie entscheidet:

```text
Welche Zeilen des Zahlen-/Tabellenraums sind sichtbar?
```

Sie hängt eng zusammen mit:

```text
reta_architecture/row_ranges.py
reta_architecture/row_filtering.py
reta_architecture/arithmetic.py
```

Priorität: **sehr hoch**.  
Gerade für Performance und Parallelisierung ist diese Topologie wichtig, weil Zeilenbereiche gut chunkbar sind.

---

## 1.5 Ausgabe-Topologie

Dimension:

```python
output_modes
```

Quelle:

```text
i18n/words_context.py -> ausgabeArt
```

Darin liegen Ausgabearten wie:

```text
shell
html
csv
markdown
bbcode
emacs
nichts
```

Diese Topologie sagt:

```text
In welchem Ausgabe-Kontext wird dieselbe semantische Tabelle gerendert?
```

Dazu kommen Ausgabeparameter aus:

```text
ausgabeParas
```

zum Beispiel:

```text
nocolor
justtext
art
onetable
breite
breiten
dontwrap
endless
endlessscreen
keinenummerierung
keineueberschriften
keineleereninhalte
spaltenreihenfolgeundnurdiese
```

Architektonisch hängt das an:

```text
reta_architecture/output_semantics.py
reta_architecture/output_syntax.py
reta_architecture/table_output.py
reta_architecture/table_wrapping.py
```

Priorität: **hoch**, aber nach Parameter- und Zeilentopologie.  
Sie verändert nicht primär die Semantik, sondern die Darstellung.

---

## 1.6 Tag-Topologie

Dimension:

```python
tag_names
```

Quelle:

```text
reta_architecture/tag_schema.py
```

Darin liegen Spaltentags aus dem Enum `ST`:

```text
sternPolygon
gleichfoermigesPolygon
keinPolygon
galaxie
universum
keinParaOdMetaP
gebrRat
```

Diese Tags klassifizieren Spalten nach geometrischen, kosmologischen oder strukturellen Rollen.

Die Datei `tag_schema.py` enthält außerdem große Mengen von Zuordnungen:

```text
Tag-Kombination -> Spaltennummern
```

Beispielhaft:

```text
{sternPolygon, galaxie, universum, keinParaOdMetaP} -> viele Spalten
{gleichfoermigesPolygon, keinParaOdMetaP} -> andere Spalten
{sternPolygon, keinParaOdMetaP, universum} -> andere Spalten
```

Das ist eine echte interne Subtopologie: Spalten werden nicht nur durch Parameter, sondern auch durch **Tag-Überdeckungen** strukturiert.

Priorität: **hoch bis sehr hoch**.  
Für generierte Spalten, Polygon-/Galaxie-/Universum-Semantik und Tabellenklassifikation ist diese Schicht zentral.

---

## 1.7 Kombinations-Topologie

Dimension:

```python
combination_parameters
```

Quelle:

```text
i18n/words_context.py -> kombiMainParas
i18n/words_matrix.py -> kombiParaNdataMatrix
i18n/words_matrix.py -> kombiParaNdataMatrix2
```

Darin liegen aktuell Hauptkombinationsbereiche wie:

```text
galaxie
universum
```

Diese Topologie ist für Kombi-CSV, Kombi-Join und Relationstabellen zuständig.

Architekturmodule:

```text
reta_architecture/combi_join.py
reta_architecture/concat_csv.py
reta_architecture/generated_columns.py
```

Funktion:

Sie beschreibt nicht einfach einzelne Parameter, sondern **Relationen zwischen Parametergebieten**.

Also eher:

```text
Galaxie × Universum
Universum × Galaxie
Motiv -> Struktur
Struktur -> Motiv
```

Priorität: **mittel bis hoch**.  
Sie ist weniger grundlegend als Haupt-/Nebenparameter, aber wichtig für die relationalen Erweiterungen.

---

## 1.8 Scope-Topologie

Dimension:

```python
scopes
```

Quelle:

```text
i18n/words_context.py -> hauptForNeben
```

Darin liegen grobe Befehlsräume:

```text
zeilen
spalten
kombination
ausgabe
h
help
debug
nichts
```

Diese Topologie entscheidet, **in welchem CLI-/Prompt-Bereich ein Ausdruck gemeint ist**.

Zum Beispiel:

```text
--zeilen
--spalten
--kombination
--ausgabe
```

oder im Prompt ein Hilfs-, Debug-, Ausgabe- oder Tabellenkontext.

Priorität: **sehr hoch für Bedienung**, aber semantisch eher organisatorisch.  
Ohne Scope weiß `reta` nicht, ob ein Wort als Zeilenfilter, Spaltenauswahl, Ausgabeart oder Kombinationsparameter gemeint ist.

---

# Priorität 2: Die offenen Mengen und Verfeinerungen

Das ist der eigentliche topologische Mechanismus.

In `ContextSelection` ist jede Dimension entweder:

```python
None
```

also uneingeschränkt, oder:

```python
frozenset({...})
```

also eingeschränkt auf bestimmte Werte.

Beispiel:

```python
ContextSelection(
    main_parameters=frozenset({"Religionen"}),
    scopes=frozenset({"spalten"})
)
```

bedeutet:

```text
offener Kontext:
  Hauptparameter = Religionen
  Scope = Spalten
  alles andere offen
```

Die wichtigste Operation ist:

```python
refine()
```

Das ist mathematisch der Durchschnitt / Meet zweier offener Mengen.

Beispiel:

```text
U = alle Religionen-Kontexte
V = alle Spalten-Kontexte

U ∩ V = Religionen im Spalten-Scope
```

Wenn eine Dimension leer wird, ist der Kontext leer:

```python
is_empty()
```

Das ist sauber: inkompatible Befehle oder Bedeutungsräume schneiden sich zu einer leeren offenen Menge.

Priorität: **maximal**.  
Das ist die eigentliche Topologie-Logik.

---

# Priorität 3: Basisöffnungen

`RetaContextTopology` erzeugt Basisöffnungen über:

```python
open_for(dimension, values)
basis_open_sets()
```

Beispiel:

```python
topology.open_for("language", ["english"])
topology.open_for("main_parameters", ["religionen"])
topology.open_for("output_modes", ["html"])
```

Eine Basisöffnung ist ein einfacher Kontext, der nur eine Dimension einschränkt.

Also:

```text
alle englischen Kontexte
alle Religions-Kontexte
alle HTML-Ausgabe-Kontexte
alle Primzahlen-Zeilen-Kontexte
```

Diese Basisöffnungen sind die Atome, aus denen größere Kontextbereiche gebaut werden.

Priorität: **sehr hoch**.  
Ohne Basisöffnungen gibt es keine klare Prägarben-/Garben-Indizierung.

---

# Priorität 4: Überdeckungen, besonders `cover_for_main`

Die Methode:

```python
cover_for_main(main_name)
```

liefert eine kleine Überdeckung für einen Hauptparameter.

Aktuell tut sie sinngemäß:

```python
[
    ContextSelection(main_parameters={canonical_main}),
    ContextSelection(scopes={"spalten"}),
]
```

Also:

```text
Ein Hauptparameter wird überdeckt durch:
1. seinen Hauptparameter-Kontext
2. den Spalten-Scope
```

Das ist mathematisch wichtig, weil daraus später lokale Sektionen entstehen können, die zur globalen Semantik verklebt werden.

Einfach gesagt:

```text
"Religionen" allein reicht nicht.
"Religionen im Spaltenraum" ist der operative Kontext.
```

Priorität: **hoch**.  
Diese Überdeckungen sind der Anschluss von Topologie an Prägarbe und Garbe.

---

# Priorität 5: Prägarben über dieser Topologie

Datei:

```text
reta_architecture/presheaves.py
```

Wichtige Klassen:

```python
LocalSection
Presheaf
FilesystemPresheaf
PromptStatePresheaf
PresheafBundle
```

Die Prägarbe sagt:

```text
Zu jedem offenen Kontext gibt es lokale Daten.
```

Lokale Daten sind zum Beispiel:

```text
CSV-Dateien
Übersetzungsdateien
Markdown-/Org-/JSONL-/JS-/TS-Assets
Prompt-Rohtext
Prompt-Tokens
```

Die Prägarbe besitzt noch nicht zwingend globale Wahrheit. Sie sammelt lokale Sektionen.

Beispiele:

```text
csv/en-religion.csv
csv/cn-kombi.csv
i18n/en/messages.po
Prompt-Eingabe "a1"
```

Jede solche lokale Datei oder Eingabe bekommt einen Kontext.

Zum Beispiel:

```text
csv/en-religion.csv
→ language = en
→ scope = csv
```

oder:

```text
Prompt-Eingabe
→ scope = prompt
```

Die wichtige Operation ist:

```python
restrict(context)
```

Damit kann man lokale Sektionen auf kleinere offene Kontexte einschränken.

Priorität: **sehr hoch**.  
Das ist die Brücke von Topologie zu konkreten Dateien und Prompt-Zuständen.

---

# Priorität 6: Garben / geklebte Semantik

Datei:

```text
reta_architecture/sheaves.py
```

Wichtige Klassen:

```python
ParameterSemanticsSheaf
GeneratedColumnsSheaf
TableOutputSheaf
HtmlReferenceSheaf
SheafBundle
```

Die Garbe ist der Schritt von:

```text
lokale Daten
```

zu:

```text
global konsistente Semantik
```

Die wichtigste Garbe ist:

```python
ParameterSemanticsSheaf
```

Sie enthält:

```text
main_alias_map
main_alias_groups
parameter_alias_groups
pair_to_columns
parameters_main
para_n_data_matrix
kombi_para_n_data_matrix
kombi_para_n_data_matrix2
global_parameter_dict
global_data_dicts
```

Funktion:

```text
Alias -> kanonischer Hauptparameter
Alias -> kanonischer Nebenparameter
kanonisches Paar -> Spaltennummern
```

Beispielhaft:

```text
"religion" -> "Religionen"
"galaxie" -> "Galaxie"
("Religionen", "Absicht...") -> {Spaltennummern}
```

Das ist die eigentliche semantische Klebung des Systems.

Priorität: **maximal nach der Kontexttopologie**.  
Die Topologie sagt, was ein Kontext ist. Die Garbe sagt, was darin semantisch gilt.

---

# Priorität 7: Universelle Gluing-Topologie

Datei:

```text
reta_architecture/universal.py
```

Wichtige Funktionen:

```python
merge_parameter_dicts()
normalize_column_buckets()
sync_generated_columns_from_tables()
sync_output_section_from_tables()
UniversalBundle.sync_tables()
```

Diese Schicht ist keine eigene Topologie im engen Sinn, sondern die **universelle Klebeoperation** über den lokalen Kontexten.

Sie beantwortet:

```text
Wie werden lokale Sektionen zu einer globalen Tabelle?
Wie werden positive/negative Spaltenauswahlen normalisiert?
Wie werden erzeugte Spalten und Ausgaben in die Garben gespiegelt?
```

Besonders wichtig:

```python
normalize_column_buckets()
```

Das entfernt widersprüchliche positive/negative Spaltenauswahlen.

Sinngemäß:

```text
Wenn eine Spalte gleichzeitig gewählt und abgewählt ist,
wird sie aus der positiven Auswahl entfernt.
```

Priorität: **sehr hoch**.  
Das ist der Mechanismus, der aus Topologie + lokalen Daten eine lauffähige globale Struktur macht.

---

# Priorität 8: Prompt-Topologie

Dateien:

```text
reta_architecture/prompt_language.py
reta_architecture/prompt_runtime.py
reta_architecture/prompt_session.py
reta_architecture/prompt_execution.py
reta_architecture/prompt_interaction.py
reta_architecture/completion_word.py
reta_architecture/completion_nested.py
```

Die Prompt-Topologie ist die Topologie der **Eingabesituationen**.

Sie enthält offene Mengen wie:

```text
aktueller Prompt-Text
Cursor-Präfix
Completion-Situation
Befehlskette
Scope des aktuellen Tokens
```

In der Kategorie-Schicht sind diese explizit benannt als:

```text
CursorPrefixOpenSet
NestedCompletionOpenSet
PromptCompletionSection
```

Beispiel:

```text
Dokument = "reta --spalten reli"
Cursor steht nach "reli"
```

Dann wird das Dokument auf ein offenes Präfix eingeschränkt:

```text
CursorPrefixOpenSet = "reli"
```

Daraus entstehen Completion-Kandidaten.

Priorität: **hoch für `retaPrompt`**, mittel für reine `reta.py`-Tabellenläufe.

---

# Priorität 9: Zeilenbereichs- und Arithmetik-Topologie

Dateien:

```text
reta_architecture/row_ranges.py
reta_architecture/row_filtering.py
reta_architecture/arithmetic.py
reta_architecture/number_theory.py
```

Diese Schicht betrifft offene Mengen über Zahlen-/Zeilenmengen.

Typische Kontexte:

```text
alle Zeilen
Primzahlen
Vielfache
Potenzen
Sonnenzahlen
Mondzahlen
Planet
heute / morgen / gestern
Ausschnitt
invertiert
```

Mathematisch ist das eine Topologie über selektierbaren Zeilenmengen.

Funktional ist sie sehr wichtig, weil sie entscheidet:

```text
Welche Zeilen der großen Tabelle werden wirklich berechnet oder angezeigt?
```

Priorität: **sehr hoch für Performance**.  
Das ist eine der besten Stellen für chunk-basierte Prozessparallelisierung.

---

# Priorität 10: Tabellen-Topologie / `TableSectionCategory`

Dateien:

```text
reta_architecture/table_runtime.py
reta_architecture/table_state.py
reta_architecture/table_preparation.py
reta_architecture/table_output.py
reta_architecture/table_wrapping.py
reta_architecture/concat_csv.py
reta_architecture/combi_join.py
```

Hier wird aus Semantik konkrete Tabelle.

Die Kategorie-Schicht nennt das:

```text
TableSectionCategory
```

Objekte:

```text
Tables
TableStateSections
TablePreparationBundle
TableOutput
```

Morphismen:

```text
prepare_output_table
filter_original_lines
readConcatCsv
render_table_output
```

Diese Topologie ist praktisch:

```text
lokale Tabellenstücke
Zeilenstücke
Spaltenauswahl
generierte Spalten
Concat-CSV
Kombi-Join
Ausgabesektion
```

Priorität: **hoch**, aber nach Kontext-, Parameter-, Zeilen- und Garbenebene.  
Sie ist der ausführende Raum.

---

# Priorität 11: Architektur-Rehearsal- und Aktivierungs-Topologie

Dateien:

```text
reta_architecture/architecture_migration.py
reta_architecture/architecture_rehearsal.py
reta_architecture/architecture_activation.py
reta_architecture/architecture_validation.py
reta_architecture/architecture_coherence.py
```

Das ist eine Meta-Topologie über dem Refactoring selbst.

Darin gibt es Begriffe wie:

```text
RehearsalOpenSetSpec
RehearsalCoverSpec
ActivationWindowSpec
ActivationTransactionSpec
```

Bedeutung:

```text
Eine Migrationswelle ist ein offener Bereich.
Ein Rehearsal prüft lokale Schritte.
Ein Cover klebt diese lokalen Schritte zu globaler Readiness.
Ein ActivationWindow ist ein Aktivierungsfenster.
```

Das ist nicht die Runtime-Topologie von `reta`, sondern die **Topologie des Architekturumbaus**.

Priorität: **mittel für Anwender**, hoch für Refactoring-Sicherheit.

---

# Priorität 12: Generierte semantische Topologie-Art

Datei:

```text
reta_architecture/generated_columns.py
```

Dort gibt es eine Funktion:

```python
geist_emotion_energie_materie_topologie()
```

und Begriffe wie:

```text
Energie-Art
Denkart
Gefühlsart
Materie-Art
Topologie-Art
```

Das ist wichtig:  
Hier bedeutet „Topologie“ eher **inhaltliche Klassifikation**, nicht die Architektur-Topologie aus `topology.py`.

Diese Schicht erzeugt zusätzliche semantische Spalten. Sie ist also Teil der generierten Inhaltslogik.

Priorität: **inhaltlich wichtig**, aber architektonisch nicht die zentrale Topologie.

---

# Die Rangfolge in kurz

Meine Priorisierung wäre:

1. **`RetaContextTopology` / `ContextSelection`**  
   Der Kern. Ohne diese Topologie gibt es keine saubere Architektur.

2. **Hauptparameter- und Nebenparameter-Topologie**  
   Der wichtigste Bedeutungsraum von `reta`.

3. **`ParameterSemanticsSheaf`**  
   Klebt Aliase, Parameter und Spaltennummern zu globaler Semantik.

4. **Zeilenparameter-/RowRange-Topologie**  
   Entscheidet, welche Zahlen-/Tabellenzeilen sichtbar und berechenbar sind.

5. **Scope-Topologie**  
   Entscheidet, ob ein Ausdruck als Zeile, Spalte, Ausgabe, Kombination, Hilfe usw. gelesen wird.

6. **Tag-Topologie**  
   Klassifiziert Spalten über Polygon-, Galaxie-, Universum- und gebrochen-rationale Tags.

7. **Ausgabe-Topologie**  
   Shell, HTML, CSV, Markdown, BBCode, Emacs, nichts.

8. **Kombinations-Topologie**  
   Galaxie/Universum/Kombi-Relationen und Join-Semantik.

9. **Prägarben lokaler Daten**  
   CSVs, Übersetzungen, Assets, Prompt-Zustand als lokale Sektionen.

10. **Tabellen-Topologie / `TableSectionCategory`**  
    Globale Tabelle, Tabellenzustände, Vorbereitung, Ausgabe, Join, Concat.

11. **Prompt-/Completion-Topologie**  
    Cursor-Präfixe, Completion-Situationen, Prompt-Befehlsräume.

12. **Architektur-Rehearsal-/Activation-Topologie**  
    Metaebene für sichere Migrationen und Refactoring-Readiness.

13. **Generierte Inhalts-Topologie-Art**  
    Semantische Klassifikation in generierten Spalten, nicht der Architekturkern.

---

# Was ist „alles darin“?

In der Topologie steckt insgesamt:

```text
Sprachen
Sprach-Aliase
Übersetzungsdomänen
Hauptparameter
Nebenparameter
Parameter-Aliase
Zeilenparameter
Ausgabeparameter
Ausgabearten
Kombinationsparameter
Scopes
Spaltentags
CSV-Sektionen
i18n-Sektionen
Asset-Sektionen
Prompt-Sektionen
kanonische Parameterpaare
Spaltennummern-Zuordnungen
generierte Spaltenmetadaten
HTML-Referenzen
Tabellenzustände
Ausgabesektionen
Completion-Kontexte
Migrations-Open-Sets
Aktivierungsfenster
```

Das ist viel. Aber es ist inzwischen nicht mehr wild verteilt, sondern in Schichten geordnet.

---

# Die beste mentale Erklärung

Vor dem Architekturumbau war `reta` ungefähr so:

```text
Ein großer Monolith aus Wörtern, CSVs, Parametern, Tabellenlogik und Promptlogik.
```

Nach dem Umbau ist es eher so:

```text
1. Es gibt offene Kontexte.
2. Lokale Dateien und Prompt-Zustände liegen über diesen Kontexten.
3. Diese lokalen Sektionen werden zu Garben verklebt.
4. Morphismen übersetzen zwischen Aliasen, Parametern, Zeilen, Tabellen und Ausgaben.
5. Funktoren beschreiben die großen Architekturpfade.
6. Natürliche Transformationen sichern, dass Legacy-Pfade und neue Architekturpfade dasselbe Ergebnis liefern.
```

Oder noch kürzer:

```text
Topologie = Wo gilt etwas?
Prägarbe = Welche lokalen Daten gibt es dort?
Garbe = Wie kleben diese Daten konsistent zusammen?
Morphismus = Wie komme ich von einem Kontext/Objekt zum anderen?
Funktor = Wie wird eine ganze Schicht in eine andere übersetzt?
Natürliche Transformation = Warum bleiben alte und neue Pfade äquivalent?
```

---

# Meine klare Bewertung

Die wichtigste Topologie ist nicht die Tabellenmatrix selbst, sondern die **Kontexttopologie**:

```text
Sprache × Hauptparameter × Nebenparameter × Zeilenparameter × Ausgabe × Tags × Kombination × Scope
```

Das ist die tragende Idee.

Die zweite große Erkenntnis ist:  
Die Architektur behandelt `reta` nicht mehr als „Programm, das Tabellen ausgibt“, sondern als System von **lokalen Bedeutungssektionen**, die über offenen Kontexten liegen und zu einer globalen Tabelle verklebt werden.

Das ist mathematisch sauber und für weitere Optimierungen gut, weil du jetzt genau unterscheiden kannst:

```text
Was ist lokal?
Was ist global?
Was darf parallel berechnet werden?
Was muss seriell geklebt werden?
Was ist nur Ausgabe?
Was ist echte Semantik?
```

Die Topologie ist damit nicht nur Deko. Sie ist die Ordnungsschicht, die verhindert, dass Prompt, CSV, i18n, Parameter, Tabellen und Ausgabe wieder zu einem Monolithen zusammenfallen.
