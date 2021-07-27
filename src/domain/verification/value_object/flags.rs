use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Flags<'a> {
    flags: &'a [u8],
}

impl Flags<'_> {
    pub fn new(&self, flags: &[u8]) {
        Flags { flags };
    }

    pub fn flags(&self) -> &[u8] {
        self.flags
    }
}
