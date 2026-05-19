# libreta_data.so — Daten und Kataloge

## Zweck

Grenze für Wörter, Aliase, CSV-/HTML-Kataloge, statische Tabellen, Sprachwerte und Datenquellen. Diese Bibliothek ist der Ort, an den Datenzugriff und unveränderliche Kataloglogik hinter die Fassade verschoben werden.

## Direkte Einbindung

Direkter Nutzer dieser Bibliothek: `libreta.so`.

Direkte Zielabhängigkeiten: keine direkte private Pflichtabhängigkeit innerhalb dieser Ebene.

## Architekturgrenze

Diese `.so` ist eine bewusste ABI-Grenze. Rust-interne Typen sollen nicht über diese Grenze wandern. Nach außen werden stabile C-Symbole, einfache Zahlenwerte und nullterminierte Zeichenketten exportiert. Das hält die Topologie stabil, auch wenn später mehr Rust-Code aus `libreta.so` in diese Bibliothek verschoben wird.

## Mathematische Rolle

Relationale Basis: Objekte sind Wörter, Aliase, CSV-Zeilen, Spalten und Katalogeinträge; Morphismen sind Lookup, Normalisierung und Projektion.

## Wichtige ABI-Symbole

- `reta_data_abi_version`
- `reta_data_abi_anchor`
- `reta_data_abi_manifest_json`
- `reta_data_abi_role_de`
- `reta_data_abi_role_en`


## Reale Code-Auslagerung

Diese Bibliothek ist nicht mehr nur ein ABI-Anker. Sie enthält jetzt die generierte `Words`-Datenstruktur und Alias-Projektionen aus der Python-Source-of-Truth-Logik. Dadurch darf sie nicht mehr dieselbe Größe wie die anderen kleinen Stub-Komponenten haben.

Zusätzliche Symbole:

- `reta_data_words_entry_count`
- `reta_data_shared_words_json`
- `reta_data_all_main_alias_groups_json`
- `reta_data_parameter_alias_groups_for_main_json`
- `reta_data_resolve_parameter_main_alias`
- `reta_data_free_string`

## Build-Regel

`build.sh` baut zuerst die privaten Core-Bibliotheken und danach `libreta.so` mit `RETA_LINK_CORE_SPLIT_LIBS=1`. Dadurch muss `rreta` nur direkt gegen `libreta.so` linken; die privaten Core-Bibliotheken erscheinen als `DT_NEEDED` von `libreta.so`.

## Nicht-Ziel

Diese Bibliothek soll keine zweite öffentliche Programmschnittstelle neben `libreta.so` werden. Die öffentliche Programmausführung bleibt über die Fassade stabil.
