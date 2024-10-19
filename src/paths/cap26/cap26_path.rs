use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    EnumAsInner,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    // SerializeDisplay,
    // DeserializeFromStr,
    // derive_more::Display,
)]
pub enum CAP26Path {
    // #[display("{value}")]
    GetID { value: CAP26GetIDPath },
    // #[display("{value}")]
    Account { value: CAP26AccountPath },
    // #[display("{value}")]
    Identity { value: CAP26IdentityPath },
}

// impl FromStr for CAP26Path {
//     type Err = CommonError;
//     fn from_str(s: &str) -> Result<Self> {
//         Self::from_bip32_string(s)
//     }
// }
// impl FromBip32String for CAP26Path {
//     fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
//         todo!()
//     }
// }
