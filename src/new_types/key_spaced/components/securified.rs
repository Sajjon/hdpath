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
    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = SecurifiedU30;

    #[test]
    fn from_str_valid_canonical_0() {
        assert_eq!(
            "0S".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(0).unwrap()
        );
    }

    #[test]
    fn from_str_valid_canonical_1() {
        assert_eq!(
            "1S".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(1).unwrap()
        );
    }

    #[test]
    fn from_str_valid_canonical_max() {
        assert_eq!(
            "1073741823S".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(U30_MAX).unwrap()
        );
    }

    #[test]
    fn from_str_valid_uncanonical_0() {
        assert_eq!(
            "0^".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(0).unwrap()
        );
    }

    #[test]
    fn from_str_valid_uncanonical_1() {
        assert_eq!(
            "1^".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(1).unwrap()
        );
    }

    #[test]
    fn from_str_valid_uncanonical_max() {
        assert_eq!(
            "1073741823^".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(U30_MAX).unwrap()
        );
    }

    #[test]
    fn from_str_invalid() {
        assert!("".parse::<Sut>().is_err());
        assert!("foobar".parse::<Sut>().is_err());
        assert!("1X".parse::<Sut>().is_err());
        assert!("987654321987654321S".parse::<Sut>().is_err());
    }
}
