# Prägarben und Garben im architekturveränderten `reta`

Die wichtigste Unterscheidung ist:

```text
Prägarbe = lokale Daten über offenen Kontexten
Garbe    = eindeutig geklebte globale Semantik aus kompatiblen lokalen Daten
```

Oder noch einfacher:

```text
Topologie             = Wo gilt etwas?
Morphismus            = Wie wird etwas übertragen?
Prägarbe              = Welche lokalen Daten liegen dort?
Garbe                 = Was ist die global konsistente Bedeutung?
Universelle Eigenschaft = Warum ist dieses globale Ergebnis der kanonische Klebepunkt?
```

Im architekturveränderten `reta` sind die wichtigsten Dateien dafür:

```text
reta_architecture/presheaves.py
reta_architecture/sheaves.py
reta_architecture/universal.py
reta_architecture/topology.py
reta_architecture/morphisms.py
reta_architecture/category_theory.py
reta_architecture/table_generation.py
reta_architecture/program_workflow.py
```

Wichtig: Nicht jede Schicht hat im Code eine Klasse mit dem Wort `Presheaf` oder `Sheaf`. Es gibt zwei Ebenen:

1. **Explizit implementierte Prägarben/Garben**
2. **Architekturweite prägarben-/garbenartige Schichten**, die über Funktoren, Morphismen, Bundles und universelle Gluing-Operationen realisiert sind

---

# 1. Die zentrale Prägarben-Idee

Eine Prägarbe ordnet jedem offenen Kontext lokale Daten zu.

Im `reta`-Fall ist ein offener Kontext zum Beispiel:

```text
Sprache = deutsch
Hauptparameter = Religionen
Scope = spalten
Zeilenbereich = primzahlen
Ausgabe = shell
```

Eine Prägarbe sagt dann:

```text
Welche lokalen Daten gibt es in diesem Kontext?
```

Zum Beispiel:

```text
deutsche Wörter
CSV-Dateien
Prompt-Eingabe
Zeilenbereichsausdruck
Spaltenparameter
Ausgabeparameter
Completion-Zustand
```

Die wichtigste Prägarbenoperation ist:

```text
restrict
```

Also:

```text
Daten über großem Kontext
        ↓
Daten über kleinerem Kontext
```

Beispiel:

```text
alle CSV-Daten
        ↓ restrict
nur deutsche Religionen-CSV-Daten
```

Eine Prägarbe muss noch nicht garantieren, dass lokale Daten eindeutig global zusammenkleben. Sie sammelt und beschränkt erst einmal lokale Sektionen.

---

# 2. Die zentrale Garben-Idee

Eine Garbe geht einen Schritt weiter.

Sie sagt:

```text
Wenn lokale Daten auf Überlappungen zusammenpassen,
dann gibt es eine eindeutige globale Sektion.
```

Im `reta`-Fall bedeutet das:

```text
lokale Aliaslisten
lokale CSV-Daten
lokale i18n-Daten
lokale Prompt-Daten
lokale Parameterdaten
        ↓
global konsistente Parametersemantik
```

Die wichtigste Garbenoperation ist:

```text
glue
```

Im Code tritt diese Idee besonders durch diese Operationen auf:

```text
merge_parameter_dicts
normalize_column_buckets
sync_tables
canonicalize_pair
column_numbers_for_pair
```

---

# 3. Höchste Priorität: `FilesystemPresheaf`

Datei:

```text
reta_architecture/presheaves.py
```

Explizite Klasse:

```python
FilesystemPresheaf
```

## Was ist darin?

Diese Prägarbe sammelt lokale Dateisektionen.

Typische lokale Sektionen:

```text
CSV-Dateien
i18n-Dateien
Markdown-Dateien
Org-Dateien
JSONL-Dateien
JS-/TS-Dateien
HTML-/Asset-Dateien
Dokumentationsdateien
Konfigurationsfragmente
```

Jede Datei wird nicht einfach als „Datei“ gesehen, sondern als lokale Sektion über einem Kontext.

Beispiel:

```text
csv/en-religion.csv
```

kann sinngemäß liegen über:

```text
language = en
main_parameters = Religionen
scope = csv / spalten
```

## Zugehörige Topologien

```text
Kontext-Topologie
Sprach-Topologie
Hauptparameter-Topologie
Nebenparameter-Topologie
Kombinations-Topologie
Ausgabe-Topologie
Prägarben-Topologie
```

## Zugehörige Morphismen

```text
add_section
restrict
open_for
refine
cover_for_main
```

## Universelle Eigenschaft

```text
Eine lokale Datei wird genau in dem kleinsten offenen Kontext registriert,
in dem sie gültig ist.
```

Und:

```text
Wenn ein kleinerer Kontext betrachtet wird,
liefert restrict die passende lokale Einschränkung.
```

Beispiel:

```text
alle Dateien
        ↓ restrict(language = de)
deutsche lokale Dateisektionen
```

## Priorität

```text
Priorität: maximal
```

Warum?

Weil `reta` sehr stark aus lokalen Datenquellen lebt: CSV, i18n, Wörter, Kombinationsdaten, Dokumentationsdaten. Ohne diese Prägarbe gibt es keine saubere lokale Datenbasis.

---

# 4. Höchste Priorität: `PromptStatePresheaf`

Datei:

```text
reta_architecture/presheaves.py
```

Explizite Klasse:

```python
PromptStatePresheaf
```

## Was ist darin?

Diese Prägarbe hält lokale Prompt-Zustände.

Typische lokale Sektionen:

```text
aktueller Prompt-Text
Tokenliste
Cursorposition
aktuelles Wort
aktueller Scope
Kurzkommando
Befehlskette
Completion-Kontext
Prompt-Modus
```

Beispiel:

```text
a1
```

oder:

```text
reta --spalten reli
```

ist nicht einfach Text, sondern eine lokale Sektion über der Prompt-Topologie.

## Zugehörige Topologien

```text
Prompt-Topologie
Scope-Topologie
Kontext-Topologie
Completion-Topologie
Hauptparameter-Topologie
Nebenparameter-Topologie
Ausgabe-Topologie
Zeilen-Topologie
```

## Zugehörige Morphismen

```text
update_prompt_state
split
split_prompt_text
split_command_words
expand_shorthand
restrict_to_cursor_prefix
select_nested_open_set
yield_nested_candidates
```

## Universelle Eigenschaft

```text
Der aktuelle Prompt-Zustand wird als kleinste lokale Sektion dargestellt,
die für Ausführung oder Completion ausreicht.
```

Beispiel:

```text
reta --spalten reli|
```

Der relevante Kontext ist nicht der ganze Verlauf, sondern der Keim am Cursor:

```text
CursorPrefixOpenSet = "reli"
```

Für Kurzbefehle gilt:

```text
a1
        ↓ expand_shorthand
kanonischer ausführbarer reta-Befehl
```

## Priorität

```text
Priorität: maximal für retaPrompt
Priorität: hoch für reta.py
```

Warum?

Weil `retaPrompt` ohne diese Prägarbe nicht sauber zwischen Rohtext, Kurzform, Scope, Completion und ausführbarem Befehl unterscheiden kann.

---

# 5. Höchste Priorität: `PresheafBundle`

Datei:

```text
reta_architecture/presheaves.py
```

Explizite Klasse:

```python
PresheafBundle
```

## Was ist darin?

Das Bundle fasst die lokalen Prägarben zusammen.

Typisch enthält es:

```text
FilesystemPresheaf
PromptStatePresheaf
lokale Sektionen
Kontextzuordnungen
Restriktionslogik
```

## Zugehörige Topologien

```text
Kontext-Topologie
Prägarben-Topologie
Prompt-Topologie
Datei-/Filesystem-Topologie
```

## Zugehörige Morphismen

```text
add_section
restrict
update_prompt_state
```

## Universelle Eigenschaft

```text
Alle lokalen Datenquellen werden in einem gemeinsamen Prägarbenobjekt gesammelt,
ohne sie schon global gleichzusetzen.
```

Das ist wichtig:

```text
Prägarbe sammelt lokal.
Garbe klebt global.
```

## Priorität

```text
Priorität: sehr hoch
```

---

# 6. Höchste Priorität: `ParameterSemanticsSheaf`

Datei:

```text
reta_architecture/sheaves.py
```

Explizite Klasse:

```python
ParameterSemanticsSheaf
```

Das ist die wichtigste Garbe im ganzen System.

## Was ist darin?

Typische Inhalte:

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

Diese Garbe enthält die globale Semantik von:

```text
Hauptparametern
Nebenparametern
Aliasen
kanonischen Parameterpaaren
Spaltennummern
Kombinationsparametern
Datenmatrizen
```

## Was macht sie?

Sie klebt lokale Alias- und Parameterdaten zu globaler Bedeutung.

Beispiel:

```text
religion
Religion
Religionen
religions
```

werden geklebt zu:

```text
Religionen
```

Und:

```text
("religion", "a1")
```

wird zu:

```text
kanonisches Parameterpaar
```

und dann zu:

```text
Spaltennummernmenge
```

## Zugehörige Topologien

```text
Hauptparameter-Topologie
Nebenparameter-Topologie
Sprach-Topologie
Scope-Topologie
Kombinations-Topologie
Tag-Topologie
Garben-Topologie
```

## Zugehörige Morphismen

```text
resolve_main_alias
resolve_parameter_alias
canonicalize_pair
column_numbers_for_pair
merge_parameter_dicts
restrict
refine
cover_for_main
```

## Universelle Eigenschaft

```text
Kompatible lokale Alias- und Parametersektionen kleben eindeutig
zu globaler kanonischer Parametersemantik.
```

Anders gesagt:

```text
Alle Schreibweisen, die dieselbe Bedeutung haben,
faktorisieren eindeutig über dieselbe kanonische Semantik.
```

Das ist die Quotienten-/Gluing-Eigenschaft:

```text
Rohaliasse / Aliasgleichheit = kanonische Parametersemantik
```

## Priorität

```text
Priorität: absolut maximal
```

Warum?

Wenn diese Garbe falsch ist, ist `reta` semantisch falsch. Dann können Tabellen, Prompt, Ausgabe und Zeilen korrekt laufen und trotzdem das Falsche bedeuten.

---

# 7. Höchste Priorität: `SheafBundle`

Datei:

```text
reta_architecture/sheaves.py
```

Explizite Klasse:

```python
SheafBundle
```

## Was ist darin?

Das Bundle fasst die global geklebten Garben zusammen.

Typische Bestandteile:

```text
ParameterSemanticsSheaf
GeneratedColumnsSheaf
TableOutputSheaf
HtmlReferenceSheaf
globale semantische Sektionen
globale Ausgabesektionen
globale Metadaten
```

## Zugehörige Topologien

```text
Garben-Topologie
Parameter-Topologie
Tabellen-Topologie
Ausgabe-Topologie
Tag-Topologie
HTML-/Referenz-Topologie
```

## Zugehörige Morphismen

```text
merge_parameter_dicts
sync_tables
sync_from_tables
normalize_column_buckets
canonicalize_pair
column_numbers_for_pair
```

## Universelle Eigenschaft

```text
Alle global geklebten semantischen Schichten werden in einem gemeinsamen
Garbenobjekt zusammengeführt.
```

Das ist der globale Zustand der Bedeutung, nicht bloß der Runtime.

## Priorität

```text
Priorität: sehr hoch
```

---

# 8. Sehr hohe Priorität: `GeneratedColumnsSheaf`

Datei:

```text
reta_architecture/sheaves.py
```

Explizite Klasse:

```python
GeneratedColumnsSheaf
```

## Was ist darin?

Diese Garbe hält Informationen über generierte Spalten.

Dazu gehören sinngemäß:

```text
generierte Spaltennamen
generierte Spaltennummern
Tag-Zuordnungen
abgeleitete Tabellenstrukturen
Metadaten zu erzeugten Spalten
```

Typische generierte Spalten entstehen über Morphismen wie:

```text
concat_love_polygon
concat_modallogik
create_spalte_gestirn
```

## Zugehörige Topologien

```text
Tag-Topologie
Tabellen-Topologie
Kombinations-Topologie
Ausgabe-Topologie
Garben-Topologie
```

## Zugehörige Morphismen

```text
sync_from_tables
sync_tables
concat_love_polygon
concat_modallogik
create_spalte_gestirn
```

## Universelle Eigenschaft

```text
Generierte Spalten sind Tabellen-Endomorphismen,
deren Metadaten eindeutig in die Garbe zurückgespiegelt werden.
```

Also:

```text
Tables
  ↓ generierte Spalten
Tables
  ↓ sync_from_tables
GeneratedColumnsSheaf
```

Die Natürlichkeitsbedingung lautet:

```text
Direkter Zugriff auf generierte Spalten
und Zugriff über die synchronisierte Garbe
müssen dieselbe GeneratedColumnSection ergeben.
```

## Priorität

```text
Priorität: sehr hoch
```

Warum?

Weil generierte Spalten sonst leicht zu verstecktem globalem Zustand werden. Die Garbe macht sie explizit und synchronisierbar.

---

# 9. Sehr hohe Priorität: `TableOutputSheaf`

Datei:

```text
reta_architecture/sheaves.py
```

Explizite Klasse:

```python
TableOutputSheaf
```

## Was ist darin?

Diese Garbe hält globale Ausgabesektionen.

Dazu gehören sinngemäß:

```text
Ausgabezustand
gerenderte Sektionen
Shell-Ausgabe
HTML-Ausgabe
CSV-Ausgabe
Markdown-Ausgabe
BBCode-Ausgabe
Emacs-Ausgabe
Normalisierungsinformationen
```

## Zugehörige Topologien

```text
Ausgabe-Topologie
Tabellen-Topologie
Garben-Topologie
Kontext-Topologie
```

## Zugehörige Morphismen

```text
output_mode_for_tables
apply_output_mode
render
render_table_output
normalize_for_parity
sync_tables
```

## Universelle Eigenschaft

```text
Eine globale Tabelle besitzt im gewählten Ausgabe-Kontext
eine kanonische Ausgabesektion.
```

Und für Parität:

```text
Verschiedene konkrete Ausgaben müssen nach Normalisierung
dieselbe semantische Ausgabe ergeben.
```

Also:

```text
Shell-Ausgabe
HTML-Ausgabe
CSV-Ausgabe
Markdown-Ausgabe
        ↓ normalize_for_parity
NormalizedOutput
```

## Priorität

```text
Priorität: sehr hoch
```

Warum?

Weil die Ausgabe das beobachtbare Ergebnis ist. Selbst wenn die interne Semantik stimmt, muss die sichtbare Ausgabe stabil bleiben.

---

# 10. Hohe Priorität: `HtmlReferenceSheaf`

Datei:

```text
reta_architecture/sheaves.py
```

Explizite Klasse:

```python
HtmlReferenceSheaf
```

## Was ist darin?

Diese Garbe hält HTML- und Referenzinformationen.

Typische Inhalte:

```text
HTML-Referenzen
Anker
Links
Ausgabe-Metadaten
Referenzen zwischen Tabellen und HTML-Ausgabe
```

## Zugehörige Topologien

```text
Ausgabe-Topologie
HTML-Topologie
Referenz-Topologie
Tabellen-Topologie
```

## Zugehörige Morphismen

```text
render
render_table_output
sync_tables
apply_output_mode
```

## Universelle Eigenschaft

```text
HTML-Referenzen sind die eindeutig synchronisierte Referenzschicht
zwischen Tabelle und HTML-Ausgabe.
```

Also:

```text
Tabelle
        ↓ HTML-Rendering
HTML-Ausgabe
        ↓ Referenz-Synchronisierung
HtmlReferenceSheaf
```

## Priorität

```text
Priorität: hoch
```

Wichtig für HTML-Ausgabe, aber weniger zentral als Parametersemantik und globale Tabelle.

---

# 11. Operative Prägarbe: Rohkommando-Prägarbe

Diese Schicht ist stärker architektonisch/funktoriell als als einzelne Klasse sichtbar.

Kategorie-/Funktoridee:

```text
RawCommandPresheafFunctor
```

## Was ist darin?

```text
rohe CLI-Argumente
roher Prompt-Text
Kurzbefehle
Tokenfolgen
Befehlsketten
uninterpretierte Parameterwörter
```

Beispiel:

```text
a1
```

oder:

```text
reta --spalten religion --zeilen 1-10
```

## Zugehörige Topologien

```text
Prompt-Topologie
Scope-Topologie
Kontext-Topologie
Hauptparameter-Topologie
Nebenparameter-Topologie
Zeilen-Topologie
Ausgabe-Topologie
```

## Zugehörige Morphismen

```text
split
split_prompt_text
split_command_words
expand_shorthand
restrict
refine
canonicalize_pair
```

## Universelle Eigenschaft

```text
Rohkommandos werden als lokale Sektionen behandelt,
bevor sie kanonisiert werden.
```

Die wichtige natürliche Transformation ist:

```text
RawToCanonicalParameterTransformation
```

Sie sagt:

```text
Erst Kontext einschränken und dann kanonisieren
=
erst kanonisieren und dann Kontext einschränken
```

## Priorität

```text
Priorität: sehr hoch
```

Warum?

Weil Nutzer nicht kanonische interne Objekte eingeben, sondern Rohtext.

---

# 12. Operative Garbe: kanonische Parametergarbe

Diese Schicht entspricht im Kern der `ParameterSemanticsSheaf`.

Funktoridee:

```text
CanonicalParameterSheafFunctor
```

## Was ist darin?

```text
kanonische Hauptparameter
kanonische Nebenparameter
kanonische Parameterpaare
Spaltennummern
Aliasauflösungen
globale Bedeutungssektionen
```

## Zugehörige Morphismen

```text
resolve_main_alias
resolve_parameter_alias
canonicalize_pair
column_numbers_for_pair
merge_parameter_dicts
```

## Universelle Eigenschaft

```text
Alle äquivalenten Rohparameterpfade führen zur selben kanonischen Semantik.
```

Beispiel:

```text
religion + a1
Religionen + absicht 1
religions + intention 1
```

müssen, wenn sie semantisch gleich sind, denselben Zielpunkt haben:

```text
CanonicalParameterPair
```

## Priorität

```text
Priorität: maximal
```

---

# 13. Operative Prägarbe: Zeilenbereichs-Prägarbe

Diese ist nicht primär als Klasse mit `Presheaf` im Namen zentral, aber architektonisch eindeutig vorhanden.

Dateien:

```text
reta_architecture/row_ranges.py
reta_architecture/row_filtering.py
reta_architecture/arithmetic.py
```

## Was ist darin?

Lokale Zeilenausdrücke:

```text
1-10
1,2,3
primzahlen
vielfache
potenzen
heute
morgen
gestern
sonne
mond
planet
invertieren
```

## Zugehörige Topologien

```text
Zeilen-Topologie
Arithmetik-Topologie
Kontext-Topologie
Tabellen-Topologie
```

## Zugehörige Morphismen

```text
validate_row_range
expand_row_range
parse_generator_literal
filter_original_lines
glue_divisor_range
factor_pairs
prime_factorize
```

## Universelle Eigenschaft als Prägarbe

```text
Ein roher Zeilenausdruck ist eine lokale Sektion
über dem Zeilenkontext.
```

Beispiel:

```text
"1-5"
```

ist zunächst lokale Syntax.

## Zugehörige Garben-Eigenschaft

```text
Der Zeilenausdruck klebt zu einer kanonischen Zeilenmenge.
```

Also:

```text
"1-5"
        ↓ expand_row_range
{1, 2, 3, 4, 5}
```

Für Parallelisierung wichtig:

```text
Die kanonische Zeilenmenge kann in Chunks zerlegt werden,
ohne ihre globale Bedeutung zu verändern.
```

## Priorität

```text
Priorität: sehr hoch
```

---

# 14. Operative Garbe: Zeilenmengen-Garbe

## Was ist darin?

```text
kanonische Zeilenmengen
gefilterte Originalzeilen
arithmetische Zeilenstrukturen
Primzahlbereiche
Vielfachenbereiche
Faktor-/Teilersektionen
```

## Zugehörige Morphismen

```text
expand_row_range
filter_original_lines
glue_divisor_range
prime_factorize
factor_pairs
```

## Universelle Eigenschaft

```text
Alle gültigen Zeilenausdrücke werden auf genau eine verwendbare Zeilenmenge gebracht.
```

Das ist wichtig, weil danach nicht mehr mit roher Syntax gerechnet werden muss.

```text
Syntax
        ↓
RowIndexSet
        ↓
Tabellenfilter
```

## Priorität

```text
Priorität: sehr hoch
```

---

# 15. Operative Prägarbe: Spaltenauswahl-Prägarbe

## Was ist darin?

Lokale Spaltenauswahlen:

```text
positive Spaltenauswahl
negative Spaltenauswahl
Parameter-basierte Spaltenauswahl
Tag-basierte Spaltenauswahl
Kombi-Spaltenauswahl
Ausgabe-Spaltenreihenfolge
```

Beispiele:

```text
--spalten religion
--spalten galaxie
--spalten -x
--ausgabe spaltenreihenfolgeundnurdiese
```

## Zugehörige Topologien

```text
Hauptparameter-Topologie
Nebenparameter-Topologie
Tag-Topologie
Kombinations-Topologie
Tabellen-Topologie
Ausgabe-Topologie
```

## Zugehörige Morphismen

```text
canonicalize_pair
column_numbers_for_pair
normalize_column_buckets
sync_tables
```

## Universelle Eigenschaft als Prägarbe

```text
Lokale Spaltenwünsche werden gesammelt,
auch wenn sie noch widersprüchlich sein können.
```

Beispiel:

```text
positive Auswahl: {1, 2, 3}
negative Auswahl: {3}
```

## Zugehörige Garben-Eigenschaft

```text
normalize_column_buckets erzeugt die kanonische widerspruchsfreie Spaltensektion.
```

Also:

```text
{1, 2, 3} positiv
{3} negativ
        ↓
{1, 2}
```

## Priorität

```text
Priorität: sehr hoch
```

---

# 16. Operative Garbe: normalisierte Spalten-Garbe

## Was ist darin?

```text
effektive Spaltenmengen
normalisierte positive/negative Auswahl
kanonische Spaltenreihenfolge
Spaltennummern aus Parameterpaaren
Tag-basierte Spaltensektionen
```

## Zugehörige Morphismen

```text
column_numbers_for_pair
normalize_column_buckets
sync_from_tables
sync_tables
```

## Universelle Eigenschaft

```text
Jeder spätere Tabellenpfad, der Spalten braucht,
faktorisiert über die normalisierte Spaltensektion.
```

Warum?

Weil sonst widersprüchliche Spaltenauswahlen an mehreren Stellen unterschiedlich interpretiert würden.

## Priorität

```text
Priorität: sehr hoch
```

---

# 17. Operative Prägarbe: Kombinations-CSV-Prägarbe

Dateien:

```text
reta_architecture/concat_csv.py
reta_architecture/combi_join.py
```

## Was ist darin?

Lokale Kombinationsdaten:

```text
Galaxie-Kombi-Daten
Universum-Kombi-Daten
Kombi-CSV-Zeilen
Relationstabellen
Join-Fragmente
Kombinationsparameter
```

## Zugehörige Topologien

```text
Kombinations-Topologie
Tabellen-Topologie
Hauptparameter-Topologie
Nebenparameter-Topologie
Tag-Topologie
```

## Zugehörige Morphismen

```text
readConcatCsv
merge_parameter_dicts
prepare_output_table
sync_tables
```

## Universelle Eigenschaft als Prägarbe

```text
Lokale Kombi-CSV-Daten liegen als lokale Sektionen über Kombinationskontexten.
```

## Zugehörige Garben-Eigenschaft

```text
Lokale Kombi-Relationen kleben zu einer globalen Relationstabelle.
```

Also:

```text
lokale Galaxie-Kombi-Sektion
lokale Universum-Kombi-Sektion
gemeinsamer Tabellenkontext
        ↓
globale Kombi-Tabelle
```

## Priorität

```text
Priorität: hoch
```

---

# 18. Operative Garbe: Tabellen-Garbe / globale Tabellensektion

Diese ist eher materialisierte Garbe als einzelne Klasse.

Dateien:

```text
reta_architecture/table_runtime.py
reta_architecture/table_state.py
reta_architecture/table_generation.py
reta_architecture/table_preparation.py
reta_architecture/table_output.py
```

## Was ist darin?

```text
globale Tabelle
Originalzeilen
gefilterte Zeilen
gewählte Spalten
generierte Spalten
Kombi-Join-Ergebnis
Zellenvorbereitung
Wrapping-Zustand
Breiteninformationen
Ausgabezustand
```

## Zugehörige Topologien

```text
Tabellen-Topologie
Zeilen-Topologie
Spalten-/Parameter-Topologie
Kombinations-Topologie
Ausgabe-Topologie
Tag-Topologie
```

## Zugehörige Morphismen

```text
prepare_output_table
filter_original_lines
readConcatCsv
render_table_output
sync_tables
concat_love_polygon
concat_modallogik
create_spalte_gestirn
```

## Universelle Eigenschaft

```text
Die globale Tabelle ist der kanonische materialisierte Klebepunkt
aus Semantik, Zeilen, Spalten, CSV, Kombi-Daten und generierten Spalten.
```

Formel:

```text
ParameterSemanticsSheaf
+ RowSet
+ ColumnNumberSet
+ LocalCsvSections
+ GeneratedColumns
        ↓
Tables
```

Nach der Prozessparallelisierung zusätzlich:

```text
parallel vorbereitete Chunks
        ↓ deterministisches Gluing in Originalreihenfolge
dieselbe vorbereitete Tabelle wie seriell
```

## Priorität

```text
Priorität: maximal für Runtime
```

---

# 19. Operative Prägarbe: Ausgabe-Prägarbe

## Was ist darin?

Lokale Ausgabeoptionen:

```text
shell
html
csv
markdown
bbcode
emacs
nichts
nocolor
justtext
onetable
breite
breiten
dontwrap
endless
keinenummerierung
keineueberschriften
keineleereninhalte
spaltenreihenfolgeundnurdiese
```

## Zugehörige Topologien

```text
Ausgabe-Topologie
Tabellen-Topologie
Kontext-Topologie
Scope-Topologie
```

## Zugehörige Morphismen

```text
output_mode_for_tables
apply_output_mode
render
render_table_output
normalize_for_parity
```

## Universelle Eigenschaft als Prägarbe

```text
Ausgabeoptionen sind lokale Sektionen über Ausgabe-Kontexten.
```

Beispiel:

```text
--ausgabe html
```

ist eine lokale Ausgabe-Sektion.

## Zugehörige Garben-Eigenschaft

```text
Die Tabelle besitzt im gewählten Ausgabe-Kontext
eine kanonische gerenderte Ausgabesektion.
```

## Priorität

```text
Priorität: hoch
```

---

# 20. Operative Garbe: normalisierte Ausgabe-Garbe

## Was ist darin?

```text
gerenderte Ausgabe
normalisierte Ausgabe
vergleichbare Legacy-/Architektur-Ausgabe
Paritätsrepräsentation
```

## Zugehörige Morphismen

```text
render
render_table_output
normalize_for_parity
```

## Universelle Eigenschaft

```text
Unterschiedliche syntaktische Ausgaben werden auf eine gemeinsame
vergleichbare Normalform gebracht.
```

Also:

```text
alte Ausgabe
neue Ausgabe
        ↓ normalize_for_parity
vergleichbare Normalform
```

## Priorität

```text
Priorität: hoch
```

---

# 21. Operative Prägarbe: Completion-Prägarbe

Dateien:

```text
reta_architecture/completion_word.py
reta_architecture/completion_nested.py
```

## Was ist darin?

Lokale Completion-Daten:

```text
Wortliste
Cursor-Präfix
Prompt-Dokument
Nested-Completion-Kontext
Parameterwerte
Zeilenwerte
Spaltenwerte
Ausgabewerte
Kombiwerte
```

## Zugehörige Topologien

```text
Prompt-Topologie
Completion-Topologie
Scope-Topologie
Kontext-Topologie
```

## Zugehörige Morphismen

```text
restrict_to_cursor_prefix
resolve_completion_words
match_completion_word
select_nested_open_set
glue_equality_value_options
yield_nested_candidates
```

## Universelle Eigenschaft als Prägarbe

```text
Completion arbeitet nur auf der kleinsten lokalen Prompt-Umgebung,
die für den Cursor relevant ist.
```

Beispiel:

```text
reta --spalten reli|
        ↓
CursorPrefixOpenSet = reli
```

## Zugehörige Garben-Eigenschaft

```text
Aus lokalen Wortquellen, Runtime-Listen und i18n-Daten
klebt die aktuell gültige Kandidatensektion.
```

Also:

```text
Wortquelle
Runtime-Vokabular
Parameterlisten
Cursor-Kontext
        ↓
CompletionCandidates
```

## Priorität

```text
Priorität: hoch für retaPrompt
```

---

# 22. Operative Garbe: Completion-Kandidatengarbe

## Was ist darin?

```text
gültige Completion-Kandidaten
Wortkandidaten
Nested-Kandidaten
Gleichheits-/Kommawert-Kandidaten
Prompt-toolkit-kompatible Completion-Objekte
```

## Zugehörige Morphismen

```text
match_completion_word
yield_nested_candidates
glue_equality_value_options
```

## Universelle Eigenschaft

```text
Die Kandidatenliste ist die kanonische lokale Garbensektion
für den aktuellen Cursor-Kontext.
```

Das heißt:

```text
Wenn Cursor-Kontext gleich ist,
muss die Kandidatensektion stabil und reproduzierbar sein.
```

## Priorität

```text
Priorität: mittel bis hoch
```

---

# 23. Operative Prägarbe: Arithmetik-Prägarbe

Datei:

```text
reta_architecture/arithmetic.py
```

## Was ist darin?

Lokale arithmetische Ausdrücke:

```text
Zahlen
Faktorfragen
Teilerbereiche
Primfaktorfragen
Vielfachenfragen
Modulo-Strukturen
Ziffernprüfungen
```

## Zugehörige Topologien

```text
Zeilen-Topologie
Arithmetik-Topologie
Tabellen-Topologie
```

## Zugehörige Morphismen

```text
factor_pairs
prime_factorize
glue_divisor_range
divisor_range
modulo_table_lines
has_digit
```

## Universelle Eigenschaft als Prägarbe

```text
Arithmetische Rohdaten sind lokale Sektionen über Zahlen-/Zeilenkontexten.
```

## Zugehörige Garben-Eigenschaft

```text
Jede Zahl besitzt kanonische arithmetische Sektionen.
```

Beispiele:

```text
12 -> Faktorpaare
12 -> Primfaktoren
Zeilenmenge -> Teilersektion
```

## Priorität

```text
Priorität: hoch
```

---

# 24. Operative Prägarbe: Legacy-Fassaden-Prägarbe

Dateien:

```text
reta.py
libs/center.py
libs/lib4tables_prepare.py
libs/lib4tables_concat.py
libs/tableHandling.py
libs/word_completerAlx.py
libs/nestedAlx.py
```

## Was ist darin?

Alte Zugriffspfade:

```text
alte Funktionsnamen
alte Klassen
alte Importpfade
alte CLI-Einstiege
alte Prompt-Einstiege
alte TableHandling-Reexports
```

## Zugehörige Topologien

```text
Legacy-Fassaden-Topologie
Architektur-Kompatibilitäts-Topologie
Runtime-Topologie
Prompt-Topologie
Tabellen-Topologie
```

## Zugehörige Morphismen

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

## Universelle Eigenschaft als Prägarbe

```text
Alte Aufrufpfade sind lokale Sektionen über Legacy-Kontexten.
```

## Zugehörige Garben-Eigenschaft

```text
Alter Pfad und neuer Architekturpfad müssen beobachtbar kommutieren.
```

Also:

```text
Legacy-Aufruf
        ↓
altes Ergebnis
```

muss gleich sein zu:

```text
Architektur-Aufruf
        ↓
neues Ergebnis
```

nach Normalisierung.

## Priorität

```text
Priorität: hoch für Kompatibilität
Priorität: mittel für neue Architektur
```

---

# 25. Meta-Prägarbe: Architektur-Witness-/Vertrags-Prägarbe

Dateien:

```text
reta_architecture/architecture_contracts.py
reta_architecture/architecture_validation.py
reta_architecture/architecture_coherence.py
reta_architecture/architecture_witnesses.py
```

## Was ist darin?

Lokale Architekturzeugnisse:

```text
Kategorien
Funktoren
natürliche Transformationen
Witnesses
Contracts
Validation Specs
Coherence Checks
Package Integrity Checks
```

## Zugehörige Topologien

```text
Architektur-Topologie
Validierungs-Topologie
Kohärenz-Topologie
Kategorie-Theorie-Topologie
```

## Zugehörige Morphismen

```text
bootstrap_architecture_contracts
validate_contract_references
bootstrap_architecture_validation
validate
render_validation_diagram
```

## Universelle Eigenschaft als Prägarbe

```text
Lokale Architekturbehauptungen werden als prüfbare Sektionen gesammelt.
```

## Zugehörige Garben-Eigenschaft

```text
Alle lokalen Architektur-Witnesses kleben zu einem globalen Validierungszustand.
```

Also:

```text
Kategorie-Theorie
Architekturkarte
Verträge
Witnesses
Paketstruktur
        ↓
ArchitectureValidationBundle
```

## Priorität

```text
Priorität: hoch für Refactoring-Sicherheit
Priorität: mittel für Runtime
```

---

# 26. Meta-Garbe: Rehearsal-/Activation-Garbe

Dateien:

```text
reta_architecture/architecture_migration.py
reta_architecture/architecture_rehearsal.py
reta_architecture/architecture_activation.py
```

## Was ist darin?

```text
Migrationsschritte
Migrationswellen
Gates
Rehearsal-Moves
Rollback-Sektionen
Activation-Windows
Commit-Transaktionen
```

## Zugehörige Topologien

```text
Migrations-Topologie
Rehearsal-Topologie
Activation-Topologie
Architektur-Topologie
```

## Zugehörige Morphismen

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

## Universelle Eigenschaft

```text
Lokale Migrationsmoves kleben nur dann zu globaler Readiness,
wenn Gates, Invarianten und Rollbacks zusammenpassen.
```

Besonders wichtig:

```text
cover_wave
commit_transaction
```

Sie bilden:

```text
lokale Moves
lokale Gates
lokale Rollbacks
        ↓
globale Aktivierungstransaktion
```

## Priorität

```text
Priorität: hoch für Projektstabilität
Priorität: mittel für Programmausführung
```

---

# 27. Nach Topologien getrennt

| Topologie | Prägarbe | Garbe | Universelle Eigenschaft |
|---|---|---|---|
| Kontext-Topologie | lokale Kontextsektionen | globale Kontextsemantik | `refine` ist der kanonische Schnitt |
| Sprach-Topologie | lokale Sprachdaten, i18n-Dateien | geklebte Sprachsemantik | Sprachsektionen beschränken und kleben |
| Hauptparameter-Topologie | lokale Hauptparameter-Aliase | kanonische Hauptparameter | Alias-Quotient |
| Nebenparameter-Topologie | lokale Nebenparameter-Aliase | kanonische Parameterpaare | Nebenparameter wird im Hauptkontext eindeutig |
| Zeilen-Topologie | lokale Zeilenausdrücke | kanonische Zeilenmenge | Zeilensyntax expandiert eindeutig |
| Ausgabe-Topologie | lokale Ausgabeoptionen | kanonische Ausgabesektion | Ausgabe normalisiert auf semantische Gleichheit |
| Tag-Topologie | lokale Tag-Zuordnungen | globale Spaltentag-Sektion | Tag-Kombination klassifiziert Spalten |
| Kombinations-Topologie | lokale Kombi-CSV-Sektionen | globale Kombi-Tabelle | lokale Relationen kleben zum Join |
| Scope-Topologie | lokale Scope-Lesarten | kanonische Befehlslesart | Rohtext wird im richtigen Scope gelesen |
| Prompt-Topologie | PromptStatePresheaf | Prompt-/Command-Semantik | Cursor-Keim ist kleinster relevanter Kontext |
| Completion-Topologie | lokale Wort-/Nested-Optionen | Kandidatensektion | lokale Optionen kleben zu Completions |
| Tabellen-Topologie | lokale Tabellenfragmente | globale Tabelle | Tabelle ist materialisierter Klebepunkt |
| Arithmetik-Topologie | lokale Zahlenausdrücke | Faktor-/Teilersektionen | Zahlen haben kanonische Struktur |
| Legacy-Topologie | alte Zugriffspfade | Architekturäquivalenz | alte und neue Pfade kommutieren |
| Rehearsal-Topologie | lokale Migrationsmoves | globale Readiness | Covers kleben zu Aktivierbarkeit |
| Activation-Topologie | lokale Aktivierungen | Commit-Transaktion | lokale Aktivierungen kleben mit Rollback |

---

# 28. Nach Morphismusgruppen getrennt

| Morphismusgruppe | Prägarben-Seite | Garben-Seite | Universelle Eigenschaft |
|---|---|---|---|
| Kontextmorphismen | lokale offene Kontexte | verfeinerter Kontext | kleinster gemeinsamer Unterkontext |
| Alias-Morphismen | rohe Alias-Sektionen | kanonische Parametersemantik | Alias-Quotient |
| Prägarben-Morphismen | lokale Sektionen | eingeschränkte lokale Sektionen | Restriktion entlang kleinerer Kontexte |
| Garben-Morphismen | kompatible lokale Daten | globale Sektion | eindeutiges Gluing |
| Spalten-Morphismen | lokale Spaltenwünsche | normalisierte Spaltensektion | widerspruchsfreie Spaltennormalform |
| Zeilen-Morphismen | lokale Zeilensyntax | RowIndexSet | kanonische Zeilenmenge |
| Tabellen-Morphismen | Tabellenfragmente | globale Tabelle | materialisiertes Runtime-Objekt |
| Ausgabe-Morphismen | Ausgabeoptionen | Rendered/Normalized Output | kanonische Ausgabeform |
| Prompt-Morphismen | Prompt-Rohtext | ausführbarer Befehl | Kurzform/Rohtext wird kanonisch |
| Completion-Morphismen | Cursor-/Optionensektion | Kandidatensektion | Kandidaten kleben aus lokalem Kontext |
| Arithmetik-Morphismen | Zahlen-/Zeilenausdruck | Faktor-/Teilerstruktur | kanonische Zahlensektion |
| Legacy-Morphismen | alte Import-/Aufrufpfade | Architekturpfade | Kommutativität alter und neuer Pfade |
| Meta-Morphismen | lokale Architektur-Witnesses | globale Validierung | Witnesses kleben zu Kohärenz |
| Activation-Morphismen | lokale Moves/Gates | Transaktion | Readiness + Rollback + Commit |

---

# 29. Priorisierte Gesamtliste

## 1. `ParameterSemanticsSheaf`

```text
höchste Priorität
```

Warum?

Sie enthält die globale Bedeutung von Parametern, Aliasen und Spalten.

Ohne sie gibt es keine saubere Semantik.

---

## 2. `FilesystemPresheaf`

```text
höchste Priorität
```

Warum?

Sie sammelt die lokalen Datenquellen: CSV, i18n, Assets, Dokumente.

Ohne lokale Sektionen gibt es nichts zu kleben.

---

## 3. `PromptStatePresheaf`

```text
höchste Priorität für retaPrompt
```

Warum?

Sie macht Prompt-Eingaben zu lokalen Sektionen, statt sie als chaotischen Text zu behandeln.

---

## 4. Rohkommando-Prägarbe

```text
sehr hohe Priorität
```

Warum?

Nutzereingaben beginnen als Rohtext und müssen erst kontextuell gelesen werden.

---

## 5. Kanonische Parametergarbe

```text
maximale Priorität
```

Warum?

Sie ist der Zielpunkt der Aliasauflösung.

---

## 6. Zeilenbereichs-Prägarbe und Zeilenmengen-Garbe

```text
sehr hohe Priorität
```

Warum?

Zeilenbereiche bestimmen, was überhaupt berechnet und angezeigt wird.

---

## 7. Spaltenauswahl-Prägarbe und normalisierte Spalten-Garbe

```text
sehr hohe Priorität
```

Warum?

Parametersemantik muss in konkrete Spalten übersetzt werden.

---

## 8. Tabellen-Garbe / globale Tabellensektion

```text
maximale Runtime-Priorität
```

Warum?

Die Tabelle ist der materialisierte Klebepunkt der gesamten Runtime.

---

## 9. `GeneratedColumnsSheaf`

```text
sehr hohe Priorität
```

Warum?

Generierte Spalten müssen explizit synchronisiert werden, sonst werden sie versteckter Zustand.

---

## 10. `TableOutputSheaf`

```text
hohe Priorität
```

Warum?

Die sichtbare Ausgabe ist der beobachtbare Beweis, dass der Pfad korrekt war.

---

## 11. Completion-Prägarbe und Kandidaten-Garbe

```text
hoch für retaPrompt
```

Warum?

Sie macht Prompt-Completion lokal, kontextuell und reproduzierbar.

---

## 12. Kombinations-CSV-Prägarbe und Kombi-Garbe

```text
hoch
```

Warum?

Galaxie-/Universum-/Kombi-Relationen brauchen lokales CSV-Gluing.

---

## 13. Arithmetik-Prägarbe und Arithmetik-Garbe

```text
hoch
```

Warum?

Zahlen-, Teiler-, Primfaktor- und Vielfachenlogik tragen wichtige Zeilen-/Tabellensemantik.

---

## 14. Ausgabe-Prägarbe und normalisierte Ausgabe-Garbe

```text
hoch
```

Warum?

Alte und neue Ausgabe müssen vergleichbar bleiben.

---

## 15. Legacy-Fassaden-Prägarbe

```text
hoch für Kompatibilität
```

Warum?

Alte Aufrufpfade müssen weiterhin kommutieren.

---

## 16. Architektur-Witness-/Vertrags-Garbe

```text
hoch für Refactoring-Sicherheit
```

Warum?

Sie verhindert, dass die Architektur wieder in Monolith-Struktur zurückfällt.

---

## 17. Rehearsal-/Activation-Garbe

```text
hoch für Projektstabilität
```

Warum?

Weitere Umbauten müssen über Gates, Rollbacks und Transaktionen abgesichert bleiben.

---

# 30. Zentrale Pipeline

Die ganze Architektur lässt sich als Prägarben-/Garben-Pipeline lesen:

```text
lokale Rohdaten
  CSV, i18n, Prompt, Aliase, Zeilenausdrücke, Ausgabeoptionen
        ↓
Prägarben
  FilesystemPresheaf, PromptStatePresheaf, RawCommandPresheaf
        ↓
Restriktion auf offene Kontexte
  restrict, refine, open_for
        ↓
kompatible lokale Sektionen
        ↓
Garben-Gluing
  merge_parameter_dicts, canonicalize_pair, column_numbers_for_pair
        ↓
globale Semantik
  ParameterSemanticsSheaf
        ↓
Zeilen- und Spaltensektionen
  RowIndexSet, ColumnNumberSet
        ↓
globale Tabelle
  Tables
        ↓
generierte Spalten und Kombi-Join
  GeneratedColumnsSheaf, Kombi-Sektionen
        ↓
Ausgabe-Garbe
  TableOutputSheaf, HtmlReferenceSheaf
        ↓
normalisierte Ausgabe
  NormalizedOutput
```

---

# 31. Die wichtigste universelle Gesamteigenschaft

Die zentrale Aussage des architekturveränderten `reta` ist:

```text
Alle lokalen Datenquellen, Prompt-Eingaben, Aliase, CSV-Sektionen,
Zeilenparameter, Spaltenparameter und Ausgabeoptionen werden erst
als lokale Prägarben-Sektionen behandelt.

Nur kompatible lokale Sektionen werden über Garben-Morphismen geklebt.

Das geklebte Ergebnis ist die kanonische globale Semantik,
aus der eindeutig Tabellen, generierte Spalten und Ausgaben entstehen.
```

Oder sehr kurz:

```text
Prägarben sammeln lokal.
Garben kleben global.
Universelle Eigenschaften garantieren Eindeutigkeit.
Morphismen transportieren Bedeutung zwischen den Schichten.
Topologien sagen, wo alles gilt.
```

Das ist der eigentliche Gewinn des Architekturumbaus: `reta` ist nicht mehr ein Monolith aus Parametern, CSVs, Promptlogik und Tabellenzustand, sondern ein System aus lokalen Sektionen, Kontexten, Morphismen und global geklebten Semantiken.
