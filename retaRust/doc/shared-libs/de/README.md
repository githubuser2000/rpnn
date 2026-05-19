# Reta Shared-Library-Topologie

Diese Dokumentation beschreibt jede gebaute `.so`-Bibliothek auf Deutsch.

## Zielstruktur

```text
rreta
  -> libreta.so
       -> libreta_data.so
       -> libreta_parse.so
       -> libreta_semantics.so
       -> libreta_table.so
       -> libreta_render.so
       -> libreta_arch.so
       -> libreta_runtime.so

rrp / rrpl / rrpe
  -> libretaprompt_input.so
  -> libretaprompt_commands.so

rrpb
  -> libretaprompt_commands.so
```

## Grundregel

Die Executables bleiben klein. Programmlogik liegt in `.so`-Bibliotheken. `libreta.so` ist jetzt bewusst nur die stabile, dünne Fassade; der schwere nicht-interaktive Reta-Kern liegt in `libreta_runtime.so` und wird über private `reta_runtime_core_*`-Symbole erreicht. Die übrigen privaten Core-Bibliotheken bilden die explizite interne Topologie und können später weiter mit konkreter Fachlogik gefüllt werden.

## Größenregel

`libreta.so` muss kleiner als `libreta_runtime.so` sein. Die Build-Skripte brechen ab, wenn `libreta.so` wieder zum schweren Engine-Träger wird.

## Einzeldokumente

- [libreta.so](libreta.md)
- [libreta_data.so](libreta_data.md)
- [libreta_parse.so](libreta_parse.md)
- [libreta_semantics.so](libreta_semantics.md)
- [libreta_table.so](libreta_table.md)
- [libreta_render.so](libreta_render.md)
- [libreta_arch.so](libreta_arch.md)
- [libreta_runtime.so](libreta_runtime.md)
- [libretaprompt_commands.so](libretaprompt_commands.md)
- [libretaprompt_input.so](libretaprompt_input.md)
