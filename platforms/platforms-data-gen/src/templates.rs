//! Provides a convenient interface atop of code-generation templates.
//! The templates themselves are in the `templates` folder.

pub const HEADERS: &[(&'static str, &'static [u8])] = &[
    ("target_arch", include_bytes!("../templates/arch_header.rs")),
    ("target_os", include_bytes!("../templates/os_header.rs")),
    ("target_env", include_bytes!("../templates/env_header.rs")),
    (
        "target_endian",
        include_bytes!("../templates/endian_header.rs"),
    ),
    (
        "target_pointer_width",
        include_bytes!("../templates/bits_header.rs"),
    ),
];

pub const FOOTERS: &[(&'static str, &'static [u8])] = &[
    ("target_arch", include_bytes!("../templates/arch_footer.rs")),
    ("target_os", include_bytes!("../templates/os_footer.rs")),
    ("target_env", include_bytes!("../templates/env_footer.rs")),
    (
        "target_endian",
        include_bytes!("../templates/endian_footer.rs"),
    ),
    (
        "target_pointer_width",
        include_bytes!("../templates/bits_footer.rs"),
    ),
];

use std::collections::HashMap;

pub(crate) struct Templates {
    headers: HashMap<&'static str, &'static [u8]>,
    footers: HashMap<&'static str, &'static [u8]>,
}

impl Templates {
    pub fn new() -> Self {
        let headers = HEADERS.to_owned().into_iter().collect();
        let footers = FOOTERS.to_owned().into_iter().collect();
        Templates { headers, footers }
    }

    /// Accepts the raw (non-enumified) identifier as argument
    pub fn header(&self, key: &str) -> Option<&'static [u8]> {
        self.headers.get(key).cloned()
    }

    /// Accepts the raw (non-enumified) identifier as argument
    pub fn footer(&self, key: &str) -> Option<&'static [u8]> {
        self.footers.get(key).cloned()
    }
}
