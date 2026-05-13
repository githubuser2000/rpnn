# Offene Punkte erledigt – 2026-04-23

## Erledigt

### 1. Paritätstest entblockt
- `tests/test_command_parity.py` kann jetzt ohne `/mnt/data/reta.todel.zip` laufen.
- Falls das externe Archiv fehlt, erzeugt die Suite aus der Git-Historie per
  `git archive` ein temporäres Referenzarchiv.
- Optionaler Override: `RETA_PARITY_BASELINE_COMMIT`.

### 2. Fortschrittsoverlay auf den realen Ist-Zustand gezogen
- `reta_architecture/architecture_progress.py` bewertet die Parität nicht mehr
  als offenen Blocker, wenn ein Git-Baseline-Archiv erzeugt werden kann.
- Ergebnis: `outstanding_work = 0`, `validation.status = passed`.

### 3. Architekturtest angepasst
- `tests/test_architecture_refactor.py` prüft nun den neuen geschlossenen
  Zustand des Fortschrittsoverlays.

### 4. Aktuelles Statusdokument ergänzt
- Neues zentrales Dokument: `ARCHITECTURE_STATUS.md`.
- Historische Stage-42-Dokumente verweisen jetzt auf dieses aktuelle Dokument.

## Betroffene Dateien
- `tests/test_command_parity.py`
- `reta_architecture/architecture_progress.py`
- `tests/test_architecture_refactor.py`
- `ARCHITECTURE_STATUS.md`
- `ARCHITECTURE_STATUS_STAGE42.md`
- `PACKAGE_AUDIT_STAGE42.md`
- `STAGE42_CHANGES.md`

## Verifikation
Erfolgreich ausgeführt:

- `python -m py_compile tests/test_command_parity.py tests/test_architecture_refactor.py reta_architecture/architecture_progress.py`
- `python -B -S reta_architecture_probe_py.py architecture-progress-json`
- `python -m unittest tests.test_command_parity.CommandParityMatrixTest.test_representative_command_matrix_matches_original -v`
- `python -m unittest tests.test_architecture_refactor.ArchitectureRefactorRegressionTest.test_architecture_progress_layer_is_explicit -v`

## Neuer Status
- keine verbleibenden Pflichtpunkte im Fortschrittsoverlay
- keine gemischten Owner mehr
- Paritätsbaseline ist im normalen Git-Checkout selbstauflösend
- verbleiben nur optionale Nachverdichtungen von Kompatibilitätsfassaden
