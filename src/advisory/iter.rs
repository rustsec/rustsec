use std::collections::btree_map;

use super::{Advisory, AdvisoryId};

/// Advisory iterator
pub struct Iter<'a>(pub(crate) btree_map::Iter<'a, AdvisoryId, Advisory>);

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
