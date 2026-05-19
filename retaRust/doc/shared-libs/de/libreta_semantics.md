# libreta_semantics.so — Semantik, Topologie, Prägarbe

## Zweck

Grenze für Spaltenauswahl, Zeilenfilter, Generatorwahl, Zahlenlogik, Tags und semantische Verdichtung von Parametern.

## Direkte Einbindung

Direkter Nutzer dieser Bibliothek: `libreta.so`.

Direkte Zielabhängigkeiten: keine direkte private Pflichtabhängigkeit innerhalb dieser Ebene.

## Architekturgrenze

Diese `.so` ist eine bewusste ABI-Grenze. Rust-interne Typen sollen nicht über diese Grenze wandern. Nach außen werden stabile C-Symbole, einfache Zahlenwerte und nullterminierte Zeichenketten exportiert. Das hält die Topologie stabil, auch wenn später mehr Rust-Code aus `libreta.so` in diese Bibliothek verschoben wird.

## Mathematische Rolle

Topologie und Prägarbe: lokale Parameterinformationen werden als Sektionen betrachtet, deren Nähe, Abschluss und Kompatibilität bestimmt werden.

## Wichtige ABI-Symbole

- `reta_semantics_abi_version`
- `reta_semantics_abi_anchor`
- `reta_semantics_abi_manifest_json`
- `reta_semantics_abi_role_de`
- `reta_semantics_abi_role_en`


## Reale Code-Auslagerung

Diese Bibliothek enthält jetzt die semantischen Wahl- und Prompt-Auswahlinventare (`WAHL15`, `WAHL16`, Hauptschalter, Sektionsschalter).

Zusätzliche Symbole:

- `reta_semantics_choice_counts_json`
- `reta_semantics_wahl15_value`
- `reta_semantics_wahl16_value`
- `reta_semantics_free_string`

## Build-Regel

`build.sh` baut zuerst die privaten Core-Bibliotheken und danach `libreta.so` mit `RETA_LINK_CORE_SPLIT_LIBS=1`. Dadurch muss `rreta` nur direkt gegen `libreta.so` linken; die privaten Core-Bibliotheken erscheinen als `DT_NEEDED` von `libreta.so`.

## Nicht-Ziel

Diese Bibliothek soll keine zweite öffentliche Programmschnittstelle neben `libreta.so` werden. Die öffentliche Programmausführung bleibt über die Fassade stabil.
