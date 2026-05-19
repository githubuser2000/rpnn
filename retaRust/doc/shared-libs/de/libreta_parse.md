# libreta_parse.so — Parsing und Input-Morphismen

## Zweck

Grenze für Kommandozeilen-Parsing, Textzerlegung, Aliasauflösung, Parameter-Vorbereitung und Prompt-Token-Übersetzung.

## Direkte Einbindung

Direkter Nutzer dieser Bibliothek: `libreta.so`.

Direkte Zielabhängigkeiten: keine direkte private Pflichtabhängigkeit innerhalb dieser Ebene.

## Architekturgrenze

Diese `.so` ist eine bewusste ABI-Grenze. Rust-interne Typen sollen nicht über diese Grenze wandern. Nach außen werden stabile C-Symbole, einfache Zahlenwerte und nullterminierte Zeichenketten exportiert. Das hält die Topologie stabil, auch wenn später mehr Rust-Code aus `libreta.so` in diese Bibliothek verschoben wird.

## Mathematische Rolle

Morphismenfamilie von rohem Text oder `argv` in eine kanonische Anfrage. Die Bibliothek soll keine Tabellen rendern und keine globalen Daten verändern.

## Wichtige ABI-Symbole

- `reta_parse_abi_version`
- `reta_parse_abi_anchor`
- `reta_parse_abi_manifest_json`
- `reta_parse_abi_role_de`
- `reta_parse_abi_role_en`


## Reale Code-Auslagerung

Diese Bibliothek enthält jetzt den Shell-/Prompt-Tokenizer `split_shell_like`. Das ist die erste echte Parsing-Funktion hinter dieser ABI-Grenze.

Zusätzliche Symbole:

- `reta_parse_shell_token_count`
- `reta_parse_shell_tokens_json`
- `reta_parse_free_string`

## Build-Regel

`build.sh` baut zuerst die privaten Core-Bibliotheken und danach `libreta.so` mit `RETA_LINK_CORE_SPLIT_LIBS=1`. Dadurch muss `rreta` nur direkt gegen `libreta.so` linken; die privaten Core-Bibliotheken erscheinen als `DT_NEEDED` von `libreta.so`.

## Nicht-Ziel

Diese Bibliothek soll keine zweite öffentliche Programmschnittstelle neben `libreta.so` werden. Die öffentliche Programmausführung bleibt über die Fassade stabil.
