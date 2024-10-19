use crate::prelude::*;

#[derive(
    Clone,
    PartialEq,
    EnumAsInner,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    // SerializeDisplay,
    // DeserializeFromStr,
    MoreDebug,
    // derive_more::Display,
)]
pub enum CAP26Path {
    // #[display("{value}")]
    // #[debug("{:?}", value)]
    GetID { value: CAP26GetIDPath },

    // #[display("{value}")]
    // #[debug("{:?}", value)]
    Account { value: CAP26AccountPath },

    // #[display("{value}")]
    // #[debug("{:?}", value)]
    Identity { value: CAP26IdentityPath },
}

impl CAP26Path {
    pub fn get_id() -> Self {
        Self::GetID {
            value: CAP26GetIDPath,
        }
    }
    pub fn account(path: CAP26AccountPath) -> Self {
        Self::Account { value: path }
    }
    pub fn identity(path: CAP26IdentityPath) -> Self {
        Self::Identity { value: path }
    }
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

// impl FromBIP32Str for CAP26Path {
//     fn from_bip32_string(s: &str) -> Result<Self> {
//         CAP26IdentityPath::from_bip32_string(s)
//             .map(|_| Self::get_id())
//             .or(UnsecurifiedHardened::from_bip32_string(s).map(Self::Unsecurified))
//     }
// }
