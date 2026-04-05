# Start der 1:1-Transcompilierung

Dies ist kein Refactoring-Projekt.  
Dies ist ein Paritätsprojekt.

## Bereits begonnen

- `src/bin/grundStrukHtml.rs`
  - direkte strukturelle Übersetzung von:
    - `cmp_before`
    - `cmpx`
    - `merge_dicts`
    - `traverseHierarchy`
    - `myprint`
    - Top-Level-Ablauf
- `src/bin/reta.rs`
  - Startanker für die zweite Exec

## Nächste harte Schritte

1. `i18n/words.py`
   - `wahl15`
   - `ParametersMain.grundstrukturen`
   - keine Vereinfachung
   - gleiche Namen

2. `libs/LibRetaPrompt.py`
   - nur so weit portieren, dass `grundStrukHtml` exakt dieselben Daten bekommt

3. `reta.py`
   - `Program`
   - `produceAllSpaltenNumbers`
   - innere Hilfsfunktionen in derselben Reihenfolge

4. danach erst:
   - `center.py`
   - `tableHandling.py`
   - `lib4tables*.py`
   - weitere Modulabhängigkeiten

## Harte Regeln

- Python bleibt die einzige Wahrheit.
- Kein idiomatisches Rust, wenn das Verhalten driftet.
- Reihenfolge ist heilig.
- Zwischenzustände sind relevant.
- Fehlerfälle müssen mit portiert werden.
