use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::FromRepr;

use crate::prelude::*;

#[derive(
    Serialize_repr,
    Deserialize_repr,
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    EnumAsInner,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
)]
#[repr(u32)]
pub enum CAP26KeyKind {
    /// For a key to be used for signing transactions.
    /// The value is the ascii sum of `"TRANSACTION_SIGNING"`
    TransactionSigning = 1460,

    /// For a key to be used for signing authentication..
    /// The value is the ascii sum of `"AUTHENTICATION_SIGNING"`
    AuthenticationSigning = 1678,

    /// For a key to be used for encrypting messages.
    /// The value is the ascii sum of `"MESSAGE_ENCRYPTION"`
    MessageEncryption = 1391,
}

impl HasSampleValues for CAP26KeyKind {
    fn sample() -> Self {
        Self::TransactionSigning
    }
    fn sample_other() -> Self {
        Self::AuthenticationSigning
    }
}

impl CAP26KeyKind {
    /// The raw representation of this key kind, an `u32`.
    pub fn discriminant(&self) -> u32 {
        *self as u32
    }
}

impl TryFrom<U31> for CAP26KeyKind {
    type Error = CommonError;
    fn try_from(value: U31) -> Result<Self> {
        let value = u32::from(value);
        match value {
            1460 => Ok(Self::TransactionSigning),
            1678 => Ok(Self::AuthenticationSigning),
            1391 => Ok(Self::MessageEncryption),
            _ => Err(CommonError::InvalidKeyKind),
        }
    }
}
