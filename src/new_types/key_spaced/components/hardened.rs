use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, EnumAsInner)]
pub enum Hardened {
    Unsecurified(UnsecurifiedHardened),
    Securified(SecurifiedU30),
}

impl IsMappableToGlobalKeySpace for Hardened {
    fn into_global_key_space(self) -> u32 {
        match self {
            Hardened::Unsecurified(u) => u.into_global_key_space(),
            Hardened::Securified(s) => s.into_global_key_space(),
        }
    }
}

impl TryFrom<HDPathComponent> for Hardened {
    type Error = CommonError;

    fn try_from(value: HDPathComponent) -> Result<Self> {
        match value {
            HDPathComponent::Unsecurified(u) => {
                UnsecurifiedHardened::try_from(u).map(Self::Unsecurified)
            }
            HDPathComponent::Securified(s) => Ok(Hardened::Securified(s)),
        }
    }
}
