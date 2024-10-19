use crate::prelude::*;

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
pub enum Hardened {
    #[display("{_0}")]
    #[debug("{:?}", _0)]
    Unsecurified(UnsecurifiedHardened),

    #[display("{_0}")]
    #[debug("{:?}", _0)]
    Securified(SecurifiedU30),
}

impl HasSampleValues for Hardened {
    fn sample() -> Self {
        Self::Unsecurified(UnsecurifiedHardened::sample())
    }

    fn sample_other() -> Self {
        Self::Securified(SecurifiedU30::sample_other())
    }
}

impl IsMappableToGlobalKeySpace for Hardened {
    fn map_to_global_key_space(self) -> u32 {
        match self {
            Self::Unsecurified(u) => u.map_to_global_key_space(),
            Self::Securified(s) => s.map_to_global_key_space(),
        }
    }
}

impl IsMappableToLocalKeySpace for Hardened {
    fn map_to_local_key_space(&self) -> KeySpaceWithLocalIndex {
        match self {
            Self::Unsecurified(u) => u.map_to_local_key_space(),
            Self::Securified(s) => s.map_to_local_key_space(),
        }
    }
}

impl FromGlobalKeySpace for Hardened {
    fn from_global_key_space(value: u32) -> Result<Self> {
        SecurifiedU30::from_global_key_space(value)
            .map(Self::Securified)
            .or(UnsecurifiedHardened::from_global_key_space(value).map(Self::Unsecurified))
    }
}

impl FromBIP32Str for Hardened {
    fn from_bip32_string(s: &str) -> Result<Self> {
        SecurifiedU30::from_bip32_string(s)
            .map(Self::Securified)
            .or(UnsecurifiedHardened::from_bip32_string(s).map(Self::Unsecurified))
    }
}

impl Hardened {
    pub fn from_local_key_space_unsecurified(value: u32) -> Result<Self> {
        Self::from_local_key_space(value, false)
    }
    pub fn from_local_key_space(value: u32, is_securified: bool) -> Result<Self> {
        if is_securified {
            SecurifiedU30::from_local_key_space(value).map(Self::Securified)
        } else {
            UnsecurifiedHardened::from_local_key_space(value).map(Self::Unsecurified)
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

impl FromStr for Hardened {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    type Sut = Hardened;

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
    fn unsecurified_from_local() {
        assert_eq!(
            Sut::from_local_key_space(1, false).unwrap(),
            Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED + 1).unwrap()
        );

        assert_eq!(
            Sut::from_local_key_space(3, false).unwrap(),
            Sut::from_global_key_space(GLOBAL_OFFSET_HARDENED + 3).unwrap()
        );
    }

    #[test]
    fn securified_from_local() {
        assert_eq!(
            Sut::from_local_key_space(0, true).unwrap(),
            Sut::from_global_key_space(GLOBAL_OFFSET_SECURIFIED).unwrap()
        );

        assert_eq!(
            Sut::from_local_key_space(3, true).unwrap(),
            Sut::from_global_key_space(3 + GLOBAL_OFFSET_SECURIFIED).unwrap()
        );
    }

    #[test]
    fn from_str_valid_0_unsecurified() {
        assert_eq!(
            "0H".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(0, false).unwrap()
        );
    }

    #[test]
    fn from_str_valid_0_securified() {
        assert_eq!(
            "0S".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(0, true).unwrap()
        );
    }

    #[test]
    fn from_str_valid_1_securified_canonical() {
        assert_eq!(
            "1S".parse::<Sut>().unwrap(),
            Sut::from_global_key_space(1 + GLOBAL_OFFSET_SECURIFIED).unwrap()
        );
    }
    #[test]
    fn from_str_valid_1_securified_non_canonical() {
        assert_eq!(
            "1^".parse::<Sut>().unwrap(),
            Sut::from_global_key_space(1 + GLOBAL_OFFSET_SECURIFIED).unwrap()
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
            "1073741823S".parse::<Sut>().unwrap(),
            Sut::from_local_key_space(U30_MAX, true).unwrap()
        );
    }

    #[test]
    fn display_unsec_1() {
        assert_eq!(
            format!(
                "{}",
                Sut::from_global_key_space(1 + GLOBAL_OFFSET_HARDENED).unwrap()
            ),
            "1H"
        );
    }
    #[test]
    fn display_unsec_2() {
        assert_eq!(
            format!(
                "{}",
                Sut::from_global_key_space(2 + GLOBAL_OFFSET_HARDENED).unwrap()
            ),
            "2H"
        );
    }

    #[test]
    fn display_sec_1() {
        assert_eq!(
            format!(
                "{}",
                Sut::from_global_key_space(1 + GLOBAL_OFFSET_SECURIFIED).unwrap()
            ),
            "1S"
        );
    }
    #[test]
    fn display_sec_2() {
        assert_eq!(
            format!(
                "{}",
                Sut::from_global_key_space(2 + GLOBAL_OFFSET_SECURIFIED).unwrap()
            ),
            "2S"
        );
    }

    #[test]
    fn debug_unsec_0() {
        assert_eq!(
            format!("{:?}", Sut::from_local_key_space(0, false).unwrap()),
            "0'"
        );
    }

    #[test]
    fn debug_sec_0() {
        assert_eq!(
            format!("{:?}", Sut::from_local_key_space(0, true).unwrap()),
            "0^"
        );
    }

    #[test]
    fn display_unsec_0() {
        assert_eq!(
            format!("{}", Sut::from_local_key_space(0, false).unwrap()),
            "0H"
        );
    }

    #[test]
    fn display_sec_0() {
        assert_eq!(
            format!("{}", Sut::from_local_key_space(0, true).unwrap()),
            "0S"
        );
    }

    #[test]
    fn from_str_invalid() {
        assert!("".parse::<Sut>().is_err());
        assert!("foobar".parse::<Sut>().is_err());
        assert!("1".parse::<Sut>().is_err());
        assert!("987654321987654321".parse::<Sut>().is_err());
    }

    #[test]
    fn from_global_valid_securified() {
        assert_eq!(
            Sut::from_global_key_space(1337 + GLOBAL_OFFSET_SECURIFIED).unwrap(),
            Sut::Securified(SecurifiedU30::from_local_key_space(1337).unwrap())
        );
    }

    #[test]
    fn from_global_valid_unsecurified() {
        assert_eq!(
            Sut::from_global_key_space(1337 + GLOBAL_OFFSET_HARDENED).unwrap(),
            Sut::Unsecurified(UnsecurifiedHardened::from_local_key_space(1337).unwrap())
        );
    }

    #[test]
    fn from_global_invalid() {
        assert!(Sut::from_global_key_space(0).is_err());
    }

    #[test]
    fn index_in_local_key_space_unsecurified() {
        assert_eq!(
            Sut::from_global_key_space(1337 + GLOBAL_OFFSET_HARDENED)
                .unwrap()
                .index_in_local_key_space(),
            U31::from(1337)
        );
    }

    #[test]
    fn index_in_local_key_space_unsecurified_key_space() {
        assert_eq!(
            Sut::from_global_key_space(1337 + GLOBAL_OFFSET_HARDENED)
                .unwrap()
                .map_to_local_key_space()
                .key_space(),
            KeySpace::Unsecurified { is_hardened: true }
        );
    }

    #[test]
    fn index_in_local_key_space_securified_key_space() {
        assert_eq!(
            Sut::from_global_key_space(1337 + GLOBAL_OFFSET_SECURIFIED)
                .unwrap()
                .map_to_local_key_space()
                .key_space(),
            KeySpace::Securified
        );
    }

    #[test]
    fn try_from_hd_path_component_securified() {
        let secu30 = SecurifiedU30::sample();
        let sut = Sut::Securified(secu30);
        let from = HDPathComponent::Securified(secu30);
        assert_eq!(Sut::try_from(from).unwrap(), sut)
    }

    #[test]
    fn index_in_local_key_space_securified() {
        assert_eq!(
            Sut::from_global_key_space(1337 + GLOBAL_OFFSET_SECURIFIED)
                .unwrap()
                .index_in_local_key_space(),
            U31::from(1337)
        );
    }

    #[test]
    fn into_global_unsecurified() {
        assert_eq!(
            Sut::from_global_key_space(1337 + GLOBAL_OFFSET_HARDENED)
                .unwrap()
                .map_to_global_key_space(),
            1337 + GLOBAL_OFFSET_HARDENED
        );
    }

    #[test]
    fn into_global_securified() {
        assert_eq!(
            Sut::from_global_key_space(1337 + GLOBAL_OFFSET_SECURIFIED)
                .unwrap()
                .map_to_global_key_space(),
            1337 + GLOBAL_OFFSET_SECURIFIED
        );
    }

    #[test]
    fn json_roundtrip_securified() {
        let sut = Sut::from_local_key_space(1337, true).unwrap();

        assert_json_value_eq_after_roundtrip(&sut, json!("1337S"));
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(&sut, json!("0S"));
    }

    #[test]
    fn json_roundtrip_unsecurified() {
        let sut = Sut::from_local_key_space(1337, false).unwrap();

        assert_json_value_eq_after_roundtrip(&sut, json!("1337H"));
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(&sut, json!("0H"));
    }

    #[test]
    fn json_fails_for_invalid() {
        assert_json_value_fails::<Sut>(json!(""));
        assert_json_value_fails::<Sut>(json!("^"));
        assert_json_value_fails::<Sut>(json!("2"));
        assert_json_value_fails::<Sut>(json!("2X"));
        assert_json_value_fails::<Sut>(json!("   "));
    }
}
