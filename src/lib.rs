mod new_types;
mod paths;
mod unchanged;

pub mod prelude {
    pub use crate::new_types::*;
    pub use crate::paths::*;
    pub use crate::unchanged::*;

    pub(crate) use derive_more::derive::{AsRef, Debug as MoreDebug, Deref, Display, Mul};
    pub(crate) use enum_as_inner::EnumAsInner;
    pub(crate) use serde_with::{DeserializeFromStr, SerializeDisplay};

    pub(crate) use std::{ops::Deref, str::FromStr};

    #[cfg(test)]
    pub(crate) use std::collections::HashSet;
}

pub use prelude::*;
