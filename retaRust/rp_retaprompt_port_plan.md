# rp / retaPrompt für Rust: Zielarchitektur und konkrete Umstellung

## Kernidee

`reta` bleibt der bitgenaue Batch-/CLI-Kern.

`rp` wird **nicht** eine zweite Parser- oder Ausgabelogik, sondern nur eine interaktive Frontschicht über derselben `reta`-Lib:

- **reedline**: Eingabezeile, History, Completion, Hints, Vi/Emacs-Modus
- **ratatui**: Panels, Vorschau, Hilfe, Statuszeile, Kandidatenlisten, Tabellen
- **crossterm**: Event-Layer, Alternate Screen, Raw Mode, Resize, Key Events

Das ist architektonisch sauberer als `reta_min`, weil `reta_min` aktuell nur ein zweiter Name für denselben Batch-Entry-Point ist.

## Was konkret ersetzt werden soll

In `Cargo.toml`:

- `[[bin]] name = "reta_min"` raus
- `[[bin]] name = "rp"` rein
- optional zusätzlich `rpl` als Logging-/Low-noise-Variante

`reta` bleibt unverändert für direkten Shell-Aufruf.

## Empfohlene Modulstruktur

```text
src/
  prompt/
    mod.rs
    app.rs              # Zustand der App
    command.rs          # RP-Kommandos vs reta-Kommandos
    completion.rs       # Komplettierung aus reta-Metadaten
    history.rs          # Pfade und History-Verwaltung
    preview.rs          # Live-Vorschau per reta::run_reta_from_args
    repl.rs             # Hauptschleife um reedline
    tui.rs              # ratatui-Rendering
  bin/
    reta.rs
    rp.rs
    rpl.rs             # optional
```

## Trennung der Ebenen

### 1. Kern: `reta`

Der bestehende Aufruf ist schon fast ideal:

```rust
pub fn run_reta_from_args(argv: Vec<String>) -> RetaRunResult
```

Genau diese Funktion ist der Motor für:

- normale Shell-Ausführung
- Vorschau im Prompt
- endgültige Enter-Ausführung im Prompt

Damit wird **keine** zweite Businesslogik gebaut.

### 2. `rp`-Befehlsraum

`rp` braucht einen kleinen Vorparser für Prompt-spezifische Befehle:

- `help`, `hilfe`, `befehle`, `kurzbefehle`
- `q`, `quit`, `exit`, `:q`, `ende`
- `shell ...`
- `python ...` (optional später)
- `math ...` (optional später)
- `loggen`, `nichtloggen`
- Speicher-/Makro-Befehle wie `s`, `S`, `o`, `l`

Alles, was **nicht** als RP-Sonderbefehl erkannt wird, läuft als:

- direkter `reta ...`-Befehl, oder
- Kurzsyntax, die zuerst in einen `reta`-Befehl expandiert wird.

## Minimale Zustandsmaschine

```rust
pub enum PromptMode {
    Normal,
    Speichern,
    LoeschenStart,
    SpeicherungAusgaben,
    LoeschenSelect,
    SpeicherungAusgabenMitZusatz,
    AusgabeSelektiv,
}
```

Die Python-Namen sollten beibehalten werden. Gerade hier lohnt sich Python-Nähe mehr als „schönes Rust“.

## Vorschlag für `AppState`

```rust
pub struct AppState {
    pub vi_mode: bool,
    pub logging_enabled: bool,
    pub prompt_mode: PromptMode,
    pub stored_commands: Vec<String>,
    pub last_command: Option<String>,
    pub status_line: String,
    pub preview_enabled: bool,
    pub preview_text: String,
    pub candidates: Vec<String>,
    pub help_text: String,
    pub history_path: std::path::PathBuf,
}
```

## Completion-Quelle

Die Completion darf nicht aus händisch duplizierten Listen bestehen, sondern aus denselben `reta`-Daten wie Python bzw. Rust-Kern:

- Hauptparameter: `-zeilen`, `-spalten`, `-kombination`, `-ausgabe`
- Nebenparameter aus `retaProgram.paraDict` / Rust-Äquivalent
- bekannte Werte für `--zeit=`, `--typ=`, `--primzahlen=` usw.
- RP-Sonderbefehle (`help`, `quit`, `shell`, `loggen`, ...)

Wichtig: Completion sollte kontextsensitiv sein:

- Zeilen 1: erster Token → RP-Befehl oder `reta`
- nach `reta` oder nach Kurzexpansion → `reta`-Parameter
- nach `-zeilen` → nur passende `--...`
- nach `--zeit=` → `heute,gestern,morgen,*`

## Warum `reedline` hier passt

`reedline` bringt genau die Interaktionsbausteine mit, die du willst:

- `read_line()`-basierte REPL-Schleife
- `FileBackedHistory`
- Completer-Integration
- Hinter/Fish-style Suggestions
- Emacs- und Vi-Edit-Mode
- Menüs für Kandidatenlisten

## Warum `ratatui` hier nicht die Eingabe selbst machen sollte

`ratatui` ist stark für die Panel-/Frame-Seite, aber nicht der richtige Ersatz für line editing. Deshalb:

- **Input Editing** in `reedline`
- **Flächen-/Panel-Rendering** in `ratatui`
- **Terminal-Steuerung** in `crossterm`

Genau diese Aufteilung ist sauber.

## Zwei sinnvolle Betriebsarten

### A. Schnell lauffähige erste Stufe

Nur `reedline` + `reta::run_reta_from_args()`:

- Prompt
- History
- Completion
- Hints
- Vi/Emacs
- Hilfe-Befehle
- `reta ...`-Execution

Noch **ohne** Fullscreen-TUI.

Vorteil: schnell benutzbar, klein, robust.

### B. Zweite Stufe mit `ratatui`

Fullscreen/Alternate-Screen mit Bereichen:

- oben: Eingabe-/Statusbereich
- rechts oder unten: Live-Vorschau des expandierten `reta`-Befehls
- links: Kandidatenliste / Kontext-Hilfe
- unten: Statuszeile (Mode, Logging, Sprache, Vi/Emacs)

Das ist die richtige Stelle für `ratatui`.

## Sauberer Startpunkt: `rp.rs`

```rust
fn main() {
    std::process::exit(reta::prompt::repl::run_from_env(false));
}
```

Optional:

```rust
fn main() {
    std::process::exit(reta::prompt::repl::run_from_env(true));
}
```

für `rpl` mit Logging an.

## Konkreter Cargo-Vorschlag

```toml
[dependencies]
indexmap = "2"
hypher = { version = "0.1.7", default-features = false, features = ["german"] }
termimad = "0.34.1"
reedline = "0.47"
ratatui = { version = "0.30", features = ["crossterm"] }
crossterm = "0.29"
dirs = "6"
nu-ansi-term = "0.50"
```

## Robuste Implementierungsreihenfolge

### Schritt 1

`reta_min` logisch durch `rp` ersetzen:

- neuer Binärname `rp`
- ruft neues Prompt-Modul auf
- `reta` unverändert lassen

### Schritt 2

Nur `reedline`-REPL implementieren:

- History-Datei
- Vi/Emacs-Umschaltung bei Start
- einfache Completion
- `help` / `quit`
- `reta ...` ausführen

### Schritt 3

RP-Kurzbefehle und PromptMode-Zustände transcompilieren.

### Schritt 4

`ratatui`-UI ergänzen:

- Preview-Panel
- Hilfe-Panel
- Kandidatenlisten-Panel
- Tabellen-/Vorschaupanel

## Technischer Knackpunkt

Die eigentliche Schwierigkeit ist **nicht** die TUI, sondern die Python-Logik von `LibRetaPrompt.py`:

- Kurzbefehlexpansion
- Speicher-/Kombinationslogik
- Moduswechsel
- spezielle Zahlen-/Bruchsyntax
- Kontextregeln für Ergänzung und Vorschau

Die UI-Schicht ist der leichtere Teil. Die Prompt-Semantik ist der teure Teil. Deshalb sollte die Portierung dort streng Python-nah erfolgen.

## Harte Empfehlung

Mach **nicht** alles sofort mit Fullscreen-`ratatui`.

Richtige Reihenfolge:

1. `rp` als `reedline`-REPL stabil
2. Python-Logik von `LibRetaPrompt.py` bitnah portieren
3. danach `ratatui`-Panels darüberlegen

Sonst baust du zuerst Oberfläche und musst danach die ganze Interaktionslogik wieder umbauen.

## Beispiel-Skelett für den Einstieg

```rust
pub fn run_from_env(logging_default: bool) -> i32 {
    let argv = std::env::args().collect::<Vec<_>>();
    let vi_mode = argv.iter().any(|a| a == "-vi");
    let mut app = AppState::new(vi_mode, logging_default);
    let mut editor = build_editor(&app);
    let prompt = DefaultPrompt::default();

    loop {
        match editor.read_line(&prompt) {
            Ok(Signal::Success(line)) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                match dispatch_line(&mut app, line) {
                    DispatchResult::Continue(msg) => {
                        if !msg.is_empty() {
                            println!("{}", msg);
                        }
                    }
                    DispatchResult::RunReta(argv) => {
                        let result = reta::run_reta_from_args(argv);
                        println!("{}", result.render_text());
                    }
                    DispatchResult::Exit(code) => return code,
                }
            }
            Ok(Signal::CtrlC) | Ok(Signal::CtrlD) => return 0,
            Err(err) => {
                eprintln!("rp input error: {}", err);
                return 1;
            }
        }
    }
}
```

## Fazit

Ja: genau diese Aufteilung ist richtig.

- `rp` statt `reta_min`
- `rp` als interaktive Frontschicht
- `reta` als unveränderter Kern
- `reedline` für Eingabelogik
- `ratatui` für Panels und Vorschau
- `crossterm` als technische Terminalbasis

Das ist die saubere Rust-Entsprechung zu `rp`/`retaPrompt` aus Python.
