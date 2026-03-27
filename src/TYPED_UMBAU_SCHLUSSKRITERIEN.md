# Typed-Umbau: Schlusskriterien

Schluss ist erst dann, wenn **alle drei** Punkte erfüllt sind:

1. `cargo build` ist grün.
2. Diese Regressionen laufen sichtbar korrekt:
   - `target/debug/rpnn --vorhervonausschnitt 1-3 --spaltenname universum geist`
   - `target/debug/rpnn --vorhervonausschnitt 1-3 --spaltenname universum primzahlkreuz`
   - `target/debug/rpnn --vorhervonausschnitt 1-3 --spaltenname menschliches Gewalt`
   - `target/debug/rpnn --vorhervonausschnitt 1-3 --spaltenname Symbole Religion`
   - `target/debug/rpnn --vorhervonausschnitt 1-20 --alles --art html`
3. Es gibt im Request-Pfad keinen inneren Rückweg mehr auf künstlich rekonstruierte CLI-Strings.

## Was danach noch optional wäre

- Warnings aufräumen
- `KategorieMap` intern noch stärker typisieren
- eines der beiden Request-Modelle langfristig löschen

Das sind **keine** Blocker mehr für den aktuellen Umbau.
