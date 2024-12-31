use serde::{Deserialize, Serialize};

// Serializing Option<Option<String>> does not work as expected. This is a workaround.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum MaybeString {
    Some(String),
    None,
}

impl From<Option<String>> for MaybeString {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(value) => MaybeString::Some(value),
            None => MaybeString::None,
        }
    }
}

impl From<MaybeString> for Option<String> {
    fn from(value: MaybeString) -> Self {
        match value {
            MaybeString::Some(value) => Some(value),
            MaybeString::None => None,
        }
    }
}

impl MaybeString {
    pub fn as_deref(&self) -> Option<&str> {
        match self {
            MaybeString::Some(value) => Some(value),
            MaybeString::None => None,
        }
    }

    pub fn option(self) -> Option<String> {
        self.into()
    }
}
