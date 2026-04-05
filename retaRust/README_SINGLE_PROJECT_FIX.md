# single project fix

Dieser Stand entfernt den Fehler mit zwei Projekten.

Wichtig:
- CSV-Loader ist jetzt im bestehenden Projekt integriert
- es gibt keinen zweiten Cargo-Ordner mehr
- `src/bin/reta.rs` benutzt wieder das echte transcompilierte `Program`
- `csv/` bleibt der Python-treue Pfad
