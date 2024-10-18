use crate::prelude::*;

#[derive(
    Clone,
    Default,
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
pub struct GetIDPath;

pub const GET_ID_LAST: HDPathComponent = unsafe { hard(365) };
impl GetIDPath {
    pub const PATH: [HDPathComponent; 3] = [PURPOSE, COIN_TYPE, GET_ID_LAST];
}

impl From<GetIDPath> for HDPath {
    fn from(_: GetIDPath) -> Self {
        Self::new(Vec::from_iter(GetIDPath::PATH))
    }
}
impl GetIDPath {
    fn to_hd_path(&self) -> HDPath {
        HDPath::from(self.clone())
    }
}
impl TryFrom<HDPath> for GetIDPath {
    type Error = CommonError;
    fn try_from(path: HDPath) -> Result<Self> {
        let _self = Self;
        if path == _self.to_hd_path() {
            Ok(_self)
        } else {
            Err(CommonError::Overflow)
        }
    }
}

impl ToBIP32Str for GetIDPath {
    fn to_bip32_string(&self) -> String {
        self.to_hd_path().to_bip32_string()
    }
    fn to_bip32_string_debug(&self) -> String {
        self.to_hd_path().to_bip32_string_debug()
    }
}
impl FromBIP32Str for GetIDPath {
    fn from_bip32_string(s: &str) -> Result<Self> {
        HDPath::from_bip32_string(s).and_then(Self::try_from)
    }
}
impl FromStr for GetIDPath {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    type Sut = GetIDPath;

    #[test]
    fn display() {
        assert_eq!(Sut::default().to_string(), "m/44H/1022H/365H");
    }

    #[test]
    fn from_str_canonical() {
        assert_eq!(Sut::from_str("m/44H/1022H/365H").unwrap(), Sut::default());
    }

    #[test]
    fn from_str_non_canonical() {
        assert_eq!(Sut::from_str("m/44'/1022'/365'").unwrap(), Sut::default());
    }

    #[test]
    fn equality_from_diff_string() {
        assert_eq!(
            Sut::from_str("m/44H/1022H/365H").unwrap(),
            Sut::from_str("m/44'/1022'/365'").unwrap()
        );
    }

    #[test]
    fn from_str_canonical_uppercase() {
        assert_eq!(Sut::from_str("M/44H/1022H/365H").unwrap(), Sut::default());
    }

    #[test]
    fn from_str_no_m() {
        assert_eq!(Sut::from_str("44H/1022H/365H").unwrap(), Sut::default());
    }

    #[test]
    fn from_str_leading_slash() {
        assert_eq!(Sut::from_str("/44H/1022H/365H").unwrap(), Sut::default());
    }

    #[test]
    fn from_str_trailing_slash() {
        assert_eq!(Sut::from_str("m/44H/1022H/365H/").unwrap(), Sut::default());
    }

    #[test]
    fn json_roundtrip() {
        let sut = Sut::default();

        assert_json_value_eq_after_roundtrip(&sut, json!("m/44H/1022H/365H"));
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
