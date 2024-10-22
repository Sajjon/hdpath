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
pub struct CAP26IdentityPath {
    network_id: NetworkID,
    key_kind: CAP26KeyKind,
    index: Hardened,
}

impl IsNetworkAware for CAP26IdentityPath {
    fn network_id(&self) -> NetworkID {
        self.network_id
    }
}

impl IsSecurityStateAware for CAP26IdentityPath {
    fn is_securified(&self) -> bool {
        self.index.is_securified()
    }
}

impl NewEntityPath for CAP26IdentityPath {
    fn new(
        network_id: impl Into<NetworkID>,
        key_kind: impl Into<CAP26KeyKind>,
        index: impl Into<Hardened>,
    ) -> Self {
        Self {
            network_id: network_id.into(),
            key_kind: key_kind.into(),
            index: index.into(),
        }
    }
}

impl TryFrom<HDPath> for CAP26IdentityPath {
    type Error = CommonError;
    fn try_from(path: HDPath) -> Result<Self> {
        UnvalidatedCAP26Path::try_from(path).and_then(Self::try_from_unvalidated)
    }
}

impl HasSampleValues for CAP26IdentityPath {
    fn sample() -> Self {
        Self::new(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            Hardened::from_local_key_space_unsecurified(0u32).unwrap(),
        )
    }
    fn sample_other() -> Self {
        Self::new(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            Hardened::from_local_key_space_unsecurified(1u32).unwrap(),
        )
    }
}

impl From<CAP26IdentityPath> for HDPath {
    fn from(identity_path: CAP26IdentityPath) -> Self {
        identity_path.to_hd_path()
    }
}

impl CAP26IdentityPath {
    pub fn to_hd_path(&self) -> HDPath {
        CAP26(
            self.network_id,
            Self::entity_kind(),
            self.key_kind,
            self.index,
        )
    }
}

impl HasEntityKind for CAP26IdentityPath {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Identity
    }
}

impl ToBIP32Str for CAP26IdentityPath {
    fn to_bip32_string(&self) -> String {
        self.to_hd_path().to_bip32_string()
    }
    fn to_bip32_string_debug(&self) -> String {
        self.to_hd_path().to_bip32_string_debug()
    }
}

impl FromBIP32Str for CAP26IdentityPath {
    fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
        HDPath::from_bip32_string(s).and_then(Self::try_from)
    }
}
impl FromStr for CAP26IdentityPath {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    type Sut = CAP26IdentityPath;

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
        let sut = Sut::sample();
        assert_eq!(format!("{}", sut), "m/44H/1022H/1H/618H/1460H/0H");
    }

    #[test]
    fn debug() {
        let sut = Sut::sample();
        assert_eq!(format!("{:?}", sut), "m/44'/1022'/1'/618'/1460'/0'");
    }

    #[test]
    fn to_bip32_path() {
        let sut = Sut::sample();
        assert_eq!(format!("{}", sut), "m/44H/1022H/1H/618H/1460H/0H");
    }

    #[test]
    fn from_str() {
        let sut = Sut::from_str("m/44H/1022H/1H/618H/1460H/0H").unwrap();
        assert_eq!(sut, Sut::sample());
    }

    #[test]
    fn from_str_securified() {
        let sut = Sut::from_str("m/44H/1022H/1H/618H/1460H/0S").unwrap();
        assert_ne!(sut, Sut::sample());
    }

    #[test]
    fn from_str_account() {
        assert!(matches!(
            Sut::from_str("m/44H/1022H/1H/525H/1460H/0H"),
            Err(CommonError::WrongEntityKind {
                expected: CAP26EntityKind::Identity,
                found: CAP26EntityKind::Account
            })
        ))
    }

    #[test]
    fn json_roundtrip() {
        let sut = Sut::sample();

        assert_json_value_eq_after_roundtrip(&sut, json!("m/44H/1022H/1H/618H/1460H/0H"));
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

    #[test]
    fn is_network_aware() {
        assert_eq!(
            Sut::new(
                NetworkID::Stokenet,
                CAP26KeyKind::sample(),
                Hardened::sample()
            )
            .network_id(),
            NetworkID::Stokenet
        );
    }

    #[test]
    fn is_security_aware_unsecurified() {
        assert!(!Sut::new(
            NetworkID::Stokenet,
            CAP26KeyKind::sample(),
            Hardened::sample()
        )
        .is_securified(),);
    }

    #[test]
    fn is_security_aware_securified() {
        assert!(Sut::new(
            NetworkID::Stokenet,
            CAP26KeyKind::sample(),
            Hardened::sample_other()
        )
        .is_securified());
    }

    #[test]
    fn entity_kind() {
        assert_eq!(Sut::entity_kind(), CAP26EntityKind::Identity);
    }

    #[test]
    fn get_entity_kind() {
        assert_eq!(Sut::sample().get_entity_kind(), CAP26EntityKind::Identity);
    }
}
