use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Unsecurified {
    Unhardened(Unhardened),
    Hardened(UnsecurifiedHardened),
}

impl IsMappableToGlobalKeySpace for Unsecurified {
    fn into_global_key_space(self) -> u32 {
        match self {
            Self::Unhardened(u) => u.into_global_key_space(),
            Self::Hardened(h) => h.into_global_key_space(),
        }
    }
}

impl From<UnsecurifiedHardened> for Unsecurified {
    fn from(value: UnsecurifiedHardened) -> Self {
        Unsecurified::Hardened(value)
    }
}

impl TryFrom<HDPathComponent> for Unsecurified {
    type Error = CommonError;

    fn try_from(value: HDPathComponent) -> Result<Self> {
        value
            .into_unsecurified()
            .map_err(|_| CommonError::IndexSecurifiedExpectedUnsecurified)
    }
}
