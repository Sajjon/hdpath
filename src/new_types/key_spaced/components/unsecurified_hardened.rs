use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, Debug, Mul, AsRef)]
#[deref(forward)]
pub struct UnsecurifiedHardened(U30);

impl UnsecurifiedHardened {
    pub const fn new(value: U30) -> Self {
        Self(value)
    }

    pub fn new_from_global_key_space(value: u32) -> Result<Self> {
        U30::try_from(value).map(Self::new)
    }
}
impl FromLocalKeySpace for UnsecurifiedHardened {
    /// 0' => Ok(0)
    /// 1' => Ok(1)
    /// 2^31 + 5 (5') => Err
    fn from_local_key_space(value: u32) -> Result<Self> {
        U30::try_from(value).map(Self::new)
    }
}

impl HasIndexInLocalKeySpace for UnsecurifiedHardened {}
impl HasOffsetFromGlobalKeySpace for UnsecurifiedHardened {
    fn offset_from_global_key_space() -> u32 {
        LOCAL_OFFSET_SECURIFIED
    }
}

impl TryFrom<Unsecurified> for UnsecurifiedHardened {
    type Error = CommonError;

    fn try_from(value: Unsecurified) -> Result<Self> {
        match value {
            Unsecurified::Unhardened(_) => Err(CommonError::NonHardenedIndex),
            Unsecurified::Hardened(u) => Ok(u),
        }
    }
}
