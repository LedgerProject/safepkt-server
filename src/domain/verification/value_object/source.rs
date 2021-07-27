use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Source<'a> {
    source: &'a [u8],
}

impl Source<'_> {
    pub fn new(&self, source: &[u8]) {
        Source { source };
    }

    pub fn source(&self) -> &[u8] {
        self.source
    }
}
