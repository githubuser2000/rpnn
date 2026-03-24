# Vollständige Markdown-Dokumentation des `src`-Verzeichnisses

## 1. Kurzüberblick

Dieses Projekt ist ein CLI-Werkzeug, das CSV-Daten in SQLite lädt, Benutzerargumente auf Spalten- und Zeilenbereiche abbildet, daraus optional zusätzliche berechnete Spalten erzeugt und die Ergebnisse terminalgerecht als Tabelle rendert.

Der aktuelle Hauptpfad läuft grob so:

1. `src/main.rs` startet `workflows::main_workflow()`.
2. `workflows.rs` lädt die Kategoriemap, verarbeitet CLI-Argumente und Generatorwünsche.
3. `csv_importer.rs` importiert die CSV-Dateien in eine SQLite-DB im Speicher.
4. `pypy_compat.rs` ergänzt bei Bedarf zusätzliche kompatible Hilfsspalten.
5. `table_printer/query.rs` baut die SQL-Abfrage, sortiert die Rückgabe und stößt die Ausgabe an.
6. `table_printer/printer.rs` und `reta_ausgabe/*` rendern die Daten in terminalgerechten Chunks.

## 2. Architektur

### 2.1 Domänenzustand

- `TextBereich` ist die wichtigste Zustandsstruktur. Sie hält Zeilenbereiche, Spaltenselektion, Breitenvorgaben, Generatorbefehle, PyPy-Kompatibilitätsflags und weitere Laufzeitoptionen.
- `SpaltenNamen`/`SpaltenNamenListe` halten die semantische Anforderung aus der CLI, also z. B. Ober- und Unterkategorie.
- `KategorieMap` ist das Nachschlagewerk von Begriffen auf konkrete Spaltennummern.

### 2.2 Datenfluss

- Eingaben werden in `cli/parser.rs` syntaktisch zerlegt.
- `argument_verarbeiter.rs` und `kategorie_verarbeiter.rs` machen daraus eine fachliche Spaltenauswahl.
- `column_manager/*` und `table_printer/query.rs` übersetzen diese Auswahl in SQL.
- `generated_columns_words_registry.rs` und `pypy_compat.rs` können den Datenbestand bzw. die Header dynamisch erweitern.
- `table_printer/printer.rs` berechnet Breiten/Chunks und delegiert für die konkrete Zellen-/Zeilenrepräsentation an `reta_ausgabe/*`.

### 2.3 Technische Schwerpunkte

- SQLite wird als temporäre Query-Engine genutzt, statt CSV direkt zu durchsuchen.
- Die Ausgabe ist stark terminalorientiert: Wortumbruch, Chunking, Zeilennummern, Spezialmarkierungen und explizite Spaltenbreiten sind zentrale Features.
- Im Quellbaum liegen sowohl aktuelle Rust-Implementierungen als auch ältere Python-/Backup-Dateien. Das Projekt ist sichtbar in Migration.

## 3. Datei-für-Datei-Dokumentation

### `src/argument_verarbeiter.rs`

**Umfang:** 260 Zeilen  
**Rolle:** Verknüpft rohe CLI-Daten mit der Kategoriemap. Sucht passende Spalten, behandelt Generator-Sonderfälle und erzeugt die finale interne Suchbeschreibung.

**Wichtige Symbole:**

- Zeile 8: `fn normalize_category_key`
- Zeile 15: `fn is_primzahlkreuz_pro_contra_request`
- Zeile 23: `struct SpaltenVerarbeiter`
- Zeile 28: `struct VerarbeitungsErgebnis`
- Zeile 34: `fn new`
- Zeile 38: `fn verarbeite_zu_tupel`
- Zeile 43: `fn verarbeite`
- Zeile 55: `fn merge_exact`
- Zeile 91: `fn verarbeite_automatische_spalten`
- Zeile 163: `fn suche_und_setze_spalten`
- Zeile 219: `fn finalize_found_columns`
- Zeile 228: `fn setze_gefundene_spalten`
- Zeile 251: `fn fallback_zu_standards`

**Kommentar:** Der entscheidende Schritt ist `verarbeite()`: Hier werden Parser-Ergebnisse mit der Kategoriemap, exakten Generatoren und Fallback-Regeln zusammengeführt.

### `src/argument_verarbeiter_generated_pair.rs`

**Umfang:** 230 Zeilen  
**Rolle:** Backup-, Zwischenstands- oder experimentelle Datei; nützlich zum Vergleich, aber nicht zwingend Teil des aktuellen Hauptpfads.

- Status: alternative/ältere Logik für Generated-Pair-Auflösung.

**Wichtige Symbole:**

- Zeile 8: `fn normalize_category_key`
- Zeile 15: `fn is_primzahlkreuz_pro_contra_request`
- Zeile 23: `struct SpaltenVerarbeiter`
- Zeile 28: `struct VerarbeitungsErgebnis`
- Zeile 34: `fn new`
- Zeile 38: `fn verarbeite_zu_tupel`
- Zeile 43: `fn verarbeite`
- Zeile 64: `fn verarbeite_automatische_spalten`
- Zeile 122: `fn suche_und_setze_spalten`
- Zeile 176: `fn setze_gefundene_spalten`
- Zeile 205: `fn fallback_zu_standards`
- Zeile 224: `fn zeige_alternative_kombinationen`

### `src/cli/bereich.rs`

**Umfang:** 64 Zeilen  
**Rolle:** Definiert die Kern-Konfigurationsstruktur `TextBereich` sowie `PypyCompatConfig`, also den gesamten Anfragezustand für einen Lauf.

**Wichtige Symbole:**

- Zeile 6: `struct PypyCompatConfig`
- Zeile 18: `struct TextBereich`
- Zeile 41: `fn default`

### `src/cli/mod.rs`

**Umfang:** 8 Zeilen  
**Rolle:** Projektdatei im `src`-Baum.

**Wichtige Symbole:** keine erkannten Rust-Top-Level-Symbole bzw. Nicht-Rust-Datei.

### `src/cli/parser.rs`

**Umfang:** 510 Zeilen  
**Rolle:** Zentrale CLI-Parserlogik. Liest Flags, Zeilenbereiche, Spaltennamen, Breitenvorgaben und Kompatibilitätsoptionen in `TextBereich` und `SpaltenNamen` ein.

**Wichtige Symbole:**

- Zeile 7: `struct SpaltenNamen`
- Zeile 13: `struct SpaltenNamenListe`
- Zeile 18: `fn default`
- Zeile 26: `fn is_flag`
- Zeile 30: `fn parse_pypy_number_set`
- Zeile 44: `fn parse_usize_csv_list`
- Zeile 65: `fn apply_pypy_compat_arg`
- Zeile 88: `fn print_all_oberkategorien`
- Zeile 110: `fn print_passende_unterkategorien`
- Zeile 141: `fn parse_cli_args`

**Kommentar:** Besonders wichtig sind hier `parse_cli_args()` als Haupteinstieg sowie die Hilfen für `--breite`, `--breiten`, `--spaltenreihenfolgeundnurdiese`, `--spaltenname` und PyPy-Kompatibilitätsargumente.

### `src/cli/utils.rs`

**Umfang:** 31 Zeilen  
**Rolle:** Projektdatei im `src`-Baum.

**Wichtige Symbole:**

- Zeile 7: `fn test_sortiere_und_fasse_zusammen`
- Zeile 14: `fn test_sortiere_und_fasse_zusammen_benachbart`
- Zeile 21: `fn test_sortiere_und_fasse_zusammen_leer`
- Zeile 26: `fn test_parse_einfache_zahl`

### `src/column_categories_complete.rs`

**Umfang:** 819 Zeilen  
**Rolle:** Enthält die große statische Kategoriendatenbank. Ordnet Ober-/Unterkategorien konkreten Spaltennummern zu und kann daraus SQL-Selektoren ableiten.

**Wichtige Symbole:**

- Zeile 6: `struct KategorieEintrag`
- Zeile 13: `fn new`
- Zeile 28: `struct KategorieMap`
- Zeile 34: `struct GeneratedInference`
- Zeile 40: `fn normalize_key`
- Zeile 48: `fn new`
- Zeile 56: `fn infer_generated_pair`
- Zeile 133: `fn finde_spaltennummern_exakt`
- Zeile 191: `fn finde_spaltennummern_fuer_kategorien`
- Zeile 250: `fn lade_kategorien`
- Zeile 704: `fn insert_entry`
- Zeile 729: `fn filtere_nach_spaltennummern`
- Zeile 743: `fn generiere_sql_selects`
- Zeile 817: `fn lade_kategorie_map`

**Kommentar:** Das Herzstück ist `lade_kategorien()`: Dort wird die semantische Spaltenlandkarte aufgebaut. `finde_spaltennummern_exakt()` und `finde_spaltennummern_fuer_kategorien()` sind die Kern-Nachschlagefunktionen.

### `src/column_manager/column_query_builder.rs`

**Umfang:** 26 Zeilen  
**Rolle:** Hilfsmodule für Spalten- und Zeilenabfragen: Namen auflösen, SQL-SELECT bauen und Eingaben validieren.

**Wichtige Symbole:**

- Zeile 6: `fn build_column_query`

### `src/column_manager/column_selector.rs`

**Umfang:** 65 Zeilen  
**Rolle:** Hilfsmodule für Spalten- und Zeilenabfragen: Namen auflösen, SQL-SELECT bauen und Eingaben validieren.

**Wichtige Symbole:**

- Zeile 4: `fn get_column_names`
- Zeile 12: `fn collect_spalten_nummern`
- Zeile 49: `fn resolve_spaltennamen`

### `src/column_manager/mod.rs`

**Umfang:** 8 Zeilen  
**Rolle:** Hilfsmodule für Spalten- und Zeilenabfragen: Namen auflösen, SQL-SELECT bauen und Eingaben validieren.

**Wichtige Symbole:** keine erkannten Rust-Top-Level-Symbole bzw. Nicht-Rust-Datei.

### `src/column_manager/row_query_builder.rs`

**Umfang:** 175 Zeilen  
**Rolle:** Hilfsmodule für Spalten- und Zeilenabfragen: Namen auflösen, SQL-SELECT bauen und Eingaben validieren.

**Wichtige Symbole:**

- Zeile 4: `fn build_row_query`
- Zeile 25: `fn build_query_with_continuous_range`
- Zeile 64: `fn build_query_with_row_ranges_enhanced`

### `src/column_manager/validation.rs`

**Umfang:** 120 Zeilen  
**Rolle:** Hilfsmodule für Spalten- und Zeilenabfragen: Namen auflösen, SQL-SELECT bauen und Eingaben validieren.

**Wichtige Symbole:**

- Zeile 5: `fn normalize_token`
- Zeile 22: `fn is_generated_pair_alias`
- Zeile 77: `fn validate_spalten_input_inner`
- Zeile 96: `fn validate_spalten_input`
- Zeile 104: `fn validate_spalten_input_with_pair`
- Zeile 114: `fn validate_spalten_input_with_generated`

### `src/column_manager/validation_generated_pair.rs`

**Umfang:** 120 Zeilen  
**Rolle:** Hilfsmodule für Spalten- und Zeilenabfragen: Namen auflösen, SQL-SELECT bauen und Eingaben validieren.

- Status: alternative/ältere Logik für Generated-Pair-Auflösung.

**Wichtige Symbole:**

- Zeile 5: `fn normalize_token`
- Zeile 22: `fn is_generated_pair_alias`
- Zeile 77: `fn validate_spalten_input_inner`
- Zeile 96: `fn validate_spalten_input`
- Zeile 104: `fn validate_spalten_input_with_pair`
- Zeile 114: `fn validate_spalten_input_with_generated`

### `src/csv_importer.rs`

**Umfang:** 104 Zeilen  
**Rolle:** Lädt die CSV-Quelldateien in eine in-memory SQLite-Datenbank und normalisiert Header sowie Zeilenlängen.

**Wichtige Symbole:**

- Zeile 7: `fn import_csvs_to_sqlite`

**Kommentar:** Praktisch wichtig: Die Funktion normalisiert doppelte/leere Header und füllt kurze Zeilen auf, damit spätere SQL-Abfragen stabil bleiben.

### `src/data.rs`

**Umfang:** 31 Zeilen  
**Rolle:** Kleines Beispielmodul für verschachtelte Datenelemente; wirkt im Hauptprogramm derzeit nicht zentral.

**Wichtige Symbole:**

- Zeile 2: `enum Element`
- Zeile 7: `fn text`
- Zeile 11: `fn list`
- Zeile 15: `fn create_example_structure`

### `src/data_fetcher.rs`

**Umfang:** 67 Zeilen  
**Rolle:** Führt SQL-Abfragen aus und liefert Tabelleninhalt inklusive einfacher Breitenstatistiken zurück.

**Wichtige Symbole:**

- Zeile 3: `fn fetch_data_with_stats`

### `src/exact_generator_bridge.rs`

**Umfang:** 98 Zeilen  
**Rolle:** Brücke zwischen exakt erkannten Generatorbegriffen und konkreten Spalten-/Generatorauflösungen.

**Wichtige Symbole:**

- Zeile 7: `struct ExactResolved`
- Zeile 14: `fn normalize_key`
- Zeile 22: `fn dedup_vec`
- Zeile 27: `fn push_unique`
- Zeile 33: `fn resolve_meta_konkret`
- Zeile 55: `fn resolve_eigenschaften_like`
- Zeile 80: `fn resolve_exact_generator`
- Zeile 94: `fn try_run_exact_generator_bridge`

### `src/generated_columns_words_registry.rs`

**Umfang:** 1461 Zeilen  
**Rolle:** Größtes Generatormodul. Erzeugt berechnete/abgeleitete Spalten aus Tabelleninhalten und aus begrifflichen Aliasen.

**Wichtige Symbole:**

- Zeile 12: `enum ST`
- Zeile 20: `struct Tables`
- Zeile 32: `struct ParametersMain`
- Zeile 39: `fn normalize_token`
- Zeile 43: `fn contains_any_alias`
- Zeile 49: `fn selected_by_pair`
- Zeile 57: `fn apply_generated_columns`
- Zeile 392: `struct SimpleFraction`
- Zeile 398: `fn new`
- Zeile 404: `fn mul`
- Zeile 408: `fn div`
- Zeile 413: `fn inv`
- Zeile 417: `fn is_integer`
- Zeile 422: `fn gcd`
- Zeile 431: `fn meta_pair_labels`
- Zeile 449: `fn make_prefix`
- Zeile 453: `fn lookup_universe_fraction`
- Zeile 471: `fn concat_universum_meta_konkret`
- Zeile 554: `fn find_header_index_casefold`
- Zeile 559: `fn cell_by_header`
- Zeile 567: `fn csv_path`
- Zeile 571: `fn read_semicolon_csv`
- Zeile 580: `fn transpose_csv`
- Zeile 592: `fn get_all_brueche`
- Zeile 608: `fn fraction_source_text`
- Zeile 653: `fn add_pair_unique`
- Zeile 661: `fn build_fraction_pairs_for_row`
- Zeile 739: `fn exact_gebr_prim_source`
- Zeile 760: `fn normalize_generated_operand`
- Zeile 773: `fn plain_prim_source`
- … insgesamt 58 erkannte Top-Level-Symbole.

**Kommentar:** Dieses Modul bündelt viele semantische Spezialfälle. Wegen Größe und Funktionsdichte ist es ein Hauptkandidat für spätere Aufteilung in kleinere Generator-Module.

### `src/i18n/update-script.sh`

**Umfang:** 5 Zeilen  
**Rolle:** Internationalisierungsdaten bzw. Skripte zur Generierung/Verarbeitung von Sprachressourcen.

**Wichtige Symbole:** keine erkannten Rust-Top-Level-Symbole bzw. Nicht-Rust-Datei.

### `src/i18n/words.py`

**Umfang:** 5055 Zeilen  
**Rolle:** Internationalisierungsdaten bzw. Skripte zur Generierung/Verarbeitung von Sprachressourcen.

**Wichtige Symbole:** keine erkannten Rust-Top-Level-Symbole bzw. Nicht-Rust-Datei.

### `src/i18n/words.rs`

**Umfang:** 179 Zeilen  
**Rolle:** Internationalisierungsdaten bzw. Skripte zur Generierung/Verarbeitung von Sprachressourcen.

**Wichtige Symbole:**

- Zeile 46: `fn wrong_lang_sentence`
- Zeile 73: `fn supported_languages`
- Zeile 77: `fn csv_for_language`
- Zeile 92: `enum Language`
- Zeile 132: `fn fmt`
- Zeile 137: `struct LanguageParseError`
- Zeile 142: `fn from_str`

### `src/i18n/words_1.rs`

**Umfang:** 248 Zeilen  
**Rolle:** Internationalisierungsdaten bzw. Skripte zur Generierung/Verarbeitung von Sprachressourcen.

**Wichtige Symbole:**

- Zeile 1: `fn get_paran_data_matrix`
- Zeile 69: `fn get_paran_data_matrix`
- Zeile 163: `fn fill_remaining_matrix`
- Zeile 217: `fn fill_final_parts_of_matrix`

### `src/if_is_zeilen_angabe/functions.rs`

**Umfang:** 152 Zeilen  
**Rolle:** Parser/Validatoren für Zeilenbereichs-Syntax und Generator-ähnliche Zahlenangaben.

**Wichtige Symbole:**

- Zeile 21: `fn is_zeilen_bruch_angabe_between_kommas`
- Zeile 26: `fn is_zeilen_bruch_or_ganz_zahl_angabe`
- Zeile 33: `fn is_zeilen_bruch_angabe`
- Zeile 43: `fn is_zeilen_angabe`
- Zeile 53: `fn is_zeilen_angabe_between_kommas`
- Zeile 59: `fn str_as_generator_to_vec_i64`
- Zeile 97: `fn str_as_generator_to_list_of_num_strs`
- Zeile 130: `fn is_zeilen_angabe_between_kommas_optimized`

### `src/if_is_zeilen_angabe/mod.rs`

**Umfang:** 7 Zeilen  
**Rolle:** Parser/Validatoren für Zeilenbereichs-Syntax und Generator-ähnliche Zahlenangaben.

**Wichtige Symbole:** keine erkannten Rust-Top-Level-Symbole bzw. Nicht-Rust-Datei.

### `src/if_is_zeilen_angabe/split.rs`

**Umfang:** 197 Zeilen  
**Rolle:** Parser/Validatoren für Zeilenbereichs-Syntax und Generator-ähnliche Zahlenangaben.

**Wichtige Symbole:**

- Zeile 5: `fn split_with_lookahead`
- Zeile 31: `fn has_unmatched_closing_bracket_ahead`
- Zeile 87: `fn split_with_bracket_balance`
- Zeile 122: `fn split_with_lookahead_optimized`

### `src/if_is_zeilen_angabe/validation.rs`

**Umfang:** 70 Zeilen  
**Rolle:** Parser/Validatoren für Zeilenbereichs-Syntax und Generator-ähnliche Zahlenangaben.

**Wichtige Symbole:**

- Zeile 5: `fn test_lookahead_implementation`
- Zeile 47: `fn test_split_functions`
- Zeile 56: `fn test_is_zeilen_bruch_angabe_between_kommas`
- Zeile 64: `fn test_is_zeilen_angabe`

### `src/input_help/bruch_validator.rs`

**Umfang:** 6 Zeilen  
**Rolle:** Kleinere Helfer und Tests für Eingabevalidierung, Musterdefinitionen und Generator-Parsing.

**Wichtige Symbole:**

- Zeile 4: `fn is_zeilen_bruch_angabe_between_kommas`

### `src/input_help/generator_parser.rs`

**Umfang:** 61 Zeilen  
**Rolle:** Kleinere Helfer und Tests für Eingabevalidierung, Musterdefinitionen und Generator-Parsing.

**Wichtige Symbole:**

- Zeile 2: `fn str_as_generator_to_list_of_num_strs`
- Zeile 34: `fn str_as_generator_to_list_of_num_strs_alt`

### `src/input_help/input_validation.rs`

**Umfang:** 56 Zeilen  
**Rolle:** Kleinere Helfer und Tests für Eingabevalidierung, Musterdefinitionen und Generator-Parsing.

**Wichtige Symbole:**

- Zeile 7: `fn is_zeilen_bruch_or_ganz_zahl_angabe`
- Zeile 13: `fn is_zeilen_bruch_angabe`
- Zeile 23: `fn is_zeilen_angabe`
- Zeile 34: `fn is_zeilen_angabe_between_kommas_optimized`

### `src/input_help/mod.rs`

**Umfang:** 7 Zeilen  
**Rolle:** Kleinere Helfer und Tests für Eingabevalidierung, Musterdefinitionen und Generator-Parsing.

**Wichtige Symbole:** keine erkannten Rust-Top-Level-Symbole bzw. Nicht-Rust-Datei.

### `src/input_help/pattern_definitions.rs`

**Umfang:** 16 Zeilen  
**Rolle:** Kleinere Helfer und Tests für Eingabevalidierung, Musterdefinitionen und Generator-Parsing.

**Wichtige Symbole:** keine erkannten Rust-Top-Level-Symbole bzw. Nicht-Rust-Datei.

### `src/input_help/tests.rs`

**Umfang:** 42 Zeilen  
**Rolle:** Kleinere Helfer und Tests für Eingabevalidierung, Musterdefinitionen und Generator-Parsing.

**Wichtige Symbole:**

- Zeile 6: `fn test_is_zeilen_bruch_angabe_between_kommas`
- Zeile 21: `fn test_is_zeilen_angabe_between_kommas`

### `src/input_help/zeilen_angabe_validator.rs`

**Umfang:** 9 Zeilen  
**Rolle:** Kleinere Helfer und Tests für Eingabevalidierung, Musterdefinitionen und Generator-Parsing.

**Wichtige Symbole:**

- Zeile 5: `fn is_zeilen_angabe_between_kommas`

### `src/kategorie_verarbeiter.rs`

**Umfang:** 216 Zeilen  
**Rolle:** Leitet aus Ober-/Unterkategorie Generatorbefehle ab und erweitert `TextBereich` um automatisch generierte Spaltenwünsche.

**Wichtige Symbole:**

- Zeile 5: `fn normalize_category_key`
- Zeile 12: `fn contains_any_alias`
- Zeile 17: `fn infer_generator_only_request`
- Zeile 161: `fn verarbeite_kategorien`

### `src/lib4tables_concat.rs`

**Umfang:** 1015 Zeilen  
**Rolle:** Großes Legacy-/Alternativmodul zur Erzeugung verketteter bzw. generierter Zusatzspalten; funktional verwandt mit `generated_columns_words_registry.rs`.

- Auffällig: sehr groß und vermutlich historisch gewachsen; überschneidet sich konzeptionell mit den neueren Generator-Modulen.

**Wichtige Symbole:**

- Zeile 9: `enum ST`
- Zeile 17: `struct Tables`
- Zeile 29: `struct ParametersMain`
- Zeile 36: `struct ConcatState`
- Zeile 46: `fn append_generated_col`
- Zeile 52: `fn current_new_col_index`
- Zeile 56: `fn register_generated_column`
- Zeile 74: `fn get_cell`
- Zeile 81: `fn join_nonempty`
- Zeile 88: `fn unique_preserve_order`
- Zeile 103: `fn primfaktoren`
- Zeile 116: `fn could_be_prime_number_primzahlkreuz`
- Zeile 119: `fn could_be_prime_number_primzahlkreuz_fuer_innen`
- Zeile 122: `fn could_be_prime_number_primzahlkreuz_fuer_aussen`
- Zeile 125: `fn prim_creativity`
- Zeile 128: `fn moon_number`
- Zeile 131: `fn prim_multiple`
- Zeile 135: `fn tagset`
- Zeile 143: `fn gleichheit_freiheit_vergleich`
- Zeile 174: `fn geist_emotion_energie_materie_topologie`
- Zeile 248: `fn concat_love_polygon`
- Zeile 284: `fn concat_gleichheit_freiheit_dominieren`
- Zeile 318: `fn concat_geist_emotion_energie_materie_topologie`
- Zeile 352: `fn concat_prim_creativity_type`
- Zeile 392: `fn concat_mond_exponzieren_logarithmus_typ`
- Zeile 470: `fn concat_vervielfache_zeile`
- Zeile 546: `struct ModalEntry`
- Zeile 555: `fn get_modaloperators_per_line_cells`
- Zeile 577: `fn prepare_modal_into_table`
- Zeile 616: `fn concat_modallogik`
- … insgesamt 31 erkannte Top-Level-Symbole.

**Kommentar:** Großes Altmodul mit viel Fachlogik; inhaltlich nützlich, architektonisch aber ein Indiz für historisch gewachsene Komplexität.

### `src/libs/center.py`

**Umfang:** 752 Zeilen  
**Rolle:** Hilfsbibliothek/Altbestand aus Python- bzw. Rust-Portierungen rund um Textausgabe, Wrapping und Generator-Utilities.

**Wichtige Symbole:** keine erkannten Rust-Top-Level-Symbole bzw. Nicht-Rust-Datei.

### `src/libs/center.rs`

**Umfang:** 199 Zeilen  
**Rolle:** Hilfsbibliothek/Altbestand aus Python- bzw. Rust-Portierungen rund um Textausgabe, Wrapping und Generator-Utilities.

**Wichtige Symbole:**

- Zeile 4: `fn getTextWrapThings`
- Zeile 19: `struct Html2Text`
- Zeile 23: `fn new`
- Zeile 28: `struct Pyphen`
- Zeile 32: `fn new`
- Zeile 42: `fn initShellRowsAmount`
- Zeile 53: `fn outputInfo`
- Zeile 64: `fn output`
- Zeile 75: `fn outputRaw`
- Zeile 86: `fn setInfo`
- Zeile 95: `fn setOutput`
- Zeile 104: `fn strAsGeneratorToListOfNumStrs`
- Zeile 124: `fn generatorToSortedUniqueIntVec`
- Zeile 139: `fn split_kommata_klammern_sicher`
- Zeile 149: `fn parseZeilenAngabe`
- Zeile 167: `fn ensurePP`
- Zeile 174: `fn initMultiplikationen`
- Zeile 186: `fn istMultiplikation`
- Zeile 197: `fn getShellRowsAmount`

### `src/main.rs`

**Umfang:** 26 Zeilen  
**Rolle:** Programm-Einstiegspunkt; registriert die Hauptmodule und delegiert vollständig an `workflows::main_workflow()`.

**Wichtige Symbole:**

- Zeile 24: `fn main`

### `src/multiples_teiler/bereichs_verarbeitung.rs`

**Umfang:** 91 Zeilen  
**Rolle:** Mathematische Hilfsfunktionen für Vielfache, Teiler, Primfaktoren und Bereichsauswertung.

**Wichtige Symbole:**

- Zeile 5: `fn simulate_bereich_to_numbers2`
- Zeile 21: `fn parse_number_range`
- Zeile 45: `fn try_parse_range`
- Zeile 65: `fn test_simulate_bereich_to_numbers2`
- Zeile 74: `fn test_parse_number_range`
- Zeile 84: `fn test_try_parse_range`

### `src/multiples_teiler/faktor_finder.rs`

**Umfang:** 107 Zeilen  
**Rolle:** Mathematische Hilfsfunktionen für Vielfache, Teiler, Primfaktoren und Bereichsauswertung.

**Wichtige Symbole:**

- Zeile 4: `fn multiples`
- Zeile 31: `fn find_all_divisors`
- Zeile 54: `fn find_factor_pairs`
- Zeile 63: `fn test_multiples`
- Zeile 89: `fn test_find_all_divisors`
- Zeile 101: `fn test_find_factor_pairs`

### `src/multiples_teiler/mod.rs`

**Umfang:** 6 Zeilen  
**Rolle:** Mathematische Hilfsfunktionen für Vielfache, Teiler, Primfaktoren und Bereichsauswertung.

**Wichtige Symbole:** keine erkannten Rust-Top-Level-Symbole bzw. Nicht-Rust-Datei.

### `src/multiples_teiler/teiler_rechner.rs`

**Umfang:** 129 Zeilen  
**Rolle:** Mathematische Hilfsfunktionen für Vielfache, Teiler, Primfaktoren und Bereichsauswertung.

**Wichtige Symbole:**

- Zeile 6: `fn teiler`
- Zeile 40: `fn teiler_enhanced`
- Zeile 68: `fn teiler_einzeln`
- Zeile 85: `fn test_teiler`
- Zeile 104: `fn test_teiler_single_number`
- Zeile 111: `fn test_teiler_enhanced`
- Zeile 124: `fn test_teiler_einzeln`

### `src/multiples_teiler/teiler_utils.rs`

**Umfang:** 146 Zeilen  
**Rolle:** Mathematische Hilfsfunktionen für Vielfache, Teiler, Primfaktoren und Bereichsauswertung.

**Wichtige Symbole:**

- Zeile 4: `fn hashset_to_sorted_strings`
- Zeile 17: `fn is_prime`
- Zeile 42: `fn gcd`
- Zeile 51: `fn lcm`
- Zeile 60: `fn prime_factors`
- Zeile 107: `fn test_is_prime`
- Zeile 121: `fn test_gcd`
- Zeile 130: `fn test_lcm`
- Zeile 139: `fn test_prime_factors`

### `src/prim.rs`

**Umfang:** 250 Zeilen  
**Rolle:** Eigenständige Primzahl-/Primfaktor-Experimente mit Tests und einer separaten `main()`-Funktion im Modul.

- Auffällig: enthält eine modulinterne `main()`, also eher Test-/Standalone-Code als integrierter Programmpfad.

**Wichtige Symbole:**

- Zeile 4: `fn primfaktoren`
- Zeile 36: `fn prim_repeat`
- Zeile 82: `fn prim_repeat2`
- Zeile 124: `fn prim_repeat2_alternative`
- Zeile 157: `fn test_primfaktoren`
- Zeile 165: `fn test_primfaktoren_modulo`
- Zeile 176: `fn test_prim_repeat`
- Zeile 196: `fn test_prim_repeat2`
- Zeile 214: `fn test_prim_repeat2_alternative`
- Zeile 228: `fn main`

### `src/pypy_compat.rs`

**Umfang:** 451 Zeilen  
**Rolle:** Erzeugt zusätzliche Hilfsdatenbanken aus CSV-Dateien und injiziert kompatible Zusatzspalten/Bruch- bzw. Kombi-Spalten in den Hauptdatenbestand.

**Wichtige Symbole:**

- Zeile 10: `struct PypyCompatDbs`
- Zeile 19: `fn read_csv_matrix`
- Zeile 35: `fn create_single_csv_db`
- Zeile 58: `fn load_table`
- Zeile 85: `fn rebuild_table`
- Zeile 110: `fn csv_path`
- Zeile 114: `fn parse_kombi_numbers`
- Zeile 136: `fn main_lookup`
- Zeile 147: `fn format_fraction_cell`
- Zeile 194: `fn append_fraction_columns`
- Zeile 237: `fn append_kombi_columns`
- Zeile 282: `fn build_extra_csv_dbs`
- Zeile 293: `fn collect_existing_selected_columns`
- Zeile 321: `fn apply_pypy_compat`

**Kommentar:** Die Datei erweitert die Haupttabelle nicht nur konzeptionell, sondern schreibt faktisch neue Header und Zellen in den SQLite-Datenbestand zurück.

### `src/python_exact_mappings.rs`

**Umfang:** 65 Zeilen  
**Rolle:** Reserviert exakte Python-Mappings bzw. Übergangslogik; in dieser Fassung praktisch leer.

- Status: Datei ist inhaltlich leer und dient derzeit eher als Platzhalter.

**Wichtige Symbole:** keine erkannten Rust-Top-Level-Symbole bzw. Nicht-Rust-Datei.

### `src/quellcodebeschreibung.txt`

**Umfang:** 50 Zeilen  
**Rolle:** Manuelle Kurznotiz zur bisherigen Architektur aus dem Projekt selbst.

**Wichtige Symbole:** keine erkannten Rust-Top-Level-Symbole bzw. Nicht-Rust-Datei.

### `src/reta.py`

**Umfang:** 1909 Zeilen  
**Rolle:** Große Python-Vorgängerversion bzw. Referenzimplementierung, aus der Teile nach Rust portiert wurden.

- Status: Python-Referenz; wichtig, um Rust-Portierung gegen Altverhalten zu vergleichen.

**Wichtige Symbole:** keine erkannten Rust-Top-Level-Symbole bzw. Nicht-Rust-Datei.

**Kommentar:** Auch wenn die Datei nicht im aktuellen Rust-Hauptpfad läuft, erklärt sie viel Fachlogik und ist für Regression-Checks wertvoll.

### `src/reta_ausgabe/cli_mod.rs`

**Umfang:** 12 Zeilen  
**Rolle:** Ältere bzw. alternative Ausgabeschicht mit `TableCell`/`TableRow`-Abstraktion, Syntax-Backends und Farb-/Wrap-Logik.

**Wichtige Symbole:** keine erkannten Rust-Top-Level-Symbole bzw. Nicht-Rust-Datei.

### `src/reta_ausgabe/cli_output.rs`

**Umfang:** 429 Zeilen  
**Rolle:** Ältere bzw. alternative Ausgabeschicht mit `TableCell`/`TableRow`-Abstraktion, Syntax-Backends und Farb-/Wrap-Logik.

**Wichtige Symbole:**

- Zeile 13: `struct CliOutput`
- Zeile 25: `fn new`
- Zeile 37: `fn is_perfect_power`
- Zeile 54: `fn is_prime`
- Zeile 75: `fn colorize`
- Zeile 120: `fn cliout2`
- Zeile 128: `fn effective_width_for_col`
- Zeile 135: `fn wrapped_cell_lines`
- Zeile 139: `fn row_wrapped_lines`
- Zeile 157: `fn visible_columns_for_row`
- Zeile 180: `fn cli_out`
- Zeile 316: `fn find_max_cell_text_len`
- Zeile 339: `fn create_test_table`
- Zeile 400: `fn create_simple_table`

### `src/reta_ausgabe/mod.rs`

**Umfang:** 12 Zeilen  
**Rolle:** Ältere bzw. alternative Ausgabeschicht mit `TableCell`/`TableRow`-Abstraktion, Syntax-Backends und Farb-/Wrap-Logik.

**Wichtige Symbole:** keine erkannten Rust-Top-Level-Symbole bzw. Nicht-Rust-Datei.

### `src/reta_ausgabe/output_syntax.rs`

**Umfang:** 61 Zeilen  
**Rolle:** Ältere bzw. alternative Ausgabeschicht mit `TableCell`/`TableRow`-Abstraktion, Syntax-Backends und Farb-/Wrap-Logik.

**Wichtige Symbole:**

- Zeile 5: `enum OutputSyntax`
- Zeile 16: `fn begin_table`
- Zeile 24: `fn end_table`
- Zeile 32: `fn generate_cell`
- Zeile 41: `fn end_cell`
- Zeile 49: `fn colored_begin_col`
- Zeile 53: `fn end_zeile`

### `src/reta_ausgabe/table_cell.rs`

**Umfang:** 58 Zeilen  
**Rolle:** Ältere bzw. alternative Ausgabeschicht mit `TableCell`/`TableRow`-Abstraktion, Syntax-Backends und Farb-/Wrap-Logik.

**Wichtige Symbole:**

- Zeile 5: `struct TableCell`
- Zeile 10: `fn new`
- Zeile 16: `fn get_line`
- Zeile 24: `fn line_count`
- Zeile 28: `fn get_line_width`
- Zeile 36: `struct TableRow`
- Zeile 43: `fn new`
- Zeile 51: `fn max_line_count`

### `src/reta_ausgabe/tables.rs`

**Umfang:** 40 Zeilen  
**Rolle:** Ältere bzw. alternative Ausgabeschicht mit `TableCell`/`TableRow`-Abstraktion, Syntax-Backends und Farb-/Wrap-Logik.

**Wichtige Symbole:**

- Zeile 5: `struct Tables`
- Zeile 15: `fn new`

### `src/reta_ausgabe/utils.rs`

**Umfang:** 108 Zeilen  
**Rolle:** Ältere bzw. alternative Ausgabeschicht mit `TableCell`/`TableRow`-Abstraktion, Syntax-Backends und Farb-/Wrap-Logik.

**Wichtige Symbole:**

- Zeile 5: `fn word_wrap`
- Zeile 94: `fn unicode_pad`

### `src/tabellen_utils.rs`

**Umfang:** 51 Zeilen  
**Rolle:** Hilfsroutinen für Testausgabe und CLI-Hilfetext.

**Wichtige Symbole:**

- Zeile 5: `fn test_simple_table`
- Zeile 44: `fn show_usage`

### `src/table_printer/config.rs`

**Umfang:** 50 Zeilen  
**Rolle:** Teil der Tabellen-Ausgabe-Pipeline: SQL-Ergebnis sortieren/aufbereiten, Spaltenbreiten berechnen und terminalgerechte Chunk-Ausgabe rendern.

**Wichtige Symbole:**

- Zeile 9: `enum ColumnKind`
- Zeile 19: `fn infer_from_header`
- Zeile 23: `fn min_width`
- Zeile 31: `fn soft_width`
- Zeile 39: `fn growth_weight`
- Zeile 47: `fn prefers_compact_layout`

### `src/table_printer/fixed_printer.rs`

**Umfang:** 424 Zeilen  
**Rolle:** Teil der Tabellen-Ausgabe-Pipeline: SQL-Ergebnis sortieren/aufbereiten, Spaltenbreiten berechnen und terminalgerechte Chunk-Ausgabe rendern.

**Wichtige Symbole:**

- Zeile 16: `fn get_explicit_width`
- Zeile 24: `fn filter_small_lines_in_cell`
- Zeile 34: `fn sanitize_chunk_data`
- Zeile 49: `fn row_has_visible_content`
- Zeile 57: `fn sanitize_chunk_data_with_rows`
- Zeile 84: `fn sanitize_header_preserve_id`
- Zeile 94: `fn print_table_chunked_with_line_numbers`
- Zeile 201: `fn build_output`
- Zeile 215: `fn render_rows`
- Zeile 231: `fn print_table`
- Zeile 235: `fn print_table_with_offset`
- Zeile 266: `fn estimate_natural_width_for_chunking`
- Zeile 287: `fn print_table_chunked`
- Zeile 304: `fn print_table_chunked_with_offset`
- Zeile 413: `fn print_table_auto`

### `src/table_printer/mod.rs`

**Umfang:** 10 Zeilen  
**Rolle:** Teil der Tabellen-Ausgabe-Pipeline: SQL-Ergebnis sortieren/aufbereiten, Spaltenbreiten berechnen und terminalgerechte Chunk-Ausgabe rendern.

**Wichtige Symbole:** keine erkannten Rust-Top-Level-Symbole bzw. Nicht-Rust-Datei.

### `src/table_printer/printer.rs`

**Umfang:** 673 Zeilen  
**Rolle:** Teil der Tabellen-Ausgabe-Pipeline: SQL-Ergebnis sortieren/aufbereiten, Spaltenbreiten berechnen und terminalgerechte Chunk-Ausgabe rendern.

**Wichtige Symbole:**

- Zeile 20: `fn get_explicit_width`
- Zeile 28: `fn effective_min_column_width`
- Zeile 32: `fn clamp_chunk_width`
- Zeile 36: `fn clamp_explicit_width`
- Zeile 40: `fn filter_small_lines_in_cell`
- Zeile 49: `fn sanitize_chunk_data`
- Zeile 64: `fn row_has_visible_content`
- Zeile 72: `fn sanitize_chunk_data_with_rows`
- Zeile 99: `fn sanitize_header_preserve_id`
- Zeile 109: `fn is_special_power`
- Zeile 135: `fn next_special_power`
- Zeile 146: `fn power_bucket_for_line`
- Zeile 162: `fn build_power_bucket_strings`
- Zeile 169: `fn estimate_natural_width_for_chunking`
- Zeile 190: `fn explicit_mask_for_range`
- Zeile 200: `fn shrink_widths_to_budget_preserving_explicit`
- Zeile 226: `fn stretch_last_non_explicit_or_last_column`
- Zeile 253: `fn determine_chunk_end`
- Zeile 299: `fn build_chunk_widths`
- Zeile 342: `fn build_meta_widths`
- Zeile 369: `fn prepend_meta_columns`
- Zeile 393: `fn stretch_last_column_to_fill_budget`
- Zeile 406: `fn print_table_chunked_with_line_numbers`
- Zeile 496: `fn build_output`
- Zeile 511: `fn render_rows`
- Zeile 532: `fn print_table`
- Zeile 541: `fn print_table_with_offset`
- Zeile 580: `fn print_table_chunked`
- Zeile 597: `fn print_table_chunked_with_offset`
- Zeile 661: `fn print_table_auto`

**Kommentar:** Diese Datei enthält die aktuelle Kernlogik für Chunkbildung, Breitenbudget, Metaspalten vor der Tabelle und das finale Rendering.

### `src/table_printer/query.rs`

**Umfang:** 421 Zeilen  
**Rolle:** Teil der Tabellen-Ausgabe-Pipeline: SQL-Ergebnis sortieren/aufbereiten, Spaltenbreiten berechnen und terminalgerechte Chunk-Ausgabe rendern.

**Wichtige Symbole:**

- Zeile 14: `fn build_original_line_numbers`
- Zeile 51: `fn expand_bereich_rows`
- Zeile 129: `fn normalize_token`
- Zeile 133: `fn contains_any_alias`
- Zeile 139: `fn selected_by_pair`
- Zeile 147: `fn should_use_full_table_for_generated`
- Zeile 261: `fn build_full_table_row_query`
- Zeile 311: `fn sanitize_headers`
- Zeile 326: `fn query_column_by_index`
- Zeile 406: `fn sort_by_indices`

**Kommentar:** `query_column_by_index()` ist der operative Knotenpunkt zwischen Datenbank, Spaltenreihenfolge, Generatorlogik, Sortierung und Druckfunktion.

### `src/table_printer/query_generated_pair.rs`

**Umfang:** 339 Zeilen  
**Rolle:** Teil der Tabellen-Ausgabe-Pipeline: SQL-Ergebnis sortieren/aufbereiten, Spaltenbreiten berechnen und terminalgerechte Chunk-Ausgabe rendern.

- Status: alternative/ältere Logik für Generated-Pair-Auflösung.

**Wichtige Symbole:**

- Zeile 12: `fn normalize_token`
- Zeile 25: `fn token_is`
- Zeile 30: `fn generated_alias_present`
- Zeile 43: `fn try_resolve_generated_pair`
- Zeile 160: `fn requires_full_table_for_generated`
- Zeile 173: `fn build_original_line_numbers`
- Zeile 211: `fn build_full_table_row_query`
- Zeile 221: `fn query_column_by_index`
- Zeile 309: `fn sort_by_indices`

### `src/table_printer/table_utils.rs`

**Umfang:** 489 Zeilen  
**Rolle:** Teil der Tabellen-Ausgabe-Pipeline: SQL-Ergebnis sortieren/aufbereiten, Spaltenbreiten berechnen und terminalgerechte Chunk-Ausgabe rendern.

**Wichtige Symbole:**

- Zeile 9: `fn convert_to_table_rows_with_line_numbers`
- Zeile 38: `struct ColumnStats`
- Zeile 48: `struct TableLayout`
- Zeile 54: `fn get_terminal_width`
- Zeile 60: `fn compute_max_lengths`
- Zeile 104: `fn compute_column_stats`
- Zeile 168: `fn compute_column_widths_from_global_mass`
- Zeile 216: `fn compute_column_widths_linear_natural`
- Zeile 252: `fn shrink_widths_to_fit_budget`
- Zeile 276: `fn compute_column_widths_optimized`
- Zeile 288: `fn compute_column_widths`
- Zeile 304: `fn compute_columns_per_table_from_widths`
- Zeile 335: `fn compute_columns_per_table`
- Zeile 352: `fn build_table_layout`
- Zeile 369: `fn normalize_row`
- Zeile 375: `fn row_numbers_for_data_len`
- Zeile 387: `fn build_header_row`
- Zeile 400: `fn build_data_row`
- Zeile 420: `fn convert_to_table_rows`
- Zeile 429: `fn convert_to_table_rows_with_offset`
- Zeile 474: `fn chunk_bounds`

### `src/watchLater/dual_cli_ncurses`

**Umfang:** 1321 Zeilen  
**Rolle:** Backup-, Zwischenstands- oder experimentelle Datei; nützlich zum Vergleich, aber nicht zwingend Teil des aktuellen Hauptpfads.

**Wichtige Symbole:** keine erkannten Rust-Top-Level-Symbole bzw. Nicht-Rust-Datei.

### `src/watchLater/dual_cli_ncurses.cpp`

**Umfang:** 71 Zeilen  
**Rolle:** Backup-, Zwischenstands- oder experimentelle Datei; nützlich zum Vergleich, aber nicht zwingend Teil des aktuellen Hauptpfads.

**Wichtige Symbole:** keine erkannten Rust-Top-Level-Symbole bzw. Nicht-Rust-Datei.

### `src/workflows.rs`

**Umfang:** 58 Zeilen  
**Rolle:** Orchestriert den kompletten CLI-Lauf: Argumente lesen, Kategorien laden, CSV-Dateien importieren, optionale PyPy-Kompatibilität anwenden und die finale Tabellenabfrage/Anzeige starten.

**Wichtige Symbole:**

- Zeile 14: `fn main_workflow`

**Kommentar:** Hier sieht man den echten Programmablauf am kompaktesten; für Onboarding ist diese Datei der beste Startpunkt.

## 4. Wichtigste aktuelle Programmpfade

### Start und Orchestrierung

- `main.rs` → `workflows::main_workflow()`
- `main_workflow()` lädt die Kategoriemap, verarbeitet Argumente, importiert CSV-Dateien und startet die Ausgabe.

### Spaltenauflösung

- `cli/parser.rs` erzeugt `TextBereich` und `SpaltenNamen`.
- `argument_verarbeiter.rs` und `kategorie_verarbeiter.rs` füllen daraus konkrete Spaltennummern und Generatorbefehle.
- `column_categories_complete.rs` ist die semantische Wissensbasis dafür.

### Datenimport und Datenanreicherung

- `csv_importer.rs` baut `csv_data` in SQLite auf.
- `pypy_compat.rs` und `generated_columns_words_registry.rs` hängen weitere Inhalte an.

### Anzeige

- `table_printer/query.rs` liest Daten in gewünschter Reihenfolge.
- `table_printer/table_utils.rs` berechnet Layout/Breiten.
- `table_printer/printer.rs` chunked die Tabelle.
- `reta_ausgabe/*` liefert die eigentlichen Renderbausteine.

## 5. Auffälligkeiten und technische Schulden

- Mehrere Dateien existieren in Parallelvarianten (`.bak`, `generated_pair`, `old`). Das erschwert die mentale Karte des Systems.
- Es gibt mindestens drei Logikschichten, die sich überlappen: Kategorienauflösung, Generatorauflösung und PyPy-Kompatibilitätsanreicherung. Diese sind fachlich verwandt, aber auf mehrere große Dateien verteilt.
- `generated_columns_words_registry.rs` und `lib4tables_concat.rs` sind sehr groß und mischen Domänenlogik, String-Alias-Erkennung und Datenmanipulation.
- Die Existenz von `reta.py` zeigt, dass ein Teil der Wahrheit des Systems historisch noch in Python steckt. Für vollständige Wartbarkeit wäre eine klare Referenzstrategie sinnvoll: Entweder Python bleibt offiziell Goldstandard oder Rust übernimmt sie vollständig.

## 6. Empfohlene Lesereihenfolge

1. `src/main.rs`
2. `src/workflows.rs`
3. `src/cli/bereich.rs` und `src/cli/parser.rs`
4. `src/argument_verarbeiter.rs`
5. `src/column_categories_complete.rs`
6. `src/csv_importer.rs`
7. `src/pypy_compat.rs`
8. `src/table_printer/query.rs`
9. `src/table_printer/printer.rs`
10. danach erst die großen Spezialmodule wie `generated_columns_words_registry.rs`, `lib4tables_concat.rs` und `reta.py`.

## 7. Fazit

Der `src`-Baum zeigt ein funktional reiches, aber historisch geschichtetes System. Die aktuelle produktive Linie ist klar erkennbar: CLI → Kategorien/Generatoren → SQLite → Druckpipeline. Gleichzeitig liegen viele Spuren einer laufenden Portierung und mehrerer Evolutionsstufen im Quellbaum. Genau deshalb ist die beste Wartungsstrategie nicht bloß „mehr Kommentare“, sondern eine stärkere Trennung von aktueller Kernlogik, Legacy-Code und experimentellen Pfaden.
