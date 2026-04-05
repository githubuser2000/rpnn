/*
MINIMALE PYTHON-ARTIGE STÜTZSTRUKTUREN

Absichtlich nicht idiomatisch.
Nur Gerüst, damit die nächste Stufe nicht im luftleeren Raum beginnt.
*/

#[allow(non_camel_case_types)]
pub type pybool = bool;

#[derive(Clone, Debug)]
pub struct PyTupleLike;

#[derive(Clone, Debug)]
pub struct PyOrderedSetLike;

#[derive(Clone, Debug)]
pub struct PyOrderedDictLike;

#[derive(Clone, Debug)]
pub struct TablesPlaceholder;

impl TablesPlaceholder {
    pub fn new() -> Self {
        Self
    }
}
