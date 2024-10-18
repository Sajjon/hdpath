use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deref, Mul, AsRef)]
#[deref(forward)]
pub struct SecurifiedU30(U30);

impl HasIndexInLocalKeySpace for SecurifiedU30 {}
impl HasOffsetFromGlobalKeySpace for SecurifiedU30 {
    fn offset_from_global_key_space() -> u32 {
        GLOBAL_OFFSET_SECURIFIED
    }
}

impl TryFrom<HDPathComponent> for SecurifiedU30 {
    type Error = CommonError;

    fn try_from(value: HDPathComponent) -> Result<Self> {
        value
            .into_securified()
            .map_err(|_| CommonError::IndexUnsecurifiedExpectedSecurified)
    }
}
