use serde::{Deserialize, Serialize};

// Serializing Option<Option<String>> does not work as expected. This is a workaround.

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum MaybeSet<T> {
    Set(T),
    NoChange,
}

impl<T> MaybeSet<T> {
    pub fn as_deref(&self) -> MaybeSet<&T::Target>
    where
        T: std::ops::Deref,
    {
        match self {
            Self::Set(value) => MaybeSet::Set(value.deref()),
            Self::NoChange => MaybeSet::NoChange,
        }
    }

    pub fn as_ref(&self) -> MaybeSet<&T> {
        match self {
            Self::Set(value) => MaybeSet::Set(value),
            Self::NoChange => MaybeSet::NoChange,
        }
    }

    pub fn into_option(self) -> Option<T> {
        match self {
            MaybeSet::Set(value) => Some(value),
            MaybeSet::NoChange => None,
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> MaybeSet<U> {
        match self {
            MaybeSet::Set(value) => MaybeSet::Set(f(value)),
            MaybeSet::NoChange => MaybeSet::NoChange,
        }
    }

    pub fn map_into<U>(self) -> MaybeSet<U>
    where
        U: From<T>,
    {
        self.map(|x| x.into())
    }
}

impl<T> MaybeSet<Option<T>> {
    pub fn map_inner_deref(&self) -> MaybeSet<Option<&T::Target>>
    where
        T: std::ops::Deref,
    {
        self.as_ref().map(|x| x.as_deref())
    }

    pub fn map_inner_into<U>(self) -> MaybeSet<Option<U>>
    where
        U: From<T>,
    {
        self.map(|x| x.map(|y| y.into()))
    }

    pub fn as_inner_ref(&self) -> MaybeSet<Option<&T>> {
        self.as_ref().map(|x| x.as_ref())
    }
}
