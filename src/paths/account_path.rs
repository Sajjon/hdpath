use crate::prelude::*;

#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    EnumAsInner,
    PartialOrd,
    Ord,
    // derive_more::Display,
    derive_more::Debug,
)]
pub enum AccountPath {
    // #[debug("{}", self.bip32_string())]
    CAP26 { value: CAP26AccountPath },
    // #[debug("{}", self.bip32_string())]
    BIP44Like { value: BIP44LikePath },
}

// impl ToBip32String for AnyAccountPath {
//     fn bip32_string(&self) -> String {
//         todo!()
//     }
// }
// impl FromBip32String for AnyAccountPath {
//     fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
//         todo!()
//     }
// }
// impl_from_str_for__from_bip32_str!(AnyAccountPath);
