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
    #[debug("{:?}", _0)]
    Unsecurified(Unsecurified),

    #[display("{_0}")]
    #[debug("{:?}", _0)]
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

impl IsMappableToLocalKeySpace for HDPathComponent {
    fn map_to_local_key_space(&self) -> KeySpaceWithLocalIndex {
        match self {
            Self::Unsecurified(u) => u.map_to_local_key_space(),
            Self::Securified(s) => s.map_to_local_key_space(),
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
    fn map_to_global_key_space(&self) -> u32 {
        match self {
            HDPathComponent::Unsecurified(u) => u.map_to_global_key_space(),
            HDPathComponent::Securified(s) => s.map_to_global_key_space(),
        }
    }
}

impl HDPathComponent {
    pub fn from_local_key_space(
        u31: impl TryInto<U31, Error = CommonError>,
        key_space: KeySpace,
    ) -> Result<Self> {
        match key_space {
            KeySpace::Securified => {
                let u31 = u31.try_into().map_err(|_| CommonError::Overflow)?;
                SecurifiedU30::from_local_key_space(u31).map(Self::Securified)
            }
            KeySpace::Unsecurified { is_hardened } => {
                Unsecurified::from_local_key_space(u31, is_hardened).map(Self::Unsecurified)
            }
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
    fn map_to_local() {
        let sut = Sut::Securified(SecurifiedU30::sample());
        assert!(sut.map_to_local_key_space().key_space().is_securified())
    }

    #[test]
    fn unsecurified_unhardened_from_local() {
        assert_eq!(
            Sut::from_local_key_space(0u32, KeySpace::Unsecurified { is_hardened: false }).unwrap(),
            Sut::from_global_key_space(0).unwrap()
        );

        assert_eq!(
            Sut::from_local_key_space(3u32, KeySpace::Unsecurified { is_hardened: false }).unwrap(),
            Sut::from_global_key_space(3).unwrap()
        );
    }

    #[test]
    fn unsecurified_hardened_from_local() {
        assert_eq!(
            Sut::from_local_key_space(0u32, KeySpace::Unsecurified { is_hardened: true }).unwrap(),
            Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED).unwrap()
        );

        assert_eq!(
            Sut::from_local_key_space(3u32, KeySpace::Unsecurified { is_hardened: true }).unwrap(),
            Sut::from_global_key_space(3 + GLOBAL_OFFSET_HARDENED).unwrap()
        );
    }

    #[test]
    fn from_hardened() {
        let sec = SecurifiedU30::sample();
        let hardened = Hardened::Securified(sec);
        assert_eq!(Sut::from(hardened), Sut::Securified(sec));
    }

    #[test]
    fn map_to_global_securified() {
        let sec = SecurifiedU30::sample();
        let sut = Sut::Securified(sec);
        assert_eq!(sut.map_to_global_key_space(), 30 + GLOBAL_OFFSET_SECURIFIED);
    }

    #[test]
    fn from_local_key_space_securified() {
        assert_eq!(
            Sut::from_local_key_space(42u32, KeySpace::Securified).unwrap(),
            Sut::securified(U30::try_from(42u32).unwrap())
        )
    }

    #[test]
    fn securified_hardened_from_local() {
        assert_eq!(
            Sut::from_local_key_space(0u32, KeySpace::Securified).unwrap(),
            Sut::from_global_key_space(GLOBAL_OFFSET_SECURIFIED).unwrap()
        );

        assert_eq!(
            Sut::from_local_key_space(3u32, KeySpace::Securified).unwrap(),
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
                Unhardened::from_local_key_space(1337u32).unwrap()
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
    fn map_to_local_key_space() {
        let local = Sut::from_global_key_space(1337)
            .unwrap()
            .map_to_local_key_space();
        assert!(local.key_space().is_unsecurified_unhardened());
        assert_eq!(local.index(), U31::from(1337));
    }

    #[test]
    fn index_in_local_key_space() {
        assert_eq!(
            Sut::from_global_key_space(1337)
                .unwrap()
                .index_in_local_key_space(),
            U31::from(1337)
        );
    }

    #[test]
    fn into_global() {
        assert_eq!(
            Sut::from_global_key_space(1337)
                .unwrap()
                .map_to_global_key_space(),
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
}
