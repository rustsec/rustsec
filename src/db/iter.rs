//! Advisory [`Database`] iterator

use crate::{
    advisory::{self, Advisory},
    map,
};

/// Advisory [`Database`] iterator
pub struct Iter<'a>(map::Iter<'a, advisory::Id, Advisory>);

impl<'a> Iter<'a> {
    /// Create a new iterator
    pub(crate) fn new(iter: map::Iter<'a, advisory::Id, Advisory>) -> Self {
        Iter(iter)
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Advisory;

    fn next(&mut self) -> Option<&'a Advisory> {
        self.0.next().map(|(_, adv)| adv)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
