pub mod faktor_finder;
pub mod teiler_rechner;
pub mod bereichs_verarbeitung;
pub mod teiler_utils;

// Re-export der wichtigsten Funktionen
pub use faktor_finder::{multiples, find_all_divisors};
pub use teiler_rechner::{teiler, teiler_enhanced};
pub use bereichs_verarbeitung::simulate_bereich_to_numbers2;
pub use teiler_utils::*;
