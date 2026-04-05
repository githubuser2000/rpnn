# Phase 5 more real

Dieser Stand geht über die bloße Verkabelung hinaus und übernimmt mehr vom echten Python-Verhalten.

## Neu direkter übernommen

- Phase 2
  - iteriert jetzt wirklich über `cmd[eq+1:].split(",")`
  - berücksichtigt führendes `-` pro Einzelwert
  - ruft bei Treffern `resultingSpaltenFromTuple(...)` auf
  - erzeugt Fehlerausgaben näher an den Python-Zweigen

- Phase 3
  - versucht jetzt bei parameterlosen Kommandos wirklich `self.paraDict[(cmd, "")]`
  - ruft dann `resultingSpaltenFromTuple(...)` auf
  - sonst Fehlerausgabe näher am Python-Zweig

- Phase 4
  - unterscheidet `galaxie=` und `universum=`
  - iteriert über einzelne Kombi-Spalten
  - versucht echte Lookup-Pfade über `kombiReverseDict` / `kombiReverseDict2`
  - ruft bei Treffern `resultingSpaltenFromTuple(...)` auf

- Phase 5
  - `--breite=0` wird jetzt in der Finish-Reihenfolge berücksichtigt
  - Rekursion + anschließendes `spalten_removeDoublesNthenRemoveOneFromAnother()` folgen jetzt näher der Python-Reihenfolge

## Noch offen

- reale Daten in `paraDict`, `kombiReverseDict`, `kombiReverseDict2`
- echter gebrochen-Branch
- echte i18n-Texte statt Platzhalter
- vollständige Typ-/Datenparität

Das ist jetzt deutlich näher am tatsächlichen Python-Ablauf als die vorigen Zwischenstände.
