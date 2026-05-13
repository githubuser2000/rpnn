# Stage 11 – Prompt-Vorbereitung und Paketintegrität

Dieser Schritt baut auf Stage 10 auf und beginnt nicht neu.

## Korrektur zur Archivgröße

Stage 10 war kleiner als Stage 9, weil Stage 9 viele Laufzeit-Artefakte enthielt, vor allem `__pycache__`-Dateien für Python 3.11 und Python 3.13. In Stage 10 waren diese Artefakte entfernt. Die großen Daten- und Quellbestände blieben erhalten.

Das vom Nutzer gelieferte Paket `reta.differentArchitecture1.tar.bz2` ließ sich vollständig entpacken, ist aber inhaltlich älter als Stage 10/11: Es enthält noch viele `__pycache__`-Dateien und besitzt die Stage-10/11-Dateien `reta_architecture/prompt_execution.py`, `reta_architecture/prompt_preparation.py` und `reta_architecture/package_integrity.py` noch nicht. Deshalb wurde Stage 10 als Fortsetzungsbasis verwendet.

## Neu in Stage 11

- `reta_architecture/prompt_preparation.py`
  - neue explizite Schicht für die Prompt-Vorbereitung
  - enthält jetzt die aus `retaPrompt.py` herausgezogenen Funktionen:
    - `verdreheWoReTaBefehl(...)`
    - `regExReplace(...)`
    - `promptVorbereitungGrosseAusgabe(...)`
  - kapselt Regex-/Wildcard-Expansion, Reta-Befehl-Rotation und Normalisierung vor der großen Prompt-Ausgabe

- `retaPrompt.py`
  - delegiert Prompt-Vorbereitung jetzt an `architecture.bootstrap_prompt_preparation(...)`
  - enthält keine eigenen Definitionen von `regExReplace(...)` und `promptVorbereitungGrosseAusgabe(...)` mehr
  - ist von ca. 815 Zeilen auf ca. 454 Zeilen gefallen

- `reta_architecture/facade.py`
  - neue Methode `bootstrap_prompt_preparation(...)`
  - `snapshot()` enthält jetzt `prompt_preparation`

- `reta_architecture_probe_py.py`
  - neuer Befehl `prompt-preparation-json`
  - neuer Befehl `package-integrity-json`

- `reta_architecture/package_integrity.py`
  - neues Audit-/Manifest-Modul
  - ignoriert Laufzeit-Artefakte wie `__pycache__` und `.pyc`
  - prüft Pflichtdateien, CSV-Zeilenzahlen, Gesamtdateiliste und Digest

- `tests/test_architecture_refactor.py`
  - neue Tests für `PromptPreparationBundle`
  - neuer Manifest-/Paketintegritätstest
  - Source-Checks stellen sicher, dass `retaPrompt.py` die neue Vorbereitungsschicht nutzt

## Geprüft

- `py_compile` über die Python-Dateien
- `reta_architecture_probe_py.py prompt-preparation-json`
- `reta_architecture_probe_py.py package-integrity-json`
- `reta.py` Smoke-Test mit direkter Tabellenabfrage
- `python -m unittest tests.test_architecture_refactor -v`
  - 24 Tests
  - Ergebnis: OK
- manuelle Parität gegen das Original-Archiv:
  - kleine Shell-Abfrage: Ausgabe identisch
  - kleine Markdown-Abfrage: Ausgabe identisch

## Nicht behauptet

Die vollständige große Command-Parity-Matrix wurde in diesem Schritt nicht bis zum Ende neu durchlaufen. Die repräsentativen kleinen Shell-/Markdown-Vergleiche waren identisch, und die Architektur-/Semantiktests laufen grün. Für ein finales Release sollte die große Paritätsmatrix weiter ausgebaut und in einer stabilen CI-Umgebung vollständig ausgeführt werden.
