use crate::prelude::*;

#[derive(
    Clone,
    PartialEq,
    EnumAsInner,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    SerializeDisplay,
    DeserializeFromStr,
    MoreDebug,
    derive_more::Display,
)]
pub enum CAP26Path {
    #[display("{value}")]
    #[debug("{:?}", value)]
    GetID { value: CAP26GetIDPath },

    #[display("{value}")]
    #[debug("{:?}", value)]
    Account { value: CAP26AccountPath },

    #[display("{value}")]
    #[debug("{:?}", value)]
    Identity { value: CAP26IdentityPath },
}

impl CAP26Path {
    pub fn get_id(path: CAP26GetIDPath) -> Self {
        Self::GetID { value: path }
    }
    pub fn account(path: CAP26AccountPath) -> Self {
        Self::Account { value: path }
    }
    pub fn identity(path: CAP26IdentityPath) -> Self {
        Self::Identity { value: path }
    }
}

impl FromStr for CAP26Path {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

impl FromBIP32Str for CAP26Path {
    fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
        let s = s.as_ref();
        CAP26IdentityPath::from_bip32_string(s)
            .map(Self::identity)
            .or(CAP26AccountPath::from_bip32_string(s).map(Self::account))
            .or(CAP26GetIDPath::from_bip32_string(s).map(Self::get_id))
    }
}

impl CAP26Path {
    pub fn to_hd_path(&self) -> HDPath {
        match self {
            Self::GetID { value } => value.to_hd_path(),
            Self::Account { value } => value.to_hd_path(),
            Self::Identity { value } => value.to_hd_path(),
        }
    }
}

impl ToBIP32Str for CAP26Path {
    fn to_bip32_string(&self) -> String {
        self.to_hd_path().to_bip32_string()
    }
    fn to_bip32_string_debug(&self) -> String {
        self.to_hd_path().to_bip32_string_debug()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = CAP26Path;

    #[test]
    fn test_to_bip32_string_is_display_account() {
        let sut = Sut::Account {
            value: CAP26AccountPath::sample(),
        };
        assert_eq!(sut.to_bip32_string(), format!("{}", sut));
    }

    #[test]
    fn test_to_bip32_string_is_debug_account() {
        let sut = Sut::Account {
            value: CAP26AccountPath::sample(),
        };
        assert_eq!(sut.to_bip32_string_debug(), format!("{:?}", sut));
    }

    #[test]
    fn test_to_bip32_string_is_display_identity() {
        let sut = Sut::Identity {
            value: CAP26IdentityPath::sample(),
        };
        assert_eq!(sut.to_bip32_string(), format!("{}", sut));
    }

    #[test]
    fn test_to_bip32_string_is_debug_identity() {
        let sut = Sut::Identity {
            value: CAP26IdentityPath::sample(),
        };
        assert_eq!(sut.to_bip32_string_debug(), format!("{:?}", sut));
    }

    #[test]
    fn string_roundtrip_account_from_account() {
        let value = CAP26AccountPath::sample();
        let s = value.to_bip32_string();
        let path2 = Sut::from_bip32_string(&s).unwrap();
        assert_eq!(Sut::Account { value }, path2);
    }

    #[test]
    fn string_roundtrip_account_from_cap26() {
        let sut = Sut::Account {
            value: CAP26AccountPath::sample(),
        };
        let s = sut.to_bip32_string();
        let value = CAP26AccountPath::from_bip32_string(&s).unwrap();
        assert_eq!(Sut::Account { value }, sut)
    }

    #[test]
    fn string_roundtrip_identity_from_identity() {
        let value = CAP26IdentityPath::sample();
        let s = value.to_bip32_string();
        let path2 = Sut::from_bip32_string(&s).unwrap();
        assert_eq!(Sut::Identity { value }, path2);
    }

    #[test]
    fn string_roundtrip_identity_from_cap26() {
        let sut = Sut::Identity {
            value: CAP26IdentityPath::sample(),
        };
        let s = sut.to_bip32_string();
        let value = CAP26IdentityPath::from_bip32_string(&s).unwrap();
        assert_eq!(Sut::Identity { value }, sut)
    }

    #[test]
    fn string_roundtrip_getid_from_getid() {
        let value = CAP26GetIDPath;
        let s = value.to_bip32_string();
        let path2 = Sut::from_bip32_string(&s).unwrap();
        assert_eq!(Sut::GetID { value }, path2);
    }

    #[test]
    fn string_roundtrip_getid_from_cap26() {
        let sut = Sut::GetID {
            value: CAP26GetIDPath,
        };
        let s = sut.to_bip32_string();
        let value = CAP26GetIDPath::from_bip32_string(&s).unwrap();
        assert_eq!(Sut::GetID { value }, sut)
    }
}
