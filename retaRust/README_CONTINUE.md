# Fortsetzung der direkten Transcompilierung

Neu in diesem Stand:

- `src/i18n/words.rs`
  - erste direkte Rust-Spur für `wahl15`
  - erste direkte Rust-Spur für `ParametersMain.grundstrukturen[0]`
  - direkte Python-Extrakte als String-Konstanten eingebettet

- `src/libs/LibRetaPrompt.rs`
  - minimale Brücke für `grundStrukHtml`

- `src/bin/grundStrukHtml.rs`
  - jetzt an `wahl15()` und `grundstrukturen_0()` angebunden

## Ehrlicher Stand

Das ist noch nicht vollständig 1:1-paritätisch.
Aber es ist jetzt der erste echte Schritt mit Daten aus der Python-Quelle selbst.
