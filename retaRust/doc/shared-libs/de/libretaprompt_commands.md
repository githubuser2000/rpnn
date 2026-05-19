# libretaprompt_commands.so — Prompt-Kommandos

## Zweck

Befehlsbibliothek für `rrpb` und die Kommando-Seite von `rrp`, `rrpl` und `rrpe`. `rrpb` verwendet nur diese Prompt-Bibliothek.

## Direkte Einbindung

Direkter Nutzer dieser Bibliothek: `rrpb, rrp, rrpl, rrpe`.

Direkte Zielabhängigkeiten: keine direkte private Pflichtabhängigkeit innerhalb dieser Ebene.

## Architekturgrenze

Diese `.so` ist eine bewusste ABI-Grenze. Rust-interne Typen sollen nicht über diese Grenze wandern. Nach außen werden stabile C-Symbole, einfache Zahlenwerte und nullterminierte Zeichenketten exportiert. Das hält die Topologie stabil, auch wenn später mehr Rust-Code aus `libreta.so` in diese Bibliothek verschoben wird.

## Mathematische Rolle

Morphismen von Prompt-Text in ausführbare Reta-Kommandos; keine Eingabezeilen-UI, kein Autocomplete, kein Autosuggest.

## Wichtige ABI-Symbole

- `retaprompt_commands_run_kind_from_env`
- `retaprompt_commands_run_current_executable_from_env`
- `retaprompt_commands_run_rp_from_env`
- `retaprompt_commands_run_rpl_from_env`
- `retaprompt_commands_run_rpb_from_env`
- `retaprompt_commands_run_rpe_from_env`

## Build-Regel

`build.sh` baut zuerst die privaten Core-Bibliotheken und danach `libreta.so` mit `RETA_LINK_CORE_SPLIT_LIBS=1`. Dadurch muss `rreta` nur direkt gegen `libreta.so` linken; die privaten Core-Bibliotheken erscheinen als `DT_NEEDED` von `libreta.so`.

## Nicht-Ziel

Diese Bibliothek soll keine zweite öffentliche Programmschnittstelle neben `libreta.so` werden. Die öffentliche Programmausführung bleibt über die Fassade stabil.
