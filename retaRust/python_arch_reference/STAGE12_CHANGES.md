# Stage 12 – Prompt-Interaktionscontroller herausgezogen

Dieser Schritt baut direkt auf Stage 11 auf. Es wurde nicht vom Original neu angefangen; Basis ist `reta_topology_architecture_refactor_stage11.zip`.

## Neuer Architekturblock

Neu hinzugekommen ist:

- `reta_architecture/prompt_interaction.py`
  - `PromptInteractionBundle`
  - `bootstrap_prompt_interaction(...)`

Diese Schicht besitzt jetzt die verbleibende interaktive Prompt-Orchestrierung:

- Aufbau des Prompt-Loop-Setups
- Moduswechsel zwischen normaler Eingabe, Speichern, Ausgabe gespeicherter Befehle und Löschauswahl
- Speicherung und Löschung von Prompt-Platzhaltern
- Übergabe vorbereiteter Prompt-Texte an die Ausführungsschicht
- Kompatibilitätsmethoden für die alten `retaPrompt.py`-Funktionen

Damit ist die Interaktionslogik nicht mehr direkt semantischer Besitz von `retaPrompt.py`.

## Umbau von `retaPrompt.py`

`retaPrompt.py` wurde von Stage 11 ca. 454 Zeilen auf ca. 124 Zeilen reduziert.

Das Modul ist jetzt nur noch eine Kompatibilitätsfassade für historische Starter wie `rp`, `rpl`, `retaPrompt` und direkte Importe alter Namen. Die eigentliche Interaktion läuft über:

```python
promptInteraction = bootstrap_prompt_interaction(...)
```

Die alten Funktionsnamen bleiben als dünne Wrapper erhalten:

- `PromptScope(...)`
- `PromptAllesVorGroesserSchleife(...)`
- `PromptLoescheVorSpeicherungBefehle(...)`
- `promptInput(...)`
- `promptSpeicherungA(...)`
- `promptSpeicherungB(...)`
- `speichern(...)`
- `newSession(...)`

## Probe-/Audit-Erweiterungen

- `reta_architecture/facade.py`
  - neuer Bootstrap: `RetaArchitecture.bootstrap_prompt_interaction(...)`
  - `snapshot()` enthält jetzt `prompt_interaction`

- `reta_architecture_probe_py.py`
  - neuer Befehl: `prompt-interaction-json`

- `reta_architecture/package_integrity.py`
  - `reta_architecture/prompt_interaction.py` ist jetzt Pflichtdatei im Manifest

## Testanpassungen

- `tests/test_architecture_refactor.py`
  - neuer Test für die explizite Prompt-Interaktionsschicht
  - Source-Checks prüfen, dass `retaPrompt.py` auf `promptInteraction` delegiert

- `tests/test_command_parity.py`
  - Subprozesse werden jetzt mit `-S` gestartet, damit die Paritätsmatrix auch in dieser Umgebung stabil läuft und nicht durch User-/Site-Startup des Python-Interpreters blockiert wird.

## Geprüft

Ausgeführt in `/tmp/reta_stage12/reta.todel`:

```bash
PYTHONPATH=/opt/pyvenv/lib/python3.13/site-packages /opt/pyvenv/bin/python3 -S -m py_compile $(find . -name '*.py' -not -path './__pycache__/*')
PYTHONPATH=/opt/pyvenv/lib/python3.13/site-packages /opt/pyvenv/bin/python3 -S -m unittest tests.test_architecture_refactor -v
PYTHONPATH=/opt/pyvenv/lib/python3.13/site-packages /opt/pyvenv/bin/python3 -S -m unittest tests.test_command_parity -v
PYTHONPATH=/opt/pyvenv/lib/python3.13/site-packages /opt/pyvenv/bin/python3 -S -m unittest -v
PYTHONPATH=/opt/pyvenv/lib/python3.13/site-packages /opt/pyvenv/bin/python3 -S reta_architecture_probe_py.py prompt-interaction-json
PYTHONPATH=/opt/pyvenv/lib/python3.13/site-packages /opt/pyvenv/bin/python3 -S reta_architecture_probe_py.py package-integrity-json
PYTHONPATH=/opt/pyvenv/lib/python3.13/site-packages /opt/pyvenv/bin/python3 -S reta_domain_probe_py.py pair-json Religionen Hinduismus
```

Ergebnisse:

- `py_compile`: OK
- `tests.test_architecture_refactor`: 25 Tests, OK
- `tests.test_command_parity`: 1 Test, OK
- `python -m unittest -v`: 26 Tests, OK
- `prompt-interaction-json`: OK
- `package-integrity-json`: keine fehlenden Pflichtdateien
- `Religionen / Hinduismus -> [217]`

Beobachtete Kerninvarianten:

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `AllSimpleCommandSpalten_len = 554`

## Wichtig

Die Arbeit wurde diesmal bewusst in `/tmp` durchgeführt, weil temporäre Arbeitsverzeichnisse unter `/mnt/data` während der Session bereinigt wurden. Nur das fertige ZIP und die finalen Berichte werden nach `/mnt/data` geschrieben.

## Noch offen

Der Prompt-Stack ist jetzt deutlich sauberer geschichtet:

- Prompt-Runtime
- Completion-Runtime
- Prompt-Language
- Prompt-Session
- Prompt-Execution
- Prompt-Preparation
- Prompt-Interaction

Der nächste sinnvolle Schritt wäre, die noch vorhandenen Kompatibilitätsaliasse in `retaPrompt.py` weiter zu reduzieren oder gezielt zu entscheiden, welche davon als öffentliche Legacy-API erhalten bleiben sollen.
