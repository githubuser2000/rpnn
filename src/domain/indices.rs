#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnNumber(pub usize);

impl ColumnNumber {
    pub fn one_based(self) -> usize { self.0 }
    pub fn zero_based(self) -> Option<usize> { self.0.checked_sub(1) }
}

impl From<usize> for ColumnNumber {
    fn from(value: usize) -> Self { Self(value) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowNumber(pub usize);

impl From<usize> for RowNumber {
    fn from(value: usize) -> Self { Self(value) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeaderIndex(pub usize);

impl From<usize> for HeaderIndex {
    fn from(value: usize) -> Self { Self(value) }
}
