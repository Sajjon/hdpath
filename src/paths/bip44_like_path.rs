use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    SerializeDisplay,
    // DeserializeFromStr,
    derive_more::Display,
)]
// #[display("{}", self.bip32_string())]
pub struct BIP44LikePath {
    index: u32,
}

// impl ToBip32String for BIP44LikePath {
//     fn bip32_string(&self) -> String {
//         todo!()
//     }
// }
// impl FromBip32String for BIP44LikePath {
//     fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
//         todo!()
//     }
// }
// impl_from_str_for__from_bip32_str!(BIP44LikePath);
