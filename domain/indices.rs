use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnNumber(pub u32);

impl ColumnNumber {
    pub fn get(self) -> u32 { self.0 }
}

impl fmt::Display for ColumnNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for ColumnNumber {
    fn from(value: u32) -> Self { Self(value) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowNumber(pub usize);

impl RowNumber {
    pub fn get(self) -> usize { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeaderIndex(pub usize);

impl HeaderIndex {
    pub fn get(self) -> usize { self.0 }
}
