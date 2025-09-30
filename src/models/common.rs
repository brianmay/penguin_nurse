use derive_enum_all_values::AllValues;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Serializing Option<Option<String>> does not work as expected. This is a workaround.

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Default)]
pub enum MaybeSet<T> {
    Set(T),
    #[default]
    NoChange,
}

impl<T> MaybeSet<T> {
    #[cfg(feature = "server")]
    pub fn as_deref(&self) -> MaybeSet<&T::Target>
    where
        T: std::ops::Deref,
    {
        match self {
            Self::Set(value) => MaybeSet::Set(value.deref()),
            Self::NoChange => MaybeSet::NoChange,
        }
    }

    #[cfg(feature = "server")]
    pub fn as_ref(&self) -> MaybeSet<&T> {
        match self {
            Self::Set(value) => MaybeSet::Set(value),
            Self::NoChange => MaybeSet::NoChange,
        }
    }

    #[cfg(feature = "server")]
    pub fn into_option(self) -> Option<T> {
        match self {
            MaybeSet::Set(value) => Some(value),
            MaybeSet::NoChange => None,
        }
    }

    #[cfg(feature = "server")]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> MaybeSet<U> {
        match self {
            MaybeSet::Set(value) => MaybeSet::Set(f(value)),
            MaybeSet::NoChange => MaybeSet::NoChange,
        }
    }

    #[cfg(feature = "server")]
    pub fn map_into<U>(self) -> MaybeSet<U>
    where
        U: From<T>,
    {
        self.map(|x| x.into())
    }
}

impl<T> MaybeSet<Option<T>> {
    #[cfg(feature = "server")]
    pub fn map_inner_deref(&self) -> MaybeSet<Option<&T::Target>>
    where
        T: std::ops::Deref,
    {
        self.as_ref().map(|x| x.as_deref())
    }

    #[cfg(feature = "server")]
    pub fn map_inner_into<U>(self) -> MaybeSet<Option<U>>
    where
        U: From<T>,
    {
        self.map(|x| x.map(|y| y.into()))
    }

    #[cfg(feature = "server")]
    pub fn as_inner_ref(&self) -> MaybeSet<Option<&T>> {
        self.as_ref().map(|x| x.as_ref())
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Default, AllValues)]
#[serde(tag = "type")]
pub enum Urgency {
    #[default]
    U0,
    U1,
    U2,
    U3,
    U4,
    U5,
}

#[derive(Error, Debug)]
#[error("Failed to parse Urgency")]
pub struct UrgencyParseError;

impl TryFrom<i32> for Urgency {
    type Error = UrgencyParseError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Urgency::U0),
            1 => Ok(Urgency::U1),
            2 => Ok(Urgency::U2),
            3 => Ok(Urgency::U3),
            4 => Ok(Urgency::U4),
            5 => Ok(Urgency::U5),
            _ => Err(UrgencyParseError),
        }
    }
}

impl From<Urgency> for i32 {
    fn from(value: Urgency) -> Self {
        match value {
            Urgency::U0 => 0,
            Urgency::U1 => 1,
            Urgency::U2 => 2,
            Urgency::U3 => 3,
            Urgency::U4 => 4,
            Urgency::U5 => 5,
        }
    }
}

impl Urgency {
    pub fn as_id(&self) -> &'static str {
        match self {
            Urgency::U0 => "0",
            Urgency::U1 => "1",
            Urgency::U2 => "2",
            Urgency::U3 => "3",
            Urgency::U4 => "4",
            Urgency::U5 => "5",
        }
    }

    pub fn as_title(&self) -> &'static str {
        match self {
            Urgency::U0 => "No urgency",
            Urgency::U1 => "Extremely mild urgency",
            Urgency::U2 => "Mild urgency",
            Urgency::U3 => "Normal urgency",
            Urgency::U4 => "Severe urgency",
            Urgency::U5 => "Extreme urgency",
        }
    }
}
