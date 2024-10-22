use crate::prelude::*;

#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    MoreDebug,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
)]
#[display("{}", self.to_bip32_string())]
#[debug("{}", self.to_bip32_string_debug())]
pub struct Bip44LikePath {
    index: HDPathComponent,
}
impl Bip44LikePath {
    pub fn new(index: HDPathComponent) -> Self {
        Self { index }
    }
}

impl HasSampleValues for Bip44LikePath {
    fn sample() -> Self {
        Self::new(HDPathComponent::Unsecurified(Unsecurified::Unhardened(
            Unhardened::from_local_key_space(0u32).unwrap(),
        )))
    }
    fn sample_other() -> Self {
        Self::new(HDPathComponent::Unsecurified(Unsecurified::Hardened(
            UnsecurifiedHardened::from_local_key_space(1u32).unwrap(),
        )))
    }
}

impl Bip44LikePath {
    pub fn to_hd_path(&self) -> HDPath {
        bip44(self.index)
    }
}

impl From<Bip44LikePath> for HDPath {
    fn from(path: Bip44LikePath) -> Self {
        path.to_hd_path()
    }
}

impl TryFrom<HDPath> for Bip44LikePath {
    type Error = CommonError;
    fn try_from(path: HDPath) -> Result<Self> {
        let components = path.components();

        if components.len() != 5 {
            return Err(CommonError::InvalidLength);
        }
        if components[0] != PURPOSE {
            return Err(CommonError::InvalidPurpose);
        }
        if components[1] != COIN_TYPE {
            return Err(CommonError::InvalidCoinType);
        }
        let bip44_account = components[2];
        if bip44_account.is_unhardened() {
            return Err(CommonError::InvalidBip44ExpectedAccountComponentToBeHardened);
        }
        let bip44_change = components[3];

        if bip44_change.is_hardened() {
            return Err(CommonError::InvalidBip44ExpectedChangeComponentToNotBeHardened);
        }

        let index = components[4];

        Ok(Self::new(index))
    }
}

impl ToBIP32Str for Bip44LikePath {
    fn to_bip32_string(&self) -> String {
        self.to_hd_path().to_bip32_string()
    }
    fn to_bip32_string_debug(&self) -> String {
        self.to_hd_path().to_bip32_string_debug()
    }
}
impl FromBIP32Str for Bip44LikePath {
    fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
        HDPath::from_bip32_string(s).and_then(Self::try_from)
    }
}
impl FromStr for Bip44LikePath {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    type Sut = Bip44LikePath;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Sut::sample()), "m/44H/1022H/0H/0/0");
        assert_eq!(format!("{}", Sut::sample_other()), "m/44H/1022H/0H/0/1H");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", Sut::sample()), "m/44'/1022'/0'/0/0");
        assert_eq!(format!("{:?}", Sut::sample_other()), "m/44'/1022'/0'/0/1'");
    }

    #[test]
    fn to_bip32_path() {
        let sut = Sut::sample();
        assert_eq!(format!("{}", sut), "m/44H/1022H/0H/0/0");
    }

    #[test]
    fn from_str_hardened() {
        let sut = Sut::from_str("m/44H/1022H/0H/0/8H").unwrap();
        assert_eq!(
            sut.index,
            HDPathComponent::Unsecurified(Unsecurified::Hardened(
                UnsecurifiedHardened::from_local_key_space(8u32).unwrap(),
            ))
        );
    }

    #[test]
    fn from_str_hardened_non_canonical() {
        let sut = Sut::from_str("m/44'/1022'/0'/0/8'").unwrap();
        assert_eq!(
            sut.index,
            HDPathComponent::Unsecurified(Unsecurified::Hardened(
                UnsecurifiedHardened::from_local_key_space(8u32).unwrap(),
            ))
        );
    }

    #[test]
    fn from_str_unhardened() {
        let sut = Sut::from_str("m/44H/1022H/0H/0/6").unwrap();
        assert_eq!(
            sut.index,
            HDPathComponent::Unsecurified(Unsecurified::Unhardened(
                Unhardened::from_local_key_space(6u32).unwrap(),
            ))
        );
    }

    #[test]
    fn from_str_unhardened_non_canonical() {
        let sut = Sut::from_str("m/44'/1022'/0'/0/6").unwrap();
        assert_eq!(
            sut.index,
            HDPathComponent::Unsecurified(Unsecurified::Unhardened(
                Unhardened::from_local_key_space(6u32).unwrap(),
            ))
        );
    }

    #[test]
    fn json_roundtrip_unhardened() {
        let sut = Sut::sample();

        assert_json_value_eq_after_roundtrip(&sut, json!("m/44H/1022H/0H/0/0"));
        assert_json_roundtrip(&sut);
    }
    #[test]
    fn json_roundtrip_hardened() {
        let sut = Sut::sample_other();

        assert_json_value_eq_after_roundtrip(&sut, json!("m/44H/1022H/0H/0/1H"));
        assert_json_roundtrip(&sut);
    }

    #[test]
    fn json_fails_for_invalid() {
        assert_json_value_fails::<Sut>(json!(""));
        assert_json_value_fails::<Sut>(json!("foobar"));
        assert_json_value_fails::<Sut>(json!("^"));
        assert_json_value_fails::<Sut>(json!("S"));
        assert_json_value_fails::<Sut>(json!("2"));
        assert_json_value_fails::<Sut>(json!("2'"));
        assert_json_value_fails::<Sut>(json!("2X"));
        assert_json_value_fails::<Sut>(json!("   "));
    }
}
