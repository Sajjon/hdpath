use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::prelude::*;

/// Account or Identity (used by Personas) part of a CAP26 derivation
/// path.
#[derive(
    Serialize_repr,
    Deserialize_repr,
    Clone,
    Copy,
    EnumAsInner,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::Debug,
)]
#[repr(u32)]
pub enum CAP26EntityKind {
    /// An Account entity type
    #[display("Account")]
    Account = 525,

    /// An Identity entity type (used by Personas)
    #[display("Identity")]
    Identity = 618,
}

impl CAP26EntityKind {
    /// The raw representation of this entity kind, an `HDPathValue`.
    pub fn discriminant(&self) -> u32 {
        *self as u32
    }
}

impl TryFrom<u32> for CAP26EntityKind {
    type Error = CommonError;
    fn try_from(value: u32) -> Result<Self> {
        match value {
            525 => Ok(Self::Account),
            618 => Ok(Self::Identity),
            _ => Err(CommonError::InvalidEntityKind),
        }
    }
}
