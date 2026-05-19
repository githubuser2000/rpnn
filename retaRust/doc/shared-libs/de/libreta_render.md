# libreta_render.so — Rendering-Funktoren

## Zweck

Grenze für Shell/Text, HTML, BBCode, Layout, Wrapping, Nummerierung, Styles und Ausgabeformate.

## Direkte Einbindung

Direkter Nutzer dieser Bibliothek: `libreta.so`.

Direkte Zielabhängigkeiten: keine direkte private Pflichtabhängigkeit innerhalb dieser Ebene.

## Architekturgrenze

Diese `.so` ist eine bewusste ABI-Grenze. Rust-interne Typen sollen nicht über diese Grenze wandern. Nach außen werden stabile C-Symbole, einfache Zahlenwerte und nullterminierte Zeichenketten exportiert. Das hält die Topologie stabil, auch wenn später mehr Rust-Code aus `libreta.so` in diese Bibliothek verschoben wird.

## Mathematische Rolle

Funktor: dieselbe semantische Tabelle wird in konkrete Darstellungsräume abgebildet, ohne die Semantik selbst zu verändern.

## Wichtige ABI-Symbole

- `reta_render_abi_version`
- `reta_render_abi_anchor`
- `reta_render_abi_manifest_json`
- `reta_render_abi_role_de`
- `reta_render_abi_role_en`


## Reale Code-Auslagerung

Diese Bibliothek trägt jetzt die echte `grundStrukHtml`-HTML-Erzeugung. `rgrundStrukHtml` wird deshalb als kleiner C-Launcher gebaut und ruft direkt `libreta_render.so` auf, statt den schweren Rust-Core in das Executable einzubetten.

Zusätzliche Symbole:

- `reta_render_grundstruk_html`
- `reta_render_grundstruk_html_len`
- `reta_render_free_string`

## Build-Regel

`build.sh` baut zuerst die privaten Core-Bibliotheken und danach `libreta.so` mit `RETA_LINK_CORE_SPLIT_LIBS=1`. Dadurch muss `rreta` nur direkt gegen `libreta.so` linken; die privaten Core-Bibliotheken erscheinen als `DT_NEEDED` von `libreta.so`.

## Nicht-Ziel

Diese Bibliothek soll keine zweite öffentliche Programmschnittstelle neben `libreta.so` werden. Die öffentliche Programmausführung bleibt über die Fassade stabil.

## Abhängigkeit zu libreta_semantics.so

`libreta_render.so` wird im Shared-Library-Build gegen `libreta_semantics.so` gelinkt. Damit ist die semantische Inventargrenze auch für `rgrundStrukHtml` Teil der dynamischen Topologie: `rgrundStrukHtml -> libreta_render.so -> libreta_semantics.so`.
