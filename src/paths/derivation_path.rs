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
pub enum DerivationPath {
    #[display("{value}")]
    #[debug("{:?}", value)]
    BIP44Like { value: BIP44LikePath },

    #[display("{value}")]
    #[debug("{:?}", value)]
    Account { value: CAP26AccountPath },

    #[display("{value}")]
    #[debug("{:?}", value)]
    Identity { value: CAP26IdentityPath },
}

impl DerivationPath {
    pub fn bip44_like(path: BIP44LikePath) -> Self {
        Self::BIP44Like { value: path }
    }
    pub fn account(path: CAP26AccountPath) -> Self {
        Self::Account { value: path }
    }
    pub fn identity(path: CAP26IdentityPath) -> Self {
        Self::Identity { value: path }
    }
}

impl FromStr for DerivationPath {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

impl FromBIP32Str for DerivationPath {
    fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
        let s = s.as_ref();
        CAP26IdentityPath::from_bip32_string(s)
            .map(Self::identity)
            .or(CAP26AccountPath::from_bip32_string(s).map(Self::account))
            .or(BIP44LikePath::from_bip32_string(s).map(Self::bip44_like))
    }
}

impl DerivationPath {
    pub fn to_hd_path(&self) -> HDPath {
        match self {
            Self::BIP44Like { value } => value.to_hd_path(),
            Self::Account { value } => value.to_hd_path(),
            Self::Identity { value } => value.to_hd_path(),
        }
    }
}

impl ToBIP32Str for DerivationPath {
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

    type Sut = DerivationPath;

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
    fn string_roundtrip_bip44_from_bip44() {
        let value = BIP44LikePath::sample();
        let s = value.to_bip32_string();
        let path2 = Sut::from_bip32_string(&s).unwrap();
        assert_eq!(Sut::BIP44Like { value }, path2);
    }

    #[test]
    fn string_roundtrip_getid_from_cap26() {
        let sut = Sut::BIP44Like {
            value: BIP44LikePath::sample(),
        };
        let s = sut.to_bip32_string();
        let value = BIP44LikePath::from_bip32_string(&s).unwrap();
        assert_eq!(Sut::BIP44Like { value }, sut)
    }
}
