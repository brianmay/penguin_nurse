use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Serializing Option<Option<String>> does not work as expected. This is a workaround.
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Maybe<T> {
    Some(T),
    None,
}

impl<T> Maybe<T> {
    pub fn new(value: Option<T>) -> Self {
        match value {
            Some(value) => Maybe::Some(value),
            None => Maybe::None,
        }
    }

    pub fn as_ref(&self) -> Option<&T> {
        match self {
            Maybe::Some(value) => Some(value),
            Maybe::None => None,
        }
    }

    pub fn as_deref(&self) -> Option<&T::Target>
    where
        T: std::ops::Deref,
    {
        match self {
            Maybe::Some(value) => Some(value.deref()),
            Maybe::None => None,
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Maybe<U> {
        match self {
            Maybe::Some(value) => Maybe::Some(f(value)),
            Maybe::None => Maybe::None,
        }
    }
}

impl<T, E> Maybe<Result<T, E>> {
    pub fn transpose(self) -> Result<Maybe<T>, E> {
        match self {
            Maybe::Some(Ok(v)) => Ok(Maybe::Some(v)),
            Maybe::Some(Err(e)) => Err(e),
            Maybe::None => Ok(Maybe::None),
        }
    }
}

impl<T> From<Option<T>> for Maybe<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Maybe::Some(value),
            None => Maybe::None,
        }
    }
}

impl<T> From<Maybe<T>> for Option<T> {
    fn from(value: Maybe<T>) -> Self {
        match value {
            Maybe::Some(value) => Some(value),
            Maybe::None => None,
        }
    }
}

// trait FrontEndConversion<'a> {
//     type FrontEndType;
//     fn to_front_end(&'a self) -> Self::FrontEndType;
//     fn from_front_end(value: &'a Self::FrontEndType) -> Self
//     where
//         Self: Sized;
// }

// impl<'a, T: std::ops::Deref> FrontEndConversion<'a> for Option<Option<T>>
// where
//     T::Target: 'a + ToOwned<Owned = T>,
// {
//     type FrontEndType = Option<Maybe<&'a T::Target>>;
//     fn to_front_end(&'a self) -> Self::FrontEndType {
//         self.as_ref().map(|x| Maybe::new(x.as_deref()))
//     }
//     fn from_front_end(value: &'a Self::FrontEndType) -> Self {
//         value.map(|x| {
//             x.as_ref().map(|y: &&T::Target| {
//                 let z: T = (*y).to_owned();
//                 z
//             })
//         })
//     }
// }

// impl<T: Copy> Maybe<T> {
//     pub fn option(self) -> Option<T> {
//         self.into()
//     }
// }

pub type MaybeString = Maybe<String>;
pub type MaybeDateTime = Maybe<DateTime<Utc>>;
pub type MaybeF64 = Maybe<f64>;
pub type MaybeDecimal = Maybe<bigdecimal::BigDecimal>;
pub type MaybeI32 = Maybe<i32>;
