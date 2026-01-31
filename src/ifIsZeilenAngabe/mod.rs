// ifIsZeilenAngabe/mod.rs
pub mod functions;
pub mod split;
pub mod validation;

// Re-export der wichtigsten Funktionen
pub use functions::*;
pub use split::*;
pub use validation::*;
