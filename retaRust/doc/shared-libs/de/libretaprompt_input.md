# libretaprompt_input.so — Prompt-Eingabe, Autocomplete, Autosuggest

## Zweck

Interaktive Eingabebibliothek für `rrp`, `rrpl` und `rrpe`. Sie kapselt Zeileneingabe, Autocomplete, Autosuggest, Verlauf und interaktive Profile.

## Direkte Einbindung

Direkter Nutzer dieser Bibliothek: `rrp, rrpl, rrpe`.

Direkte Zielabhängigkeiten: `libretaprompt_commands.so`.

## Architekturgrenze

Diese `.so` ist eine bewusste ABI-Grenze. Rust-interne Typen sollen nicht über diese Grenze wandern. Nach außen werden stabile C-Symbole, einfache Zahlenwerte und nullterminierte Zeichenketten exportiert. Das hält die Topologie stabil, auch wenn später mehr Rust-Code aus `libreta.so` in diese Bibliothek verschoben wird.

## Mathematische Rolle

Bidirektionaler Kanal zwischen Nutzerzustand und Prompt-Zustand; Vervollständigung ist eine lokale Auswahl über dem aktuellen Token-Kontext.

## Wichtige ABI-Symbole

- `retaprompt_input_run_kind_from_env`
- `retaprompt_input_run_current_executable_from_env`
- `retaprompt_input_run_any_current_executable_from_env`
- `retaprompt_input_run_launcher_kind_from_env`
- `retaprompt_input_run_rp_from_env`
- `retaprompt_input_run_rpl_from_env`
- `retaprompt_input_run_rpe_from_env`

## Build-Regel

`build.sh` baut zuerst die privaten Core-Bibliotheken und danach `libreta.so` mit `RETA_LINK_CORE_SPLIT_LIBS=1`. Dadurch muss `rreta` nur direkt gegen `libreta.so` linken; die privaten Core-Bibliotheken erscheinen als `DT_NEEDED` von `libreta.so`.

## Nicht-Ziel

Diese Bibliothek soll keine zweite öffentliche Programmschnittstelle neben `libreta.so` werden. Die öffentliche Programmausführung bleibt über die Fassade stabil.
