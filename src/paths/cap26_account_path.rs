use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    // SerializeDisplay,
    // DeserializeFromStr,
    // derive_more::Display,
)]
// #[display("{}", self.bip32_string())]
pub struct CAP26AccountPath {
    network_id: NetworkID,
    key_kind: CAP26KeyKind,
    index: Hardened,
}

// impl ToBip32String for CAP26AccountPath {
//     fn bip32_string(&self) -> String {
//         todo!()
//     }
// }
// impl FromBip32String for CAP26AccountPath {
//     fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
//         todo!()
//     }
// }
// impl_from_str_for__from_bip32_str!(CAP26AccountPath);
