use crate::prelude::*;

/// ```ignore
/// [ <<------- UNHARDENED ------->> | <<-------- HARDENED --------->> ]
/// [ <<------------ UNSECURIFIED ---|-------->>  | <<- SECURIFIED ->> ]
/// ^                                ^            ^                    ^
/// 0                              2^31       2^31+2^30          2^32+1
///
/// ```
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumAsInner,
    Display,
    MoreDebug,
    DeserializeFromStr,
    SerializeDisplay,
)]
pub enum HDPathComponent {
    #[display("{_0}")]
    #[debug("{_0}")]
    Unsecurified(Unsecurified),

    #[display("{_0}")]
    #[debug("{_0}")]
    Securified(SecurifiedU30),
}

impl HasSampleValues for HDPathComponent {
    fn sample() -> Self {
        Self::Unsecurified(Unsecurified::sample())
    }

    fn sample_other() -> Self {
        Self::Securified(SecurifiedU30::sample_other())
    }
}

impl HasIndexInLocalKeySpace for HDPathComponent {
    fn index_in_local_key_space(&self) -> u32 {
        match self {
            Self::Unsecurified(u) => u.index_in_local_key_space(),
            Self::Securified(s) => s.index_in_local_key_space(),
        }
    }
}

impl FromGlobalKeySpace for HDPathComponent {
    fn from_global_key_space(value: u32) -> Result<Self> {
        SecurifiedU30::from_global_key_space(value)
            .map(Self::Securified)
            .or(Unsecurified::from_global_key_space(value).map(Self::Unsecurified))
    }
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

impl FromBIP32Str for HDPathComponent {
    fn from_bip32_string(s: &str) -> Result<Self> {
        SecurifiedU30::from_bip32_string(s)
            .map(Self::securified)
            .or(Unsecurified::from_bip32_string(s).map(Self::unsecurified))
    }
}

impl FromStr for HDPathComponent {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
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

impl HDPathComponent {
    pub fn from_local_key_space(
        value: u32,
        is_hardened: bool,
        is_securified: bool,
    ) -> Result<Self> {
        assert!(!is_securified || is_hardened);
        if is_securified {
            SecurifiedU30::from_local_key_space(value).map(Self::Securified)
        } else {
            Unsecurified::from_local_key_space(value, is_hardened).map(Self::Unsecurified)
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    type Sut = HDPathComponent;

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
    fn unsecurified_unhardened_from_local() {
        assert_eq!(
            Sut::from_local_key_space(0, false, false).unwrap(),
            Sut::from_global_key_space(0).unwrap()
        );

        assert_eq!(
            Sut::from_local_key_space(3, false, false).unwrap(),
            Sut::from_global_key_space(3).unwrap()
        );
    }

    #[test]
    fn unsecurified_hardened_from_local() {
        assert_eq!(
            Sut::from_local_key_space(0, true, false).unwrap(),
            Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED).unwrap()
        );

        assert_eq!(
            Sut::from_local_key_space(3, true, false).unwrap(),
            Sut::from_global_key_space(3 + GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn securified_hardened_from_local() {
        assert_eq!(
            Sut::from_local_key_space(0, true, true).unwrap(),
            Sut::from_global_key_space(GLOBAL_OFFSET_SECURIFIED).unwrap()
        );

        assert_eq!(
            Sut::from_local_key_space(3, true, true).unwrap(),
            Sut::from_global_key_space(3 + GLOBAL_OFFSET_SECURIFIED).unwrap()
        );
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
            Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED).unwrap()
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
        assert!("987654321987654321".parse::<Sut>().is_err());
    }

    #[test]
    fn from_global() {
        assert_eq!(
            Sut::from_global_key_space(1337).unwrap(),
            Sut::Unsecurified(Unsecurified::Unhardened(
                Unhardened::from_local_key_space(1337).unwrap()
            ))
        );

        assert_eq!(
            Sut::from_global_key_space(42 + GLOBAL_OFFSET_HARDENED).unwrap(),
            Sut::Unsecurified(Unsecurified::Hardened(
                UnsecurifiedHardened::from_local_key_space(42).unwrap()
            ))
        );

        assert_eq!(
            Sut::from_global_key_space(237 + GLOBAL_OFFSET_SECURIFIED).unwrap(),
            Sut::Securified(SecurifiedU30::from_local_key_space(237).unwrap())
        );
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
    fn json_roundtrip_unhardened() {
        let sut = Sut::from_global_key_space(1337).unwrap();

        assert_json_value_eq_after_roundtrip(&sut, json!("1337"));
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(&sut, json!("0"));
    }

    #[test]
    fn json_roundtrip_hardened_unsecurified() {
        let sut = Sut::from_global_key_space(6 + GLOBAL_OFFSET_HARDENED).unwrap();
        assert_json_value_eq_after_roundtrip(&sut, json!("6H"));
    }

    #[test]
    fn json_roundtrip_securified() {
        let sut = Sut::from_global_key_space(5109 + GLOBAL_OFFSET_SECURIFIED).unwrap();
        assert_json_value_eq_after_roundtrip(&sut, json!("5109S"));
    }

    #[test]
    fn json_fails_for_invalid() {
        assert_json_value_fails::<Sut>(json!(""));
        assert_json_value_fails::<Sut>(json!("^"));
        assert_json_value_fails::<Sut>(json!("2X"));
        assert_json_value_fails::<Sut>(json!("   "));
    }

    #[test]
    #[should_panic]
    fn from_local_key_space_panics_for_invalid() {
        _ = Sut::from_local_key_space(0, false, true);
    }
}
