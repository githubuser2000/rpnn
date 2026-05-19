# libreta_runtime.so — Reta-Engine, Netzwerk und Scheduler

## Zweck

`libreta_runtime.so` trägt in der Split-Build-Variante den schweren nicht-interaktiven Reta-Core. Dazu gehören Programmausführung, Workflow, Tabellenaufbau, Ausgabeerzeugung, Architektur-Shadow-Pfade, Cache und Runtime-Brücken. `libreta.so` bleibt dadurch eine kleine Fassade.

## Direkte Einbindung

Direkter Nutzer dieser Bibliothek: `libreta.so`.

Direkte Zielabhängigkeiten: keine private Pflichtabhängigkeit innerhalb dieser Ebene. Die übrigen privaten Core-Bibliotheken bleiben über `libreta.so` als Topologieanker sichtbar.

## Architekturgrenze

Diese `.so` besitzt zwei ABI-Schichten:

1. öffentliche Metadaten-Symbole wie `reta_runtime_abi_anchor`,
2. private Engine-Symbole mit Prefix `reta_runtime_core_*`, die nur von `libreta.so` benutzt werden sollen.

Externe Programme sollen weiterhin `include/reta.h` und `libreta.so` verwenden, nicht direkt diese Engine-ABI. Der Runtime-Carrier kompiliert die alten öffentlichen `reta_*`-Engine-Symbole intern ohne `no_mangle`, damit `libreta.so` nicht durch Symbol-Interposition in Rekursion geraten kann.

## Mathematische Rolle

Netzwerk: Aufgaben sind Knoten oder Kanten, Queues bestimmen Ordnung, Semaphore begrenzen Ressourcen, Reduktion hält Ausgabe deterministisch. Kategorial betrachtet ist diese Library der Morphismus-Träger, durch den die universelle Fassade konkrete Programmausführung faktorisiert.

## Wichtige ABI-Symbole

- `reta_runtime_abi_version`
- `reta_runtime_abi_anchor`
- `reta_runtime_abi_manifest_json`
- `reta_runtime_core_run_and_print_from_env_ffi`
- `reta_runtime_core_run_argv`
- `reta_runtime_core_free_string`
- `reta_runtime_core_shared_words_json`
- `reta_runtime_core_all_main_alias_groups_json`
- `reta_runtime_core_parameter_alias_groups_for_main_json`


## Neue Komponenten-Abhängigkeiten

`libreta_runtime.so` verlinkt jetzt zusätzlich gegen `libreta_data.so`, `libreta_parse.so`, `libreta_semantics.so`, `libreta_table.so`, `libreta_render.so` und `libreta_arch.so`. Der Runtime-Anker ruft deren ABI-Anker auf, damit die `DT_NEEDED`-Topologie nicht wieder zu leeren Stubs degeneriert.

## Build-Regel

`build.sh` baut diese Library vor `libreta.so`. Danach wird `libreta.so` mit `--features split-facade` gegen die prefixed Runtime-Core-Symbole gelinkt.

## Größenregel

Diese Library soll größer als `libreta.so` sein, weil sie den schweren Engine-Code trägt. Die feinere spätere Verteilung nach `data`, `parse`, `semantics`, `table` und `render` kann hinter derselben Topologie fortgesetzt werden.
