# libreta.so — dünne stabile Reta-Fassade

## Zweck

`libreta.so` ist die öffentliche, stabile C-ABI-Fassade für `rreta` und für externe Nutzer der Reta-ABI. In der Split-Build-Variante enthält diese Bibliothek nicht mehr den schweren Reta-Engine-Code. Sie exportiert die bekannten öffentlichen Symbole und leitet die eigentliche Ausführung an `libreta_runtime.so` weiter.

## Direkte Einbindung

Direkter Nutzer dieser Bibliothek: `rreta`.

Direkte Zielabhängigkeiten: `libreta_data.so`, `libreta_parse.so`, `libreta_semantics.so`, `libreta_table.so`, `libreta_render.so`, `libreta_arch.so`, `libreta_runtime.so`.

## Architekturgrenze

Diese `.so` ist eine bewusste ABI-Fassade. Rust-interne Typen wandern nicht über diese Grenze. Nach außen bleiben stabile C-Symbole, einfache Zahlenwerte, C-Strings und `RetaFfiResponse` sichtbar. Intern wird über prefixed Runtime-Symbole wie `reta_runtime_core_run_argv` an `libreta_runtime.so` delegiert.

## Mathematische Rolle

Universelle Eigenschaft: alle nicht-interaktiven Frontends faktorisieren durch denselben kanonischen Weg `argv/stdin -> RetaRequest -> RetaResponse -> Output`. Die Fassade bleibt das universelle Objekt; die Runtime-Engine ist der faktorisierte Morphismus-Träger.

## Wichtige ABI-Symbole

- `reta_run_and_print_from_env_ffi`
- `reta_abi_version`
- `reta_run_argv`
- `reta_free_string`
- `reta_shared_words_json`
- `reta_all_main_alias_groups_json`
- `reta_parameter_alias_groups_for_main_json`
- `reta_core_split_abi_anchor`
- `reta_core_split_abi_manifest_json`
- `reta_core_split_abi_is_linked`

## Build-Regel

`build.sh` baut zuerst die privaten Core-Bibliotheken und danach `libreta.so` mit `--features split-facade` und `RETA_LINK_CORE_SPLIT_LIBS=1`. Dadurch muss `rreta` nur direkt gegen `libreta.so` linken, während `libreta.so` die privaten Core-Bibliotheken als `DT_NEEDED` trägt.

## Größenregel

`libreta.so` muss kleiner als `libreta_runtime.so` sein. Das Build-Skript bricht ab, wenn diese Regel verletzt wird. So wird verhindert, dass der schwere Engine-Code versehentlich zurück in die Fassade wandert.
