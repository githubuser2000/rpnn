use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Wrap},
    DefaultTerminal,
};

use super::commands::{PromptOutput, SessionState};
use super::completion::candidates_for_input;
use super::python_like::PromptModus;

#[derive(Clone, Debug)]
pub struct RpTuiState {
    pub history_index: usize,
    pub history: Vec<String>,
    pub current_input: String,
    pub preview: PromptOutput,
    pub vi_mode: bool,
    pub completions: Vec<String>,
    pub stored_commands: Vec<String>,
    pub prompt_mode: PromptModus,
}

impl RpTuiState {
    pub fn from_session(session: &SessionState) -> Self {
        Self {
            history_index: session.history_lines.len().saturating_sub(1),
            history: session.history_lines.clone(),
            current_input: session.last_input.clone(),
            preview: session.last_output.clone(),
            vi_mode: session.vi_mode,
            completions: candidates_for_input(&session.last_input),
            stored_commands: session.stored_commands.clone(),
            prompt_mode: session.prompt_mode,
        }
    }

    pub fn selected_history(&self) -> Option<&str> {
        self.history.get(self.history_index).map(|s| s.as_str())
    }

    pub fn clamp(&mut self) {
        if self.history.is_empty() {
            self.history_index = 0;
        } else if self.history_index >= self.history.len() {
            self.history_index = self.history.len() - 1;
        }
    }
}

pub fn launch_preview_ui(session: &SessionState) -> io::Result<()> {
    let mut terminal = ratatui::try_init()?;
    let mut state = RpTuiState::from_session(session);
    state.clamp();
    let result = run_loop(&mut terminal, &mut state);
    ratatui::restore();
    result
}

fn run_loop(terminal: &mut DefaultTerminal, state: &mut RpTuiState) -> io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, state))?;
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Up | KeyCode::Char('k') => {
                        if state.history_index > 0 {
                            state.history_index -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if state.history_index + 1 < state.history.len() {
                            state.history_index += 1;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut ratatui::Frame<'_>, state: &mut RpTuiState) {
    let area = frame.area();
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(9),
            Constraint::Length(4),
        ])
        .split(area);

    render_status(frame, vertical[0], state);

    let middle = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(28), Constraint::Percentage(72)])
        .split(vertical[1]);
    render_history(frame, middle[0], state);
    render_preview(frame, middle[1], state);

    render_candidates(frame, vertical[2], state);
    render_footer(frame, vertical[3], state);
}

fn render_status(frame: &mut ratatui::Frame<'_>, area: Rect, state: &RpTuiState) {
    let status_text = vec![
        Line::from(format!(
            "rp Vorschau  |  Editiermodus: {}  |  PromptModus: {:?}  |  History: {}  |  Gespeichert: {}  |  Letzte Ausgabe: {} (code {})",
            if state.vi_mode { "vi" } else { "emacs" },
            state.prompt_mode,
            state.history.len(),
            state.stored_commands.len(),
            if state.preview.title.is_empty() {
                "-"
            } else {
                &state.preview.title
            },
            state.preview.exit_code,
        )),
        Line::from(format!("Aktuelle Eingabe: {}", state.current_input)),
    ];
    let paragraph = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL).title("Status"))
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

fn render_history(frame: &mut ratatui::Frame<'_>, area: Rect, state: &mut RpTuiState) {
    let items = if state.history.is_empty() {
        vec![ListItem::new("<leer>")]
    } else {
        state
            .history
            .iter()
            .enumerate()
            .map(|(idx, line)| ListItem::new(format!("{:>4}: {}", idx + 1, line)))
            .collect::<Vec<_>>()
    };
    let mut list_state = ListState::default();
    if !state.history.is_empty() {
        list_state.select(Some(state.history_index));
    }
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("History"))
        .highlight_symbol("> ")
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    frame.render_stateful_widget(list, area, &mut list_state);
}

fn render_preview(frame: &mut ratatui::Frame<'_>, area: Rect, state: &RpTuiState) {
    let selected_history = state.selected_history().unwrap_or("<kein Eintrag ausgewählt>");
    let stored = if state.stored_commands.is_empty() {
        "<keine gespeicherten Befehle>".to_string()
    } else {
        state.stored_commands.join("\n")
    };
    let body = if state.preview.text.trim().is_empty() {
        "<keine Ausgabe vorhanden>".to_string()
    } else {
        state.preview.text.clone()
    };

    let text = format!(
        "Gewählte History:\n{}\n\nGespeicherte Befehle:\n{}\n\nLetzte Ausgabe:\n{}",
        selected_history, stored, body
    );

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Vorschau"))
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn render_candidates(frame: &mut ratatui::Frame<'_>, area: Rect, state: &RpTuiState) {
    let rows = if state.completions.is_empty() {
        vec![Row::new(vec![Cell::from("<keine Kandidaten>"), Cell::from("")])]
    } else {
        state
            .completions
            .iter()
            .take(12)
            .enumerate()
            .map(|(idx, candidate)| {
                Row::new(vec![Cell::from((idx + 1).to_string()), Cell::from(candidate.clone())])
            })
            .collect::<Vec<_>>()
    };

    let table = Table::new(rows, [Constraint::Length(4), Constraint::Min(10)])
        .block(Block::default().borders(Borders::ALL).title("Kandidaten"))
        .header(Row::new(vec![Cell::from("#"), Cell::from("Completion")]))
        .column_spacing(1);
    frame.render_widget(table, area);
}

fn render_footer(frame: &mut ratatui::Frame<'_>, area: Rect, state: &RpTuiState) {
    let help = vec![
        Line::from("q / Esc = schließen, ↑↓ oder j/k = History wählen"),
        Line::from(format!(
            "Ausgewählter History-Eintrag: {}",
            state.selected_history().unwrap_or("<leer>")
        )),
    ];
    let paragraph = Paragraph::new(help)
        .block(Block::default().borders(Borders::ALL).title("Hilfe"))
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}
