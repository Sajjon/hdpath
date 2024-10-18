use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::prelude::*;

#[derive(
    Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
#[repr(u8)]
pub enum NetworkID {
    /// Mainnet (0x01 / 0d01)
    ///
    /// The Radix public network.
    ///
    /// https://github.com/radixdlt/radixdlt-scrypto/blob/v1.0.1/radix-engine-common/src/network/mod.rs#L79
    Mainnet = 0x01,

    /// Stokenet (0x02 / 0d02)
    ///
    /// The public testnet for Radix.
    ///
    /// https://github.com/radixdlt/radixdlt-scrypto/blob/v1.0.1/radix-engine-common/src/network/mod.rs#L71
    Stokenet = 0x02,
}
impl NetworkID {
    /// The raw representation of this network id, an `u8`.
    pub fn discriminant(&self) -> u8 {
        *self as u8
    }
}

impl TryFrom<u32> for NetworkID {
    type Error = CommonError;
    fn try_from(value: u32) -> Result<Self> {
        match value {
            0x01 => Ok(Self::Mainnet),
            0x02 => Ok(Self::Stokenet),
            _ => Err(CommonError::InvalidNetworkID),
        }
    }
}
