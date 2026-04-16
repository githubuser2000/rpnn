use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Default)]
pub struct RetaRequest {
    pub options: RetaOptions,
    pub input: RetaInput,
    pub runtime: RetaRuntime,
}

#[derive(Debug, Clone, Default)]
pub struct RetaOptions {
    pub onetable: bool,
    pub breite: Option<usize>,
    pub spaltenreihenfolgeundnurdiese: Option<Vec<String>>,
    pub vorhervonausschnitt: Option<String>,
    pub passthrough_flags: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RetaInput {
    pub stdin_text: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RetaRuntime {
    pub terminal_width: Option<usize>,
    pub stdout_is_tty: Option<bool>,
    pub stderr_is_tty: Option<bool>,
    pub stdin_is_tty: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct RetaResponse {
    pub rendered_text: String,
    pub stderr_text: String,
    pub exit_code: i32,
    pub diagnostics: Vec<RetaDiagnostic>,
    pub metadata: RetaMetadata,
}

#[derive(Debug, Clone, Default)]
pub struct RetaMetadata {
    pub effective_width: Option<usize>,
    pub selected_columns: Vec<String>,
    pub rows_emitted: usize,
}

#[derive(Debug, Clone)]
pub struct RetaDiagnostic {
    pub level: DiagnosticLevel,
    pub code: String,
    pub message: String,
}

impl RetaDiagnostic {
    pub fn info(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            level: DiagnosticLevel::Info,
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            level: DiagnosticLevel::Warning,
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            code: code.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiagnosticLevel {
    #[default]
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Default)]
pub struct NormalizedRequest {
    pub effective_width: Option<usize>,
    pub onetable: bool,
    pub raw_selection_expr: Option<String>,
    pub raw_column_order: Option<Vec<String>>,
    pub stdin_text: Option<String>,
    pub row_selection: RowSelection,
    pub diagnostics: Vec<RetaDiagnostic>,
}

#[derive(Debug, Clone, Default)]
pub struct ColumnPlan {
    pub selected_columns: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ResultingTable {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
pub enum RowSelection {
    All,
    Selected(BTreeSet<usize>),
}

impl Default for RowSelection {
    fn default() -> Self {
        Self::All
    }
}

impl RowSelection {
    pub fn contains(&self, line_number: usize) -> bool {
        match self {
            Self::All => true,
            Self::Selected(lines) => lines.contains(&line_number),
        }
    }

    pub fn selected_count(&self) -> Option<usize> {
        match self {
            Self::All => None,
            Self::Selected(lines) => Some(lines.len()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RetaError {
    InvalidUtf8Request,
    InvalidOptions(String),
    Execution(String),
}

impl RetaError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::InvalidUtf8Request | Self::InvalidOptions(_) => 2,
            Self::Execution(_) => 1,
        }
    }
}

impl fmt::Display for RetaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUtf8Request => f.write_str("invalid UTF-8 in request"),
            Self::InvalidOptions(message) => write!(f, "invalid options: {message}"),
            Self::Execution(message) => write!(f, "execution failed: {message}"),
        }
    }
}

impl Error for RetaError {}
