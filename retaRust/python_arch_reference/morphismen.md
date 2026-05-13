# Morphismen zu den Topologien im architekturveränderten `reta`

Im aktuellen architekturveränderten Stand gibt es:

```text
- 1 zentrale Morphismus-Datei:
  reta_architecture/morphisms.py

- 22 symbolische Kategorien:
  reta_architecture/category_theory.py

- 68 explizit benannte Kategorie-Morphismen

- mehrere aktivierte Morphismus-Bundles:
  RowRangeMorphismBundle
  ArithmeticMorphismBundle
  ConsoleIOMorphismBundle
  WordCompletionMorphismBundle
  NestedCompletionMorphismBundle
```

Ein Morphismus ist hier nicht einfach „eine Funktion“. In dieser Architektur bedeutet Morphismus:

```text
eine strukturwahrende Abbildung
von einem Kontext, einer lokalen Sektion, einer Garbe, einer Tabelle,
einem Prompt-Zustand oder einem Legacy-Pfad
in einen anderen solchen Bereich.
```

Oder einfacher:

```text
Topologie = Wo gilt etwas?
Morphismus = Wie bewege ich Bedeutung korrekt von einem Ort zum anderen?
```

---

# 1. Höchste Priorität: Kontext-Morphismen

Datei:

```text
reta_architecture/topology.py
```

Kategorie:

```text
OpenRetaContextCategory
```

Diese Morphismen sind die Grundlage. Ohne sie gibt es keine saubere Topologie.

## Morphismen

```text
refine
open_for
cover_for_main
```

## Erklärung

### `open_for`

```text
RetaContextTopology -> ContextSelection
```

Dieser Morphismus erzeugt eine Basisöffnung.

Beispiel:

```python
topology.open_for("main_parameters", ["Religionen"])
```

bedeutet:

```text
Öffne den Kontextbereich:
Hauptparameter = Religionen
```

Oder:

```python
topology.open_for("output_modes", ["html"])
```

bedeutet:

```text
Öffne den Ausgabe-Kontext HTML.
```

Dieser Morphismus ist fundamental, weil er aus rohen Dimensionen echte offene Mengen macht.

---

### `refine`

```text
ContextSelection -> ContextSelection
```

Das ist die Verfeinerung eines Kontextes.

Beispiel:

```text
Religionen
∩ Spalten-Scope
∩ Deutsch
```

ergibt:

```text
Religionen im deutschen Spalten-Kontext
```

`refine` ist mathematisch der wichtigste Topologie-Morphismus. Er ist der Schnitt offener Mengen.

Ohne `refine` könnte `reta` nicht sauber sagen:

```text
Dieses Wort gilt nur in diesem Scope.
Dieser Parameter gilt nur in dieser Sprache.
Diese Ausgabe gilt nur in diesem Ausgabe-Kontext.
```

---

### `cover_for_main`

```text
RetaContextTopology -> ContextCover
```

Dieser Morphismus erzeugt eine Überdeckung für einen Hauptparameter.

Beispiel:

```text
Religionen
```

wird überdeckt durch:

```text
Hauptparameter-Kontext Religionen
+
Spalten-Scope
```

Das ist wichtig für Prägarben und Garben, weil lokale Daten über solchen Überdeckungen später zu globaler Semantik geklebt werden.

## Priorität

```text
Priorität: maximal
```

Diese drei Morphismen sind die topologische Basis des gesamten Systems.

---

# 2. Höchste Priorität: Alias- und Parameter-Morphismen

Dateien:

```text
reta_architecture/morphisms.py
reta_architecture/sheaves.py
reta_architecture/semantics_builder.py
```

Konkrete Klasse:

```python
AliasMorphisms
```

Kategorie:

```text
CanonicalSemanticSheafCategory
```

## Morphismen

```text
resolve_main_alias
resolve_parameter_alias
canonicalize_pair
column_numbers_for_pair
```

In der Kategorie-Theorie-Schicht heißen die wichtigsten davon:

```text
canonicalize_pair
column_numbers_for_pair
sync_from_tables
```

---

## Erklärung

Diese Morphismen übersetzen rohe Wörter in kanonische Bedeutung.

Beispiel:

```text
religion
Religion
Religionen
religions
```

werden auf denselben kanonischen Hauptparameter gebracht:

```text
Religionen
```

Oder:

```text
a1
absicht 1
Absicht_1
```

wird auf einen kanonischen Nebenparameter gebracht.

---

### `resolve_main_alias`

```text
Rohname -> kanonischer Hauptparameter
```

Beispiel:

```text
"religion" -> "Religionen"
"galaxie" -> "Galaxie"
"universum" -> "Universum"
```

Dieser Morphismus wirkt auf der Hauptparameter-Topologie.

---

### `resolve_parameter_alias`

```text
Hauptparameter + roher Nebenparameter -> kanonischer Nebenparameter
```

Beispielhaft:

```text
Religionen + "a1"
-> Religionen + kanonische Absicht-1-Semantik
```

Dieser Morphismus ist sehr wichtig für Prompt und CLI, weil Nutzer fast nie exakt die interne kanonische Schreibweise eingeben.

---

### `canonicalize_pair`

```text
Alias-Paar -> kanonisches Parameterpaar
```

Das ist einer der wichtigsten Morphismen überhaupt.

Er macht aus:

```text
("religion", "a1")
```

sinngemäß:

```text
("Religionen", "Absicht_1_...")
```

Dieser Morphismus verbindet:

```text
Prompt-Topologie
Parameter-Topologie
Nebenparameter-Topologie
Garbe der kanonischen Semantik
```

---

### `column_numbers_for_pair`

```text
kanonisches Parameterpaar -> Spaltennummernmenge
```

Beispiel:

```text
("Religionen", "Absicht_1")
-> {bestimmte Spaltennummern}
```

Dieser Morphismus ist die Brücke von Bedeutung zu Tabelle.

Er beantwortet:

```text
Welche Spalten gehören zu diesem semantischen Begriff?
```

## Priorität

```text
Priorität: maximal
```

Wenn diese Morphismen falsch sind, ist die gesamte Semantik falsch.

---

# 3. Höchste Priorität: Prägarben-Morphismen

Datei:

```text
reta_architecture/presheaves.py
```

Kategorie:

```text
LocalSectionCategory
```

## Morphismen

```text
add_section
restrict
update_prompt_state
```

## Erklärung

Die Prägarbe sammelt lokale Daten über offenen Kontexten.

Lokale Daten sind zum Beispiel:

```text
CSV-Dateien
Übersetzungsdateien
Prompt-Eingaben
Asset-Dateien
lokale Rohparameter
```

---

### `add_section`

```text
LocalSection -> FilesystemPresheaf
```

Dieser Morphismus registriert eine lokale Sektion.

Beispiel:

```text
csv/en-religion.csv
```

wird als lokale Sektion in die Prägarbe eingehängt.

---

### `restrict`

```text
FilesystemPresheaf -> LocalSection
```

Dieser Morphismus schränkt eine lokale Sektion auf einen kleineren Kontext ein.

Beispiel:

```text
alle CSV-Sektionen
-> nur englische Religionen-CSV-Sektionen
```

Oder:

```text
alle Prompt-Sektionen
-> nur der aktuelle Spalten-Scope
```

Das ist der klassische Prägarben-Morphismus:

```text
größerer offener Kontext -> kleinerer offener Kontext
```

Wichtig: Prägarben sind kontravariant. Eine kleinere offene Menge bekommt Daten durch Einschränkung aus einer größeren offenen Menge.

---

### `update_prompt_state`

```text
PromptStatePresheaf -> LocalSection
```

Dieser Morphismus aktualisiert den lokalen Prompt-Zustand.

Beispiel:

```text
Nutzer tippt: a1
```

Daraus entsteht eine lokale Prompt-Sektion.

## Priorität

```text
Priorität: sehr hoch
```

Diese Morphismen verbinden die Topologie mit echten Dateien und echter Nutzereingabe.

---

# 4. Höchste Priorität: Garben- und Gluing-Morphismen

Dateien:

```text
reta_architecture/sheaves.py
reta_architecture/universal.py
```

Kategorien:

```text
CanonicalSemanticSheafCategory
UniversalConstructionCategory
```

## Morphismen

```text
merge_parameter_dicts
normalize_column_buckets
sync_tables
sync_from_tables
```

---

## Erklärung

Diese Morphismen kleben lokale Daten zu globaler, konsistenter Semantik.

Das ist der Kern der Garben-Idee.

```text
lokale Sektionen
-> kompatible lokale Sektionen
-> globale Semantik
```

---

### `merge_parameter_dicts`

```text
ParameterDictionaryDiagram -> ParameterSemanticsSheaf
```

Dieser Morphismus klebt lokale Parameter-Dictionaries zusammen.

Beispielhaft:

```text
lokale deutsche Parameter
lokale englische Parameter
lokale CSV-Parameter
lokale Kombi-Parameter
```

werden zu:

```text
globaler Parametersemantik
```

Das ist pushout-artiges Gluing.

---

### `normalize_column_buckets`

```text
ColumnBucketDiagram -> NormalizedColumnBuckets
```

Dieser Morphismus normalisiert positive und negative Spaltenauswahlen.

Beispiel:

```text
gewählte Spalten:    {1, 2, 3, 4}
abgewählte Spalten:  {3}
```

wird:

```text
effektive Spalten: {1, 2, 4}
```

Dieser Morphismus ist wichtig, damit widersprüchliche Eingaben nicht chaotisch in die Tabelle laufen.

---

### `sync_tables`

```text
Tables -> SheafBundle
```

Dieser Morphismus synchronisiert globale Tabellenzustände zurück in die Garben.

Beispiel:

```text
generierte Spalten
Ausgabe-Sektionen
HTML-Referenzen
Tabellenmetadaten
```

werden in die Sheaf-Schicht gespiegelt.

---

### `sync_from_tables`

```text
Tables -> GeneratedColumnsSheaf
```

Dieser Morphismus nimmt Tabellenmetadaten und macht daraus Garbeninformationen über generierte Spalten.

## Priorität

```text
Priorität: maximal bis sehr hoch
```

Diese Morphismen entscheiden, ob lokale Daten wirklich konsistent global werden.

---

# 5. Sehr hohe Priorität: Tabellen-Morphismen

Dateien:

```text
reta_architecture/table_runtime.py
reta_architecture/table_state.py
reta_architecture/table_preparation.py
reta_architecture/row_filtering.py
reta_architecture/concat_csv.py
reta_architecture/table_output.py
```

Kategorie:

```text
TableSectionCategory
```

## Morphismen

```text
prepare_output_table
filter_original_lines
readConcatCsv
render_table_output
```

## Erklärung

Diese Morphismen machen aus Semantik konkrete Tabellen.

---

### `prepare_output_table`

```text
Tables -> Tables
```

Das ist ein Tabellen-Endomorphismus.

Er nimmt eine globale Tabelle und bereitet sie für die Ausgabe vor.

Dabei passieren Dinge wie:

```text
Zeilen vorbereiten
Zellen formatieren
Breiten berechnen
Wrapping vorbereiten
Spaltenauswahl anwenden
```

Nach der Prozessparallelisierung ist genau dieser Bereich teilweise chunk-parallel ausführbar.

---

### `filter_original_lines`

```text
ParameterSection -> RowSet
```

Dieser Morphismus wählt Zeilen aus.

Beispiel:

```text
--zeilen primzahlen
```

wird zu:

```text
Menge der Primzahl-Zeilen
```

Oder:

```text
--zeilen 1-100
```

wird zu:

```text
{1, 2, 3, ..., 100}
```

Dieser Morphismus verbindet die Zeilen-Topologie mit der Tabellen-Topologie.

---

### `readConcatCsv`

```text
LocalCsvSection -> Tables
```

Dieser Morphismus klebt lokale CSV-Sektionen in die globale Tabelle.

Er verbindet:

```text
Prägarbe lokaler CSV-Dateien
-> globale Tabellen-Sektion
```

---

### `render_table_output`

```text
Tables -> RenderedOutput
```

Dieser Morphismus macht aus einer Tabelle sichtbare Ausgabe.

Beispiel:

```text
Tabelle -> Shell-Text
Tabelle -> HTML
Tabelle -> CSV
Tabelle -> Markdown
```

## Priorität

```text
Priorität: sehr hoch
```

Das sind die Morphismen, die den semantischen Kern in sichtbare Programmausgabe verwandeln.

---

# 6. Sehr hohe Priorität: Zeilenbereichs-Morphismen

Datei:

```text
reta_architecture/row_ranges.py
```

Konkrete Klasse:

```python
RowRangeMorphismBundle
```

Kategorie:

```text
ActivatedRowRangeCategory
```

## Morphismen

```text
str_as_generator_to_set
is_fraction_range_token
is_integer_range_token
is_row_range_token
is_fraction_or_integer_range
is_fraction_range
is_row_range
range_to_numbers
add_single_range_segment
add_range_couple_values
add_non_multiple_values
add_multiple_values
```

In der Kategorie-Schicht zusammengefasst als:

```text
parse_generator_literal
validate_row_range
expand_row_range
delegate_center_wrappers
```

---

## Erklärung

Diese Morphismen übersetzen rohe Zeilenbereichsausdrücke in konkrete Zeilenmengen.

---

### `parse_generator_literal`

```text
RowRangeExpression -> RowIndexSet
```

Beispiel:

```text
{1,2,3}
```

oder generatorartige Ausdrücke werden zu:

```text
{1, 2, 3}
```

---

### `validate_row_range`

```text
RowRangeExpression -> RowRangeSyntax
```

Dieser Morphismus prüft, ob ein Text überhaupt ein gültiger Zeilenbereich ist.

Beispiel:

```text
1-10
v5
3,7,11
```

werden als Zeilenbereichsausdrücke erkannt.

---

### `expand_row_range`

```text
RowRangeExpression -> RowIndexSet
```

Das ist der zentrale Zeilenmorphismus.

Er macht aus:

```text
"1-5"
```

die Menge:

```text
{1, 2, 3, 4, 5}
```

Oder aus einem Vielfachen-Ausdruck:

```text
v3
```

sinngemäß:

```text
{3, 6, 9, 12, ...}
```

---

### `delegate_center_wrappers`

```text
libs.center -> RowRangeMorphismBundle
```

Dieser Morphismus sorgt dafür, dass alte Namen aus `center.py` weiterhin funktionieren, aber intern auf die neue Architektur zeigen.

## Priorität

```text
Priorität: sehr hoch
```

Für Performance ist diese Schicht besonders wichtig, weil Zeilenmengen gut chunkbar sind.

---

# 7. Sehr hohe Priorität: Arithmetik-Morphismen

Datei:

```text
reta_architecture/arithmetic.py
```

Konkrete Klasse:

```python
ArithmeticMorphismBundle
```

Kategorie:

```text
ActivatedArithmeticCategory
```

## Morphismen

```text
factor_pairs
divisor_range
prime_factors
prime_repeat_legacy
prime_repeat_pairs
invert_int_value_dict
has_digit
modulo_table_lines
```

In der Kategorie-Schicht:

```text
factor_pairs
prime_factorize
glue_divisor_range
delegate_center_arithmetic
```

---

## Erklärung

Diese Morphismen bilden Zahlen und Zeilenmengen auf arithmetische Strukturen ab.

---

### `factor_pairs`

```text
ArithmeticExpression -> FactorPairSet
```

Beispiel:

```text
12
```

wird zu:

```text
(1,12), (2,6), (3,4)
```

Dieser Morphismus ist wichtig für Teiler-, Vielfache- und Zahlenstruktur.

---

### `prime_factorize`

```text
ArithmeticExpression -> PrimeFactorSection
```

Beispiel:

```text
12 -> 2, 2, 3
```

---

### `glue_divisor_range`

```text
RowIndexSet -> DivisorSection
```

Dieser Morphismus ist besonders interessant, weil er Zeilenbereichs-Topologie und Arithmetik-Topologie verbindet.

Beispiel:

```text
Zeilenmenge {6, 12, 18}
```

wird zu einer Teiler-/Faktorstruktur über diesen Zahlen.

---

### `delegate_center_arithmetic`

```text
libs.center -> ArithmeticMorphismBundle
```

Alte Funktionen wie:

```text
multiples
teiler
primfaktoren
primRepeat
textHatZiffer
```

delegieren auf die neue Arithmetik-Schicht.

## Priorität

```text
Priorität: sehr hoch
```

Diese Morphismen tragen die zahlenlogische Semantik von `reta`.

---

# 8. Sehr hohe Priorität: Prompt-Morphismen

Dateien:

```text
reta_architecture/morphisms.py
reta_architecture/prompt_language.py
reta_architecture/prompt_execution.py
reta_architecture/prompt_interaction.py
reta_architecture/prompt_preparation.py
```

Konkrete Klasse:

```python
PromptMorphisms
```

Zusätzliche Completion-Kategorien:

```text
ActivatedWordCompletionCategory
ActivatedNestedCompletionCategory
```

## Morphismen in `PromptMorphisms`

```text
split
split_prompt_text
split_command_words
expand_shorthand
```

---

## Erklärung

Diese Morphismen übersetzen Prompt-Text in ausführbare semantische Einheiten.

---

### `split`

```text
PromptText -> TokenList
```

Zerlegt Prompt-Text in Stücke.

---

### `split_prompt_text`

```text
PromptText -> PromptTokenSection
```

Spezialisierte Prompt-Zerlegung.

---

### `split_command_words`

```text
CommandText -> CommandWordSection
```

Besonders wichtig für Eingaben, die mit `reta` beginnen.

Beispiel:

```text
reta --spalten religion --zeilen 1-10
```

wird in Befehlswörter zerlegt.

---

### `expand_shorthand`

```text
PromptShorthand -> ExpandedCommand
```

Dieser Morphismus expandiert Kurzbefehle.

Beispielhaft:

```text
a1
```

wird in einen größeren `reta`-Befehl übersetzt.

Das ist genau der Bereich, der für `retaPrompt` entscheidend ist.

## Priorität

```text
Priorität: sehr hoch für retaPrompt
Priorität: mittel bis hoch für reta.py
```

---

# 9. Hohe Priorität: Word-Completion-Morphismen

Datei:

```text
reta_architecture/completion_word.py
```

Konkrete Klasse:

```python
WordCompletionMorphismBundle
```

Kategorie:

```text
ActivatedWordCompletionCategory
```

## Morphismen

```text
resolve_completion_words
restrict_to_cursor_prefix
match_completion_word
delegate_word_completer
```

Im Bundle zusätzlich:

```text
resolve_words
word_before_cursor
word_completion_matches
iter_word_completions
create_completer
```

---

## Erklärung

Diese Morphismen sind für einfache Wortvervollständigung zuständig.

---

### `resolve_completion_words`

```text
CompletionWordSection -> CompletionWordSection
```

Macht aus einer statischen oder callable Wortquelle eine konkrete Wortsektion.

---

### `restrict_to_cursor_prefix`

```text
Document -> CursorPrefixOpenSet
```

Dieser Morphismus ist topologisch interessant.

Er nimmt das ganze Prompt-Dokument und schränkt es auf den Text direkt vor dem Cursor ein.

Beispiel:

```text
reta --spalten reli|
```

wird zu:

```text
CursorPrefixOpenSet = "reli"
```

---

### `match_completion_word`

```text
CompletionWordSection -> CompletionCandidateSection
```

Vergleicht mögliche Wörter mit dem Cursor-Präfix.

Beispiel:

```text
"religion" passt zu "reli"
```

---

### `delegate_word_completer`

```text
libs.word_completerAlx -> WordCompletionMorphismBundle
```

Die alte Completion-Klasse bleibt Fassade, aber die Logik liegt in der Architektur.

## Priorität

```text
Priorität: hoch
```

Für die Kernsemantik weniger wichtig als Parameter- und Tabellenmorphismen, aber für `retaPrompt` sehr wichtig.

---

# 10. Hohe Priorität: Nested-Completion-Morphismen

Datei:

```text
reta_architecture/completion_nested.py
```

Konkrete Klasse:

```python
NestedCompletionMorphismBundle
```

Kategorie:

```text
ActivatedNestedCompletionCategory
```

## Morphismen

```text
select_nested_open_set
glue_equality_value_options
yield_nested_candidates
delegate_nested_completer
```

Im Bundle zusätzlich:

```text
create_completer
matchTextAlx
paraZeilen
paraSpalten
paraAusgabe
paraKombination
gleichKommaZeilen
gleichKommaSpalten
gleichKommaAusg
gleichKommaKombi
get_completions
```

---

## Erklärung

Diese Morphismen sind für hierarchische Prompt-Completion zuständig.

Also nicht nur:

```text
Welche Wörter beginnen mit "re"?
```

sondern:

```text
Wo im Befehl steht der Cursor?
Sind wir bei --zeilen?
Sind wir bei --spalten?
Sind wir nach einem "="?
Sind Kommawerte erlaubt?
Welche Werte passen jetzt?
```

---

### `select_nested_open_set`

```text
PromptDocument -> NestedCompletionOpenSet
```

Dieser Morphismus wählt die aktuelle Completion-Situation aus.

Beispiel:

```text
reta --spalten=
```

ist ein anderer offener Kontext als:

```text
reta --zeilen=
```

---

### `glue_equality_value_options`

```text
NestedOptionSection -> NestedOptionSection
```

Dieser Morphismus klebt verfügbare Werte zusammen.

Zum Beispiel aus:

```text
i18n-Wörtern
Runtime-Vokabular
Zeilenparametern
Spaltenparametern
Kombinationsparametern
Ausgabeparametern
```

---

### `yield_nested_candidates`

```text
NestedOptionSection -> NestedCompletionCandidateSection
```

Erzeugt konkrete Completion-Kandidaten für den Prompt.

---

### `delegate_nested_completer`

```text
libs.nestedAlx -> NestedCompletionMorphismBundle
```

Die alte `nestedAlx`-Importfläche bleibt kompatibel, aber die Logik ist architekturintern.

## Priorität

```text
Priorität: hoch für retaPrompt
```

Das ist die feinere Prompt-Topologie.

---

# 11. Hohe Priorität: Ausgabe-Morphismen

Dateien:

```text
reta_architecture/output_semantics.py
reta_architecture/output_syntax.py
reta_architecture/table_output.py
```

Konkrete Klasse in `morphisms.py`:

```python
RendererMorphisms
```

Kategorie:

```text
OutputFormatCategory
```

## Morphismen

```text
output_mode_for_tables
apply_output_mode
render
normalize_for_parity
```

---

## Erklärung

Diese Morphismen übersetzen Tabellen in Ausgabeformate.

---

### `output_mode_for_tables`

```text
Tables -> OutputMode
```

Bestimmt, welcher Ausgabemodus für eine Tabelle gilt.

Beispiel:

```text
shell
html
csv
markdown
bbcode
emacs
nichts
```

---

### `apply_output_mode`

```text
Tables -> OutputModeApplication
```

Wendet einen Modus auf Tabellen an.

Beispiel:

```text
Tabelle + html
-> HTML-Ausgabezustand
```

---

### `render`

```text
Tables -> RenderedOutput
```

Macht die Ausgabe wirklich sichtbar.

---

### `normalize_for_parity`

```text
RenderedOutput -> NormalizedOutput
```

Dieser Morphismus ist für Tests wichtig.

Er sagt:

```text
Unterschiedliche Ausgabeformen dürfen kleine syntaktische Unterschiede haben,
aber nach Normalisierung muss die semantische Ausgabe gleich sein.
```

## Priorität

```text
Priorität: hoch
```

Diese Morphismen sind für sichtbare Korrektheit wichtig, aber sie liegen nach der Semantik.

---

# 12. Mittlere bis hohe Priorität: Generierte-Spalten-Morphismen

Datei:

```text
reta_architecture/generated_columns.py
```

Kategorie:

```text
GeneratedColumnEndomorphismCategory
```

## Morphismen

```text
concat_love_polygon
concat_modallogik
create_spalte_gestirn
```

Im weiteren Code gibt es noch mehr generierte Spaltenlogik, aber diese drei sind die explizit symbolisch registrierten Kategorie-Morphismen.

---

## Erklärung

Diese Morphismen sind Tabellen-Endomorphismen:

```text
Tables -> Tables
```

Sie nehmen eine bestehende Tabelle und fügen abgeleitete Spalten hinzu.

---

### `concat_love_polygon`

```text
Tables -> Tables
```

Erzeugt Love-/Polygon-Spalten.

---

### `concat_modallogik`

```text
Tables -> Tables
```

Erzeugt Modallogik-Spalten.

---

### `create_spalte_gestirn`

```text
Tables -> Tables
```

Erzeugt Gestirn-Spalten.

## Priorität

```text
Priorität: mittel bis hoch
```

Diese Morphismen sind inhaltlich wichtig, aber sie kommen nach Kontext, Semantik, Zeilen und Tabellenbasis.

---

# 13. Mittlere bis hohe Priorität: Kombinations- und CSV-Morphismen

Dateien:

```text
reta_architecture/concat_csv.py
reta_architecture/combi_join.py
reta_architecture/generated_columns.py
```

Explizit registrierter Hauptmorphismus:

```text
readConcatCsv
```

Weitere operative Morphismen liegen in den Kombi-/Join-Funktionen.

## Erklärung

Diese Morphismen verbinden lokale CSV-Sektionen, Kombinationsparameter und Tabellen.

Sie arbeiten in der Kombinations-Topologie:

```text
Galaxie
Universum
Kombi-Relationen
Join-Semantik
```

Wichtige Bewegung:

```text
LocalCsvSection
-> Tables
```

und:

```text
Kombi-Parameter
-> Kombi-Tabellenstruktur
-> globale Tabelle
```

## Priorität

```text
Priorität: mittel bis hoch
```

Wichtig für relationale Erweiterungen, aber nicht die Basis des gesamten Systems.

---

# 14. Mittlere Priorität: Console-/Help-/Utility-Morphismen

Datei:

```text
reta_architecture/console_io.py
```

Konkrete Klasse:

```python
ConsoleIOMorphismBundle
```

Kategorie:

```text
ActivatedConsoleIOCategory
```

## Morphismen

```text
load_help_section
render_cli_output
discover_text_wrap_runtime
delegate_center_console_io
```

Im Bundle zusätzlich:

```text
reta_prompt_help_text
print_reta_prompt_help
reta_help_text
print_reta_help
get_text_wrap_things
cli_output
debug_pair
debug_value
chunks
unique_everseen
DefaultOrderedDict
```

---

## Erklärung

Diese Morphismen behandeln sichtbare Konsolenausgabe, Hilfe und kleine Utility-Sektionen.

---

### `load_help_section`

```text
HelpMarkdownSection -> ConsoleOutputSection
```

Lädt Hilfetexte aus lokalen Dokumentationssektionen.

---

### `render_cli_output`

```text
ConsoleOutputSection -> ConsoleOutputSection
```

Gibt Text sichtbar auf der Konsole aus.

---

### `discover_text_wrap_runtime`

```text
ConsoleOutputSection -> FiniteUtilitySection
```

Ermittelt Terminalbreite und Wrapping-Hilfen.

---

### `delegate_center_console_io`

```text
libs.center -> ConsoleIOMorphismBundle
```

Alte Namen wie:

```text
retaHilfe
retaPromptHilfe
cliout
getTextWrapThings
```

delegieren auf die neue Architektur.

## Priorität

```text
Priorität: mittel
```

Wichtig für Bedienung und Ausgabe, aber nicht die Kernsemantik.

---

# 15. Mittlere Priorität: Legacy-Fassaden-Morphismen

Dateien:

```text
reta.py
libs/tableHandling.py
libs/lib4tables_prepare.py
libs/lib4tables_concat.py
libs/center.py
libs/word_completerAlx.py
libs/nestedAlx.py
```

Kategorie:

```text
LegacyFacadeCategory
```

## Morphismen

```text
bootstrap_program
tableHandling_reexport
prepare_delegation
concat_delegation
```

Zusätzlich in aktivierten Kategorien:

```text
delegate_center_wrappers
delegate_center_arithmetic
delegate_center_console_io
delegate_word_completer
delegate_nested_completer
```

---

## Erklärung

Diese Morphismen sind Kompatibilitätsmorphismen.

Sie sagen:

```text
Alter Pfad -> neue Architektur
```

Beispiel:

```text
libs.center.BereichToNumbers2
-> reta_architecture.row_ranges.range_to_numbers
```

oder:

```text
libs.lib4tables_prepare.Prepare
-> reta_architecture.table_preparation
```

oder:

```text
libs.nestedAlx.NestedCompleter
-> reta_architecture.completion_nested.ArchitectureNestedCompleter
```

Diese Morphismen sind nicht schön, aber wichtig. Sie schützen alte Aufrufpfade.

## Priorität

```text
Priorität: mittel
```

Für Zukunftsentwicklung sollen sie eher dünner werden. Für Kompatibilität sind sie noch wichtig.

---

# 16. Meta-Priorität: Architekturvertrags-Morphismen

Dateien:

```text
reta_architecture/architecture_contracts.py
reta_architecture/architecture_validation.py
reta_architecture/architecture_coherence.py
```

Kategorien:

```text
CommutativeArchitectureContractCategory
ArchitectureValidationCategory
ArchitectureCoherenceCategory
```

## Morphismen

```text
bootstrap_architecture_contracts
validate_contract_references
render_contract_diagram
bootstrap_architecture_validation
validate_stage31_references
render_validation_diagram
validate
```

---

## Erklärung

Diese Morphismen betreffen nicht direkt `reta`-Ausgabe, sondern die Architekturprüfung.

Sie machen aus:

```text
Kategorien
Funktoren
natürlichen Transformationen
Kapseln
Witnesses
```

prüfbare Architekturverträge.

---

### `bootstrap_architecture_contracts`

```text
CategoryTheoryBundle -> ArchitectureContractsBundle
```

Macht aus Kategorie-Theorie konkrete Verträge.

---

### `validate_contract_references`

```text
ArchitectureContractsBundle -> ContractValidationSpec
```

Prüft, ob alle Verträge auf existierende Kategorien, Funktoren und Transformationen zeigen.

---

### `bootstrap_architecture_validation`

```text
ArchitectureWitnessBundle -> ArchitectureValidationBundle
```

Komponiert Witnesses, Verträge und Paketbaum zu einem Audit.

---

### `validate`

```text
ArchitectureCoherenceBundle -> ArchitectureValidationBundle
```

Prüft Kohärenz.

## Priorität

```text
Priorität: mittel für Runtime
Priorität: hoch für Refactoring-Sicherheit
```

Diese Morphismen schützen die Architektur gegen Rückfall in Monolith-Struktur.

---

# 17. Meta-Priorität: Trace-, Boundary-, Impact- und Migration-Morphismen

Dateien:

```text
reta_architecture/architecture_traces.py
reta_architecture/architecture_boundaries.py
reta_architecture/architecture_impact.py
reta_architecture/architecture_migration.py
```

Kategorien:

```text
ArchitectureTraceCategory
ArchitectureBoundaryCategory
ArchitectureImpactCategory
ArchitectureMigrationCategory
```

## Morphismen

```text
trace
classify_import
compute_impact
gate
plan
order
bind_gate
preserve_invariant
```

---

## Erklärung

Diese Morphismen beschreiben das Refactoring selbst.

---

### `trace`

```text
RetaComponentTraceSpec -> ArchitectureTraceBundle
```

Macht alte Komponenten navigierbar.

Beispiel:

```text
alter Besitzer center.py
-> neue Kapsel / neuer Architekturpfad
```

---

### `classify_import`

```text
ModuleOwnershipSpec -> ImportEdgeSpec
```

Klassifiziert Imports als Architekturgrenzen.

---

### `compute_impact`

```text
ImpactSourceSpec -> ArchitectureImpactBundle
```

Berechnet, welche Änderungen welche Architekturteile treffen.

---

### `gate`

```text
MigrationCandidateSpec -> RegressionGateSpec
```

Gibt Migrationskandidaten passende Regression-Gates.

---

### `plan`

```text
MigrationCandidateSpec -> MigrationStepSpec
```

Macht aus einem Kandidaten einen konkreten Migrationsschritt.

---

### `order`

```text
MigrationStepSpec -> MigrationWaveSpec
```

Ordnet Schritte in Migrationswellen.

---

### `bind_gate`

```text
MigrationStepSpec -> MigrationGateBindingSpec
```

Bindet Tests/Gates an konkrete Schritte.

---

### `preserve_invariant`

```text
MigrationWaveSpec -> MigrationInvariantSpec
```

Hält fest, welche Invarianten bei der Migration nicht verletzt werden dürfen.

## Priorität

```text
Priorität: mittel
```

Wichtig für Projektführung, weniger für unmittelbare Programmausgabe.

---

# 18. Meta-Priorität: Rehearsal- und Activation-Morphismen

Dateien:

```text
reta_architecture/architecture_rehearsal.py
reta_architecture/architecture_activation.py
```

Kategorien:

```text
ArchitectureRehearsalCategory
ArchitectureActivationCategory
```

## Morphismen

```text
rehearse_step
rehearse_gate
cover_wave
activate_move
activate_gate
rollback
commit_transaction
```

---

## Erklärung

Diese Morphismen behandeln Trockenläufe und Aktivierungsschritte.

---

### `rehearse_step`

```text
MigrationStepSpec -> RehearsalMoveSpec
```

Ein geplanter Schritt wird zum Trockenlauf-Move.

---

### `rehearse_gate`

```text
MigrationGateBindingSpec -> GateRehearsalSpec
```

Ein Gate-Binding wird zur prüfbaren Gate-Suite.

---

### `cover_wave`

```text
RehearsalOpenSetSpec -> RehearsalCoverSpec
```

Lokale Moves kleben zur globalen Readiness.

Das ist topologisch interessant: Eine Migrationswelle wird wie eine Überdeckung behandelt.

---

### `activate_move`

```text
RehearsalMoveSpec -> ActivationUnitSpec
```

Ein Trockenlauf wird aktivierungsbereit.

---

### `activate_gate`

```text
GateRehearsalSpec -> ActivationGateSpec
```

Ein Rehearsal-Gate wird Commit-/Rollback-Gate.

---

### `rollback`

```text
ActivationGateSpec -> ActivationRollbackSpec
```

Commit-Gate bekommt Rollback-Sektion.

---

### `commit_transaction`

```text
ActivationWindowSpec -> ActivationTransactionSpec
```

Lokale Aktivierungen werden zu einer Transaktion geklebt.

## Priorität

```text
Priorität: mittel
```

Das ist die Topologie des Architekturumbaus, nicht die Runtime-Topologie.

---

# Die Morphismen nach Topologie sortiert

## Kontext-Topologie

```text
open_for
refine
cover_for_main
```

Zweck:

```text
offene Kontexte erzeugen, schneiden, überdecken
```

---

## Sprach-Topologie

Direkt über Kontextmorphismen:

```text
open_for("language", ...)
refine(...)
```

Zusätzlich über Prägarben/Garben:

```text
restrict
merge_parameter_dicts
```

Zweck:

```text
sprachspezifische lokale Sektionen einschränken und kleben
```

---

## Hauptparameter-Topologie

Wichtige Morphismen:

```text
resolve_main_alias
canonicalize_pair
column_numbers_for_pair
cover_for_main
```

Zweck:

```text
Hauptparameter-Aliase kanonisieren und in semantische Spaltenräume überführen
```

---

## Nebenparameter-Topologie

Wichtige Morphismen:

```text
resolve_parameter_alias
canonicalize_pair
column_numbers_for_pair
```

Zweck:

```text
Nebenparameter-Aliase in kanonische Parameterpaare und Spaltenmengen übersetzen
```

---

## Zeilenparameter-Topologie

Wichtige Morphismen:

```text
validate_row_range
expand_row_range
filter_original_lines
parse_generator_literal
glue_divisor_range
```

Zweck:

```text
Zeilenangaben in konkrete Zeilenmengen überführen
```

---

## Ausgabe-Topologie

Wichtige Morphismen:

```text
output_mode_for_tables
apply_output_mode
render
render_table_output
normalize_for_parity
render_cli_output
```

Zweck:

```text
Tabellen in Shell/HTML/CSV/Markdown/BBCode/Emacs-Ausgabe transformieren
```

---

## Tag-Topologie

Wichtige Morphismen:

```text
sync_from_tables
sync_tables
concat_love_polygon
concat_modallogik
create_spalte_gestirn
```

Zweck:

```text
Spaltentags, generierte Spalten und Tabellenmetadaten synchronisieren
```

---

## Kombinations-Topologie

Wichtige Morphismen:

```text
readConcatCsv
merge_parameter_dicts
prepare_output_table
sync_tables
```

Zweck:

```text
Kombi-CSV, Galaxie/Universum-Relationen und globale Tabellenstruktur zusammenführen
```

---

## Scope-Topologie

Wichtige Morphismen:

```text
split_command_words
expand_shorthand
canonicalize_pair
filter_original_lines
apply_output_mode
```

Zweck:

```text
entscheiden, ob Eingaben als Zeilen-, Spalten-, Kombinations-, Ausgabe- oder Hilfeausdruck gelesen werden
```

---

## Prägarben-Topologie

Wichtige Morphismen:

```text
add_section
restrict
update_prompt_state
```

Zweck:

```text
lokale Dateien, Übersetzungen und Prompt-Zustände über offenen Kontexten verwalten
```

---

## Garben-Topologie

Wichtige Morphismen:

```text
merge_parameter_dicts
canonicalize_pair
column_numbers_for_pair
sync_from_tables
sync_tables
```

Zweck:

```text
lokale Semantik zu global konsistenter Semantik kleben
```

---

## Prompt-Topologie

Wichtige Morphismen:

```text
split
split_prompt_text
split_command_words
expand_shorthand
restrict_to_cursor_prefix
select_nested_open_set
glue_equality_value_options
yield_nested_candidates
```

Zweck:

```text
Prompt-Text in ausführbare Befehle und Completion-Kontexte übersetzen
```

---

## Tabellen-Topologie

Wichtige Morphismen:

```text
prepare_output_table
filter_original_lines
readConcatCsv
render_table_output
concat_love_polygon
concat_modallogik
create_spalte_gestirn
```

Zweck:

```text
aus Semantik konkrete Tabellen erzeugen, vorbereiten, erweitern und rendern
```

---

## Rehearsal-/Activation-Topologie

Wichtige Morphismen:

```text
rehearse_step
rehearse_gate
cover_wave
activate_move
activate_gate
rollback
commit_transaction
```

Zweck:

```text
Architekturumbau als sichere, prüfbare, lokal-global geklebte Migration behandeln
```

---

# Gesamtrangliste der wichtigsten Morphismen

## 1. `refine`

Weil es der zentrale Schnitt offener Kontexte ist.

```text
ContextSelection -> ContextSelection
```

Ohne `refine` gibt es keine funktionierende Kontexttopologie.

---

## 2. `canonicalize_pair`

Weil es rohe Eingabe in kanonische Semantik verwandelt.

```text
ParameterSemanticsSheaf -> CanonicalParameterPair
```

Ohne diesen Morphismus weiß `reta` nicht zuverlässig, was ein Parameter bedeutet.

---

## 3. `column_numbers_for_pair`

Weil es Bedeutung in Spalten übersetzt.

```text
CanonicalParameterPair -> ColumnNumberSet
```

Das ist die Brücke von Semantik zur Tabelle.

---

## 4. `restrict`

Weil es Prägarben auf kleinere Kontexte einschränkt.

```text
FilesystemPresheaf -> LocalSection
```

Das ist der wichtigste Prägarben-Morphismus.

---

## 5. `merge_parameter_dicts`

Weil es lokale Parameterdaten global klebt.

```text
ParameterDictionaryDiagram -> ParameterSemanticsSheaf
```

Das ist einer der wichtigsten Garben-/Gluing-Morphismen.

---

## 6. `normalize_column_buckets`

Weil es widersprüchliche Spaltenauswahlen bereinigt.

```text
ColumnBucketDiagram -> NormalizedColumnBuckets
```

Ohne diesen Morphismus entstehen leicht inkonsistente Tabellenzustände.

---

## 7. `expand_row_range`

Weil es Zeilenangaben in konkrete Zeilenmengen überführt.

```text
RowRangeExpression -> RowIndexSet
```

Für Runtime und Parallelisierung sehr wichtig.

---

## 8. `filter_original_lines`

Weil es aus Parametern echte Tabellenzeilen macht.

```text
ParameterSection -> RowSet
```

---

## 9. `prepare_output_table`

Weil es die globale Tabelle ausgabefähig macht.

```text
Tables -> Tables
```

Das ist einer der wichtigsten Runtime-Morphismen.

---

## 10. `readConcatCsv`

Weil es lokale CSV-Sektionen in die globale Tabelle klebt.

```text
LocalCsvSection -> Tables
```

---

## 11. `render_table_output` / `render`

Weil es Tabellen sichtbar macht.

```text
Tables -> RenderedOutput
```

---

## 12. `split_command_words`

Weil es Prompt-/CLI-Text in Befehlsstruktur übersetzt.

```text
CommandText -> CommandWordSection
```

---

## 13. `expand_shorthand`

Weil es Kurzbefehle wie `a1` in echte `reta`-Befehle verwandelt.

```text
PromptShorthand -> ExpandedCommand
```

---

## 14. `select_nested_open_set`

Weil es im Prompt entscheidet, welcher Completion-Kontext gerade gilt.

```text
PromptDocument -> NestedCompletionOpenSet
```

---

## 15. `sync_tables`

Weil es Tabellenzustände zurück in Garben synchronisiert.

```text
Tables -> SheafBundle
```

---

# Kompakte Gesamttabelle

| Priorität | Morphismusgruppe | Wichtigste Morphismen | Zweck |
|---:|---|---|---|
| 1 | Kontext | `refine`, `open_for`, `cover_for_main` | offene Kontexte erzeugen, schneiden, überdecken |
| 2 | Parametersemantik | `canonicalize_pair`, `column_numbers_for_pair` | Aliase zu kanonischer Semantik und Spalten machen |
| 3 | Prägarbe | `add_section`, `restrict`, `update_prompt_state` | lokale Sektionen verwalten |
| 4 | Garbe/Gluing | `merge_parameter_dicts`, `normalize_column_buckets`, `sync_tables` | lokale Daten global konsistent kleben |
| 5 | Zeilen | `expand_row_range`, `validate_row_range`, `filter_original_lines` | Zeilenangaben zu Zeilenmengen machen |
| 6 | Tabelle | `prepare_output_table`, `readConcatCsv`, `render_table_output` | Tabellen erzeugen, vorbereiten, ausgeben |
| 7 | Prompt | `split_command_words`, `expand_shorthand` | Prompt-Text zu Befehlen machen |
| 8 | Completion | `restrict_to_cursor_prefix`, `select_nested_open_set`, `yield_nested_candidates` | Prompt-Completion erzeugen |
| 9 | Ausgabe | `apply_output_mode`, `render`, `normalize_for_parity` | Ausgabeformate erzeugen und vergleichen |
| 10 | Generierte Spalten | `concat_love_polygon`, `concat_modallogik`, `create_spalte_gestirn` | abgeleitete Spalten erzeugen |
| 11 | Arithmetik | `factor_pairs`, `prime_factorize`, `glue_divisor_range` | Zahlenstruktur berechnen |
| 12 | Legacy-Fassaden | `bootstrap_program`, `prepare_delegation`, `concat_delegation` | alte Pfade auf neue Architektur abbilden |
| 13 | Metaarchitektur | `trace`, `classify_import`, `plan`, `commit_transaction` | Refactoring absichern |

---

# Wichtigste Erkenntnis

Die Morphismen im architekturveränderten `reta` bilden eine klare Kette:

```text
roher Text / lokale Datei / CSV / Prompt
        ↓
lokale Sektion
        ↓
Kontext-Einschränkung
        ↓
kanonische Semantik
        ↓
Spalten- und Zeilenmengen
        ↓
globale Tabelle
        ↓
generierte Spalten / Kombi-Join / Ausgabevorbereitung
        ↓
gerenderte Ausgabe
```

Mathematisch formuliert:

```text
Prompt-/Datei-Prägarben
werden über Kontextrestriktionen eingeschränkt,
über Garbenmorphismen geklebt,
über Tabellenmorphismen materialisiert
und über Ausgabe-Morphismen gerendert.
```

Praktisch formuliert:

```text
Die Morphismen sind die Straßen zwischen den Topologien.
```

Die wichtigsten Straßen sind:

```text
Kontext verfeinern
Alias kanonisieren
Spalten finden
Zeilen expandieren
lokale Daten kleben
Tabelle vorbereiten
Ausgabe rendern
```

Alles andere ist Ergänzung, Absicherung oder Kompatibilität.
