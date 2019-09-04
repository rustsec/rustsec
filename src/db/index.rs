//! Database indexes

pub use crate::set::Iter;

use crate::{advisory, map, package, Map, Set};

/// Database index which maps package names to a set of advisory IDs
#[derive(Debug, Default)]
pub(crate) struct Index(Map<package::Name, Set<advisory::Id>>);

impl Index {
    /// Create a new index
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert an entry into the index
    pub fn insert(&mut self, key: &package::Name, value: &advisory::Id) -> bool {
        let values = match self.0.entry(key.clone()) {
            map::Entry::Vacant(entry) => entry.insert(Set::new()),
            map::Entry::Occupied(entry) => entry.into_mut(),
        };

        values.insert(value.clone())
    }

    /// Get an iterator over advisory IDs for a given package name
    pub fn get(&self, key: &package::Name) -> Option<Iter<'_, advisory::Id>> {
        self.0.get(key).map(|set| set.iter())
    }
}
