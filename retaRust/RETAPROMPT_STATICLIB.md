# reta prompt split static libraries

Ziel ist **genau drei** statische Archive ohne gegenseitigen Implementierungs-Doppelinhalt:

- `libreta.a`
- `libretaprompt_input.a`
- `libretaprompt_commands.a`

## Fachliche Trennung

### `libreta.a`
Trägt die eigentliche Rust-Implementierung und bleibt die Kernbibliothek.

### `libretaprompt_input.a`
Thematisiert nur die **eigene Befehlseingabe** für:

- `rp`
- `rpl`
- `rpe`

### `libretaprompt_commands.a`
Thematisiert nur die **Befehlsseite** für:

- `rp`
- `rpl`
- `rpe`
- `rpb`

## Warum die zwei Zusatzarchive keine Rust-`staticlib`-Crates sind

Wenn `retaprompt_input` oder `retaprompt_commands` selbst als Rust-`staticlib` gebaut würden,
würde Cargo deren Rust-Abhängigkeiten in die Archive hineinziehen. Dann würde `reta` in beiden
Zusatzarchiven wieder stecken. Genau das soll vermieden werden.

Darum ist die Aufteilung hier absichtlich zweistufig:

1. `reta` wird als echte Rust-`staticlib` gebaut und erzeugt `libreta.a`.
2. `libretaprompt_input.a` und `libretaprompt_commands.a` werden als **kleine C-Archive** gebaut,
   die nur Forwarder-Symbole enthalten und in `libreta.a` weiterleiten.

So bleibt die Implementierung genau einmal in `libreta.a`.

## Build

```bash
./tools/build_prompt_split_staticlibs.sh release
```

Danach liegen die Archive hier:

```text
target/release/libreta.a
target/release/libretaprompt_input.a
target/release/libretaprompt_commands.a
```

## Harte Verifikation im Build-Skript

Das Build-Skript prüft nach dem Bauen zusätzlich:

- `libretaprompt_input.a` enthält **nur** `retaprompt_input_shim.o`
- `libretaprompt_commands.a` enthält **nur** `retaprompt_commands_shim.o`
- die erwarteten exportierten Forwarder-Symbole sind vorhanden
- die beiden Zusatzarchive definieren nur ihre erwarteten Forwarder-Symbole
- `libreta.a` enthält keine Shim-Objekte der zwei Zusatzarchive

Damit ist die Archivstruktur selbst maschinell abgesichert.

## Link-Reihenfolge

```text
... libretaprompt_input.a libretaprompt_commands.a libreta.a ...
```

Die beiden kleineren Archive referenzieren Symbole aus `libreta.a`.


## Paketieren der drei Archive

Wenn die drei Archive samt Headern und Manifest in ein gemeinsames Ausgabeverzeichnis
kopiert werden sollen, geht das mit:

```bash
./tools/package_prompt_split_staticlibs.sh release
```

Dann entsteht zusätzlich:

```text
target/release/retaprompt_split_staticlibs_package/
```

Darin liegen:

- `libreta.a`
- `libretaprompt_input.a`
- `libretaprompt_commands.a`
- `include/retaprompt_input.h`
- `include/retaprompt_commands.h`
- `retaprompt_split_staticlibs_manifest.json`
- `LINK_ORDER.txt`

## Zusätzliche Verifikation

Das Build-Skript prüft jetzt nicht nur, dass jedes Zusatzarchiv genau ein einziges
Shim-Objekt enthält, sondern auch, dass es **nur die erwarteten Forwarder-Symbole**
definiert und dass `libreta.a` selbst **keine Shim-Objekte** enthält. Dadurch wird
Doppelinhalt noch härter ausgeschlossen.
