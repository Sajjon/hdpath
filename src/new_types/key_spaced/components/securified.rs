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

impl FromLocalKeySpace for SecurifiedU30 {
    /// 0^ => Ok(0)
    /// 1^ => Ok(1)
    /// 2^31 + 2^30 + 5 (5^) => Err
    fn from_local_key_space(value: u32) -> Result<Self> {
        U30::try_from(value).map(Self)
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
impl IsPathComponentStringConvertible for SecurifiedU30 {
    const CANONICAL_SUFFIX: &'static str = "S";
    const NON_CANONICAL_SUFFIXES: &'static str = "^";
}
impl FromStr for SecurifiedU30 {
    type Err = CommonError;
    fn from_str(_s: &str) -> Result<Self> {
        todo!()
    }
}
