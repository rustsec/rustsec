//! This file contains manually populated data
//! that we add to auto-generated data.
//! It is not required for auto-generation to succeed,
//! but it does augment the auto-populated data for human readability.

use std::collections::HashMap;
use crate::data::enum_variant_comments::COMMENTS;

pub(crate) struct Comments {
    data: HashMap<&'static str, &'static str>,
}

impl Comments {
    pub fn new() -> Self {
        let data= COMMENTS.to_owned().into_iter().collect();
        Comments {data}
    }

    /// Accepts the raw (non-enumified) identifier as argument
    pub fn enum_variant_comment(&self, key: &str) -> Option<&'static str> {
        self.data.get(key).cloned()
    }
}