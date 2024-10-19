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
pub struct CAP26AccountPath {
    network_id: NetworkID,
    key_kind: CAP26KeyKind,
    index: Hardened,
}

impl IsNetworkAware for CAP26AccountPath {
    fn network_id(&self) -> NetworkID {
        self.network_id
    }
}

impl IsSecurityStateAware for CAP26AccountPath {
    fn is_securified(&self) -> bool {
        self.index.is_securified()
    }
}

pub trait NewEntityPath: Sized {
    fn new(
        network_id: impl Into<NetworkID>,
        key_kind: impl Into<CAP26KeyKind>,
        index: impl Into<Hardened>,
    ) -> Self;
}
pub trait NewEntityPathCheckingEntityKind: NewEntityPath {
    fn try_from_unvalidated(path: UnvalidatedCAP26Path) -> Result<Self>;
}

impl NewEntityPath for CAP26AccountPath {
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

/// A derivation path consisting of CAP26 components, alas, not validated
/// as canonical.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct UnvalidatedCAP26Path {
    pub network_id: NetworkID,
    pub entity_kind: CAP26EntityKind,
    pub key_kind: CAP26KeyKind,
    pub index: Hardened,
}

impl<T: HasEntityKind + NewEntityPath> NewEntityPathCheckingEntityKind for T {
    fn try_from_unvalidated(path: UnvalidatedCAP26Path) -> Result<Self> {
        let entity_kind = path.entity_kind;
        if entity_kind != Self::entity_kind() {
            return Err(CommonError::WrongEntityKind {
                expected: Self::entity_kind(),
                found: entity_kind,
            });
        }
        Ok(Self::new(path.network_id, path.key_kind, path.index))
    }
}

impl TryFrom<HDPath> for CAP26AccountPath {
    type Error = CommonError;
    fn try_from(path: HDPath) -> Result<Self> {
        UnvalidatedCAP26Path::try_from(path).and_then(Self::try_from_unvalidated)
    }
}
impl HasSampleValues for CAP26AccountPath {
    fn sample() -> Self {
        Self::new(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            Hardened::from_local_key_space_unsecurified(0).unwrap(),
        )
    }
    fn sample_other() -> Self {
        Self::new(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            Hardened::from_local_key_space_unsecurified(1).unwrap(),
        )
    }
}

impl From<CAP26AccountPath> for HDPath {
    fn from(account_path: CAP26AccountPath) -> Self {
        account_path.to_hd_path()
    }
}
impl From<NetworkID> for HDPathComponent {
    fn from(value: NetworkID) -> Self {
        unsafe { hard(value.discriminant() as u32) }
    }
}
impl From<CAP26EntityKind> for HDPathComponent {
    fn from(value: CAP26EntityKind) -> Self {
        unsafe { hard(value.discriminant()) }
    }
}
impl From<CAP26KeyKind> for HDPathComponent {
    fn from(value: CAP26KeyKind) -> Self {
        unsafe { hard(value.discriminant()) }
    }
}

impl CAP26AccountPath {
    fn to_hd_path(&self) -> HDPath {
        cap26(
            self.network_id,
            Self::entity_kind(),
            self.key_kind,
            self.index,
        )
    }
}

impl HasEntityKind for CAP26AccountPath {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Account
    }
}

impl ToBIP32Str for CAP26AccountPath {
    fn to_bip32_string(&self) -> String {
        self.to_hd_path().to_bip32_string()
    }
    fn to_bip32_string_debug(&self) -> String {
        self.to_hd_path().to_bip32_string_debug()
    }
}

impl FromBIP32Str for CAP26AccountPath {
    fn from_bip32_string(s: &str) -> Result<Self> {
        HDPath::from_bip32_string(s).and_then(Self::try_from)
    }
}
impl FromStr for CAP26AccountPath {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    type Sut = CAP26AccountPath;

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
    fn to_string() {
        let sut = Sut::sample();
        assert_eq!(format!("{}", sut), "m/44H/1022H/1H/525H/1460H/0H");
    }

    #[test]
    fn to_bip32_path() {
        let sut = Sut::sample();
        assert_eq!(format!("{}", sut), "m/44H/1022H/1H/525H/1460H/0H");
    }

    #[test]
    fn from_str() {
        let sut = Sut::from_str("m/44H/1022H/1H/525H/1460H/0H").unwrap();
        assert_eq!(sut, Sut::sample());
    }

    #[test]
    fn json_roundtrip() {
        let sut = Sut::sample();

        assert_json_value_eq_after_roundtrip(&sut, json!("m/44H/1022H/1H/525H/1460H/0H"));
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
}
