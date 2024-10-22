use crate::prelude::*;

macro_rules! path_union {
    (
        $(
            #[doc = $expr: expr]
        )*
        $union_name: ident,
        $(
            $variant_name: ident,
            $variant_type: ty
        )+
    ) => {
        paste::paste! {
            $(
                #[doc = $expr]
            )*
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
            pub enum $union_name {
                $(
                    #[display("{value}")]
                    #[debug("{:?}", value)]
                    $variant_name { value: $variant_type },
                )+
            }

            impl $union_name {
                $(
                    pub fn [< $variant_name:snake >](path: $variant_type) -> Self {
                        Self::$variant_name { value: path }
                    }
                )+
            }

            impl FromStr for $union_name {
                type Err = CommonError;
                fn from_str(s: &str) -> Result<Self> {
                    Self::from_bip32_string(s)
                }
            }

            impl $union_name {
                pub fn to_hd_path(&self) -> HDPath {
                    match self {
                        $(
                            Self::$variant_name { value } => value.to_hd_path(),
                        )+
                    }
                }
            }

            impl ToBIP32Str for $union_name {
                fn to_bip32_string(&self) -> String {
                    self.to_hd_path().to_bip32_string()
                }
                fn to_bip32_string_debug(&self) -> String {
                    self.to_hd_path().to_bip32_string_debug()
                }
            }

            impl FromBIP32Str for $union_name {
                fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
                    let s = s.as_ref();
                    let o = Result::<Self>::Err(CommonError::Overflow);

                    o
                    $(
                        .or($variant_type::from_bip32_string(s).map(Self::[< $variant_name:snake >]))
                    )+

                }
            }

        }
    };


}

path_union!(
    DerivationPath,
    Account, CAP26AccountPath
    Identity, CAP26IdentityPath
    BIP44Like, BIP44LikePath
);

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
    fn string_roundtrip_account_from_CAP26() {
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
    fn string_roundtrip_identity_from_CAP26() {
        let sut = Sut::Identity {
            value: CAP26IdentityPath::sample(),
        };
        let s = sut.to_bip32_string();
        let value = CAP26IdentityPath::from_bip32_string(&s).unwrap();
        assert_eq!(Sut::Identity { value }, sut)
    }

    #[test]
    fn string_roundtrip_BIP44_from_BIP44() {
        let value = BIP44LikePath::sample();
        let s = value.to_bip32_string();
        let path2 = Sut::from_bip32_string(&s).unwrap();
        assert_eq!(Sut::BIP44Like { value }, path2);
    }

    #[test]
    fn string_roundtrip_getid_from_CAP26() {
        let sut = Sut::BIP44Like {
            value: BIP44LikePath::sample(),
        };
        let s = sut.to_bip32_string();
        let value = BIP44LikePath::from_bip32_string(&s).unwrap();
        assert_eq!(Sut::BIP44Like { value }, sut)
    }
}
