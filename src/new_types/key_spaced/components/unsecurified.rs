use crate::prelude::*;

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    MoreDebug,
    DeserializeFromStr,
    SerializeDisplay,
)]
pub enum Unsecurified {
    #[display("{_0}")]
    #[debug("{_0}")]
    Unhardened(Unhardened),

    #[display("{_0}")]
    #[debug("{_0}")]
    Hardened(UnsecurifiedHardened),
}

impl HasSampleValues for Unsecurified {
    fn sample() -> Self {
        Self::Unhardened(Unhardened::sample())
    }

    fn sample_other() -> Self {
        Self::Hardened(UnsecurifiedHardened::sample_other())
    }
}

impl HasIndexInLocalKeySpace for Unsecurified {
    fn index_in_local_key_space(&self) -> u32 {
        match self {
            Self::Unhardened(u) => u.index_in_local_key_space(),
            Self::Hardened(h) => h.index_in_local_key_space(),
        }
    }
}

impl FromGlobalKeySpace for Unsecurified {
    fn from_global_key_space(value: u32) -> Result<Self> {
        UnsecurifiedHardened::from_global_key_space(value)
            .map(Self::Hardened)
            .or(Unhardened::from_global_key_space(value).map(Self::Unhardened))
    }
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

impl FromBIP32Str for Unsecurified {
    fn from_bip32_string(s: &str) -> Result<Self> {
        UnsecurifiedHardened::from_bip32_string(s)
            .map(Self::Hardened)
            .or(Unhardened::from_bip32_string(s).map(Self::Unhardened))
    }
}

impl FromStr for Unsecurified {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    type Sut = Unsecurified;

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
    fn from_str_valid_0_unhardened() {
        assert_eq!(
            "0".parse::<Sut>().unwrap(),
            Sut::from_global_key_space(0).unwrap()
        );
    }

    #[test]
    fn from_str_valid_1_unhardened() {
        assert_eq!(
            "1".parse::<Sut>().unwrap(),
            Sut::from_global_key_space(1).unwrap()
        );
    }

    #[test]
    fn from_str_valid_0_hardened_canonical() {
        assert_eq!(
            "0H".parse::<Sut>().unwrap(),
            Sut::from_global_key_space(0 + GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn from_str_valid_1_hardened_canonical() {
        assert_eq!(
            "1H".parse::<Sut>().unwrap(),
            Sut::from_global_key_space(1 + GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn from_str_valid_2_hardened_non_canonical() {
        assert_eq!(
            "2'".parse::<Sut>().unwrap(),
            Sut::from_global_key_space(2 + GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn from_str_valid_3_hardened_non_canonical() {
        assert_eq!(
            "3'".parse::<Sut>().unwrap(),
            Sut::from_global_key_space(3 + GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn from_str_valid_max() {
        assert_eq!(
            "2147483647".parse::<Sut>().unwrap(),
            Sut::from_global_key_space(U31_MAX).unwrap()
        );
    }

    #[test]
    fn display_0() {
        assert_eq!(format!("{}", Sut::from_global_key_space(0).unwrap()), "0");
    }

    #[test]
    fn debug_0() {
        assert_eq!(format!("{:?}", Sut::from_global_key_space(0).unwrap()), "0");
    }

    #[test]
    fn display_max() {
        assert_eq!(
            format!("{}", Sut::from_global_key_space(U30_MAX).unwrap()),
            "1073741823"
        );
    }

    #[test]
    fn debug_max() {
        assert_eq!(
            format!("{:?}", Sut::from_global_key_space(U30_MAX).unwrap()),
            "1073741823"
        );
    }

    #[test]
    fn from_str_invalid() {
        assert!("".parse::<Sut>().is_err());
        assert!("foobar".parse::<Sut>().is_err());
        assert!("1S".parse::<Sut>().is_err());
        assert!("1^".parse::<Sut>().is_err());
        assert!("987654321987654321".parse::<Sut>().is_err());
    }

    #[test]
    fn from_global_valid() {
        assert_eq!(
            Sut::from_global_key_space(1337).unwrap(),
            Sut::Unhardened(Unhardened::from_global_key_space(1337).unwrap())
        );
    }

    #[test]
    fn from_global_invalid() {
        assert!(Sut::from_global_key_space(GLOBAL_OFFSET_SECURIFIED).is_err());
    }

    #[test]
    fn index_in_local_key_space() {
        assert_eq!(
            Sut::from_global_key_space(1337)
                .unwrap()
                .index_in_local_key_space(),
            1337
        );
    }

    #[test]
    fn into_global() {
        assert_eq!(
            Sut::from_global_key_space(1337)
                .unwrap()
                .into_global_key_space(),
            1337
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = Sut::from_global_key_space(1337).unwrap();

        assert_json_value_eq_after_roundtrip(&sut, json!("1337"));
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(&sut, json!("0"));
    }

    #[test]
    fn json_fails_for_invalid() {
        assert_json_value_fails::<Sut>(json!(""));
        assert_json_value_fails::<Sut>(json!("^"));
        assert_json_value_fails::<Sut>(json!("2S"));
        assert_json_value_fails::<Sut>(json!("2X"));
        assert_json_value_fails::<Sut>(json!("   "));
    }
}
