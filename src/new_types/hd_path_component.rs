use crate::prelude::*;

/// ```ignore
/// [ <<------- UNHARDENED ------->> | <<-------- HARDENED --------->> ]
/// [ <<------------ UNSECURIFIED ---|-------->>  | <<- SECURIFIED ->> ]
/// ^                                ^            ^                    ^
/// 0                              2^31       2^31+2^30          2^32+1
///
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, EnumAsInner)]
pub enum HDPathComponent {
    Unsecurified(Unsecurified),
    Securified(SecurifiedU30),
}

impl From<Unsecurified> for HDPathComponent {
    fn from(value: Unsecurified) -> Self {
        Self::Unsecurified(value)
    }
}

impl From<SecurifiedU30> for HDPathComponent {
    fn from(value: SecurifiedU30) -> Self {
        Self::Securified(value)
    }
}

impl From<Hardened> for HDPathComponent {
    fn from(value: Hardened) -> Self {
        match value {
            Hardened::Unsecurified(u) => HDPathComponent::Unsecurified(Unsecurified::from(u)),
            Hardened::Securified(s) => HDPathComponent::Securified(s),
        }
    }
}

impl HDPathComponent {
    fn securified(value: impl Into<SecurifiedU30>) -> Self {
        Self::Securified(value.into())
    }
    fn unsecurified(value: impl Into<Unsecurified>) -> Self {
        Self::Unsecurified(value.into())
    }
}

impl FromStr for HDPathComponent {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        SecurifiedU30::from_str(s)
            .map(Self::securified)
            .or(Unsecurified::from_str(s).map(Self::unsecurified))
    }
}

impl IsMappableToGlobalKeySpace for HDPathComponent {
    fn into_global_key_space(self) -> u32 {
        match self {
            HDPathComponent::Unsecurified(u) => u.into_global_key_space(),
            HDPathComponent::Securified(s) => s.into_global_key_space(),
        }
    }
}
