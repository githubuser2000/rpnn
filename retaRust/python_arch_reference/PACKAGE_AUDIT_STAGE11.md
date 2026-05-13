# Paket-Audit Stage 11

## Warum Stage 10 kleiner war

Stage 9 enthielt viele erzeugte Python-Laufzeitdateien (`__pycache__`, `.pyc`) für verschiedene Python-Versionen. Stage 10 entfernte diese Artefakte. Dadurch wurde das ZIP kleiner, ohne dass die eigentlichen Quellen, CSV-Dateien oder i18n-Dateien verloren gingen.

## Nutzerpaket `reta.differentArchitecture1.tar.bz2`

Das Nutzerpaket wurde entpackt und geprüft. Ergebnis:

- Es ist grundsätzlich entpackbar.
- Es enthält viele Laufzeit-Artefakte (`__pycache__` / `.pyc`).
- Es entspricht inhaltlich eher einem älteren Stand vor Stage 10.
- Es enthält noch nicht:
  - `reta_architecture/prompt_execution.py`
  - `reta_architecture/prompt_preparation.py`
  - `reta_architecture/package_integrity.py`
  - `STAGE10_CHANGES.md`

Deshalb wurde nicht auf dem Nutzerpaket weitergebaut, sondern auf dem vollständigen Stage-10-Stand.

## Stage-11-Manifest

Das Stage-11-Paket enthält nach Entfernung der Laufzeit-Artefakte:

- keine `.pyc`-Dateien
- keine `__pycache__`-Dateien
- alle Pflichtquellen laut `reta_architecture/package_integrity.py`
- `csv/religion.csv` mit 1043 Zeilen
- `csv/vn-religion.csv` mit 1043 Zeilen

Das Manifest kann geprüft werden mit:

```bash
PYTHONDONTWRITEBYTECODE=1 python reta_architecture_probe_py.py package-integrity-json
```
