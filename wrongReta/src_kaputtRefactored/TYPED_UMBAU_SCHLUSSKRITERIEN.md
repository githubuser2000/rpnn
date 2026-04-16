# Typed-Umbau: Schlusskriterien

Schluss ist erreicht, wenn diese drei Punkte erfüllt sind:

1. `cargo build` läuft ohne Fehler.
2. Diese Befehle liefern fachlich sinnvolle Ausgabe:
   - `target/debug/rpnn --vorhervonausschnitt 1-3 --spaltenname universum geist`
   - `target/debug/rpnn --vorhervonausschnitt 1-3 --spaltenname universum primzahlkreuz`
   - `target/debug/rpnn --vorhervonausschnitt 1-3 --spaltenname menschliches Gewalt`
   - `target/debug/rpnn --vorhervonausschnitt 1-3 --spaltenname Symbole Religion`
   - `target/debug/rpnn --vorhervonausschnitt 1-20 --alles --art html`
3. Im Request-Pfad gibt es keinen inneren Rückweg mehr auf künstlich rekonstruierte CLI-Strings.

Alles danach ist Aufräumen, nicht mehr Kernumbau:
- Warnings reduzieren
- KategorieMap weiter typisieren
- eines der zwei Request-Modelle langfristig entfernen
