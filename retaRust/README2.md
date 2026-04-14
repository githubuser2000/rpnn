Diese Quelldateien setzen die rp/retaprompt-Architektur als erste echte Rust-Version um:

- rp als interaktive REPL auf Basis von reedline
- rpl als Variante mit separater History-Datei und gleichem Kern
- Completion, Hinter, History, Validator, Vi/Emacs-Modi
- reta-Dispatch in die bestehende Lib über run_reta_from_args(...)
- ratatui-Vorschau über :ui / :preview mit History-, Vorschau-, Kandidaten- und Status-Panels

Wichtige Einordnung:
- Diese Fassung fokussiert die Prompt-Infrastruktur und den sauberen Rust-Einstiegspunkt.
- Die vollständige Python-Semantik von LibRetaPrompt.py (Kurzbefehle wie a/u/t/G/B/E/I/W, Speicher-Modi s/S/o/l, Python-Regex-Expansion etc.) ist hier noch nicht komplett transcompiliert.
- Genau diese Semantik wäre der nächste Portierschritt.
