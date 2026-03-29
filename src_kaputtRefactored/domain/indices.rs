use std::fmt;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OneBased;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZeroBased;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnSpace;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowSpace;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeaderSpace;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Index<Space, Base> {
    value: usize,
    _marker: PhantomData<(Space, Base)>,
}

impl<Space, Base> Copy for Index<Space, Base> {}
impl<Space, Base> Clone for Index<Space, Base> {
    fn clone(&self) -> Self { *self }
}
impl<Space, Base> fmt::Debug for Index<Space, Base> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Index").field(&self.value).finish()
    }
}

impl<Space, Base> Index<Space, Base> {
    pub const fn new(value: usize) -> Self {
        Self { value, _marker: PhantomData }
    }

    pub const fn raw(self) -> usize { self.value }
}

impl<Space> Index<Space, OneBased> {
    pub const fn one_based(self) -> usize { self.value }
    pub fn zero_based(self) -> Option<Index<Space, ZeroBased>> {
        self.value.checked_sub(1).map(Index::new)
    }
}

impl<Space> Index<Space, ZeroBased> {
    pub const fn zero_based(self) -> usize { self.value }
    pub const fn one_based(self) -> Index<Space, OneBased> { Index::new(self.value + 1) }
}

pub type ColumnNumber = Index<ColumnSpace, OneBased>;
pub type RowNumber = Index<RowSpace, OneBased>;
pub type HeaderIndex = Index<HeaderSpace, ZeroBased>;

impl From<usize> for ColumnNumber {
    fn from(value: usize) -> Self { Self::new(value) }
}
impl From<usize> for RowNumber {
    fn from(value: usize) -> Self { Self::new(value) }
}
impl From<usize> for HeaderIndex {
    fn from(value: usize) -> Self { Self::new(value) }
}
