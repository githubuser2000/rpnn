# libreta_table.so — Tabellen, View, Garbe

## Zweck

Grenze für Tabellenmaterialisierung, Tabellenzustand, View-Aufbau, Adapter und das Zusammenführen lokaler semantischer Sektionen.

## Direkte Einbindung

Direkter Nutzer dieser Bibliothek: `libreta.so`.

Direkte Zielabhängigkeiten: keine direkte private Pflichtabhängigkeit innerhalb dieser Ebene.

## Architekturgrenze

Diese `.so` ist eine bewusste ABI-Grenze. Rust-interne Typen sollen nicht über diese Grenze wandern. Nach außen werden stabile C-Symbole, einfache Zahlenwerte und nullterminierte Zeichenketten exportiert. Das hält die Topologie stabil, auch wenn später mehr Rust-Code aus `libreta.so` in diese Bibliothek verschoben wird.

## Mathematische Rolle

Garbe: kompatible lokale Spalten-/Zeilen-/Parameter-Sektionen werden zu einer globalen Tabelle verklebt.

## Wichtige ABI-Symbole

- `reta_table_abi_version`
- `reta_table_abi_anchor`
- `reta_table_abi_manifest_json`
- `reta_table_abi_role_de`
- `reta_table_abi_role_en`


## Reale Code-Auslagerung

Diese Bibliothek enthält jetzt konkrete Tabellen-Hilfslogik zur Breitenberechnung und Budget-Schrumpfung. Die vollständige Tabellenmaterialisierung kann hinter derselben Grenze weiter verschoben werden.

Zusätzliche Symbole:

- `reta_table_natural_widths_json`
- `reta_table_shrink_widths_json`
- `reta_table_free_string`

## Build-Regel

`build.sh` baut zuerst die privaten Core-Bibliotheken und danach `libreta.so` mit `RETA_LINK_CORE_SPLIT_LIBS=1`. Dadurch muss `rreta` nur direkt gegen `libreta.so` linken; die privaten Core-Bibliotheken erscheinen als `DT_NEEDED` von `libreta.so`.

## Nicht-Ziel

Diese Bibliothek soll keine zweite öffentliche Programmschnittstelle neben `libreta.so` werden. Die öffentliche Programmausführung bleibt über die Fassade stabil.
