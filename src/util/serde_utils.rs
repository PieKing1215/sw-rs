//! Module containing some utility types for ser/de

use fakemap::FakeMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub(crate) enum RecursiveStringMap {
    String(String),
    Map(FakeMap<String, RecursiveStringMap>),
}

impl Default for RecursiveStringMap {
    fn default() -> Self {
        Self::Map(FakeMap::new())
    }
}

impl RecursiveStringMap {
    #[must_use]
    pub fn into_map(self) -> Option<FakeMap<String, RecursiveStringMap>> {
        match self {
            RecursiveStringMap::Map(m) => Some(m),
            RecursiveStringMap::String(_) => None,
        }
    }
}
