use crate::prelude::*;

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deref,
    AsRef,
    DeserializeFromStr,
    SerializeDisplay,
    Display,
    MoreDebug,
)]
#[deref(forward)]
#[display("{}", self.to_bip32_string())]
#[debug("{}", self.to_bip32_string_debug())]
pub struct UnsecurifiedHardened(U30);

impl HasSampleValues for UnsecurifiedHardened {
    fn sample() -> Self {
        Self::from_local_key_space(*U30::sample()).unwrap()
    }

    fn sample_other() -> Self {
        Self::from_local_key_space(*U30::sample_other()).unwrap()
    }
}

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

pub const UNSECURIFIED_HARDENED_CANONICAL_SUFFIX: &str = "H";
pub const UNSECURIFIED_HARDENED_NON_CANONICAL_SUFFIX: &str = "'";
impl IsPathComponentStringConvertible for UnsecurifiedHardened {
    const CANONICAL_SUFFIX: &'static str = UNSECURIFIED_HARDENED_CANONICAL_SUFFIX;
    const NON_CANONICAL_SUFFIXES: &'static str = UNSECURIFIED_HARDENED_NON_CANONICAL_SUFFIX;
}

impl HasIndexInLocalKeySpace for UnsecurifiedHardened {}
impl HasOffsetFromGlobalKeySpace for UnsecurifiedHardened {
    fn offset_from_global_key_space() -> u32 {
        GLOBAL_OFFSET_HARDENED
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

impl FromStr for UnsecurifiedHardened {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    type Sut = UnsecurifiedHardened;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample(),);
        assert_eq!(Sut::sample_other(), Sut::sample_other(),);
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other(),);
    }

    #[test]
    fn ord() {
        assert!(Sut::sample() < Sut::sample_other());
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<Sut>::from_iter([
                Sut::sample(),
                Sut::sample(),
                Sut::sample_other(),
                Sut::sample_other(),
            ])
            .len(),
            2
        )
    }

    #[test]
    fn from_str_valid_canonical_0() {
        assert_eq!(
            "0H".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(0).unwrap()
        );
    }

    #[test]
    fn from_str_valid_canonical_1() {
        assert_eq!(
            "1H".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(1).unwrap()
        );
    }

    #[test]
    fn from_str_valid_canonical_max() {
        assert_eq!(
            "1073741823H".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(U30_MAX).unwrap()
        );
    }

    #[test]
    fn from_str_valid_uncanonical_0() {
        assert_eq!(
            "0'".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(0).unwrap()
        );
    }

    #[test]
    fn from_str_valid_uncanonical_1() {
        assert_eq!(
            "1'".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(1).unwrap()
        );
    }

    #[test]
    fn from_str_valid_uncanonical_max() {
        assert_eq!(
            "1073741823'".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(U30_MAX).unwrap()
        );
    }

    #[test]
    fn display_0() {
        assert_eq!(format!("{}", Sut::from_local_key_space(0).unwrap()), "0H");
    }

    #[test]
    fn debug_0() {
        assert_eq!(format!("{:?}", Sut::from_local_key_space(0).unwrap()), "0'");
    }

    #[test]
    fn display_max() {
        assert_eq!(
            format!("{}", Sut::from_local_key_space(U30_MAX).unwrap()),
            "1073741823H"
        );
    }

    #[test]
    fn debug_max() {
        assert_eq!(
            format!("{:?}", Sut::from_local_key_space(U30_MAX).unwrap()),
            "1073741823'"
        );
    }

    #[test]
    fn from_str_invalid() {
        assert!("".parse::<Sut>().is_err());
        assert!("foobar".parse::<Sut>().is_err());
        assert!("1X".parse::<Sut>().is_err());
        assert!("987654321987654321S".parse::<Sut>().is_err());
    }

    #[test]
    fn from_global_valid() {
        assert_eq!(
            Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED).unwrap(),
            Sut::from_local_key_space(0).unwrap()
        );

        assert_eq!(
            Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED + 1337).unwrap(),
            Sut::from_local_key_space(1337).unwrap()
        );
    }

    #[test]
    fn from_global_invalid() {
        assert!(Sut::from_global_key_space(0).is_err());
        assert!(Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED - 1).is_err());
    }

    #[test]
    fn index_in_local_key_space() {
        assert_eq!(
            Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED + 1337)
                .unwrap()
                .index_in_local_key_space(),
            1337
        );
    }

    #[test]
    fn into_global() {
        assert_eq!(
            Sut::from_local_key_space(1337)
                .unwrap()
                .into_global_key_space(),
            GLOBAL_OFFSET_HARDENED + 1337
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = Sut::from_local_key_space(1337).unwrap();

        assert_json_value_eq_after_roundtrip(&sut, json!("1337H"));
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(&sut, json!("0H"));
    }

    #[test]
    fn json_fails_for_invalid() {
        assert_json_value_fails::<Sut>(json!(""));
        assert_json_value_fails::<Sut>(json!("^"));
        assert_json_value_fails::<Sut>(json!("S"));
        assert_json_value_fails::<Sut>(json!("2"));
        assert_json_value_fails::<Sut>(json!("2^"));
        assert_json_value_fails::<Sut>(json!("2X"));
        assert_json_value_fails::<Sut>(json!("   "));
    }
}
