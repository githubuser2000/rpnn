# zip_longest literal

Dieser Stand drückt zwei Stellen noch strenger in Python-Richtung:

- `intoParameterDatatype(...)`
- `mergeParameterDicts(...)`

## Strenger als zuvor

- `case 2` wird jetzt getrennt geführt
- `index2a` und `intoA` bleiben getrennte Zwischenstrukturen
- die Zusammenführung läuft jetzt explizit über eine `zip_longest`-artige Schleife
- fehlende Seite wird nicht mit einer "schönen" Rust-Abkürzung ersetzt, sondern mit Python-naher Auffülllogik

## Datenbasis

- `paraNdataMatrix`: **379 echte Einträge**
- `kombiParaNdataMatrix`: **12 echte Schlüssel**
- `kombiParaNdataMatrix2`: **14 echte Schlüssel**

## Noch offen

- vollständiger Anschluss dieser strengeren `storeParamtersForColumns`-Innensemantik an den restlichen Programmlauf
- noch exaktere Reproduktion einzelner Python-Typflüsse
