use crate::prelude::*;

/// A derivation path consisting of Cap26 components, alas, not validated
/// as canonical.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct UnvalidatedCap26Path {
    pub network_id: NetworkID,
    pub entity_kind: Cap26EntityKind,
    pub key_kind: Cap26KeyKind,
    pub index: Hardened,
}

impl TryFrom<HDPath> for UnvalidatedCap26Path {
    type Error = CommonError;

    fn try_from(path: HDPath) -> Result<Self> {
        let components = path.components();
        if components.iter().any(|c| c.is_unhardened()) {
            return Err(CommonError::Cap26DictatesThatAllIndicesMustBeHardened);
        }
        if components.len() != 6 {
            return Err(CommonError::InvalidLength);
        }
        if components[0] != PURPOSE {
            return Err(CommonError::InvalidPurpose);
        }
        if components[1] != COIN_TYPE {
            return Err(CommonError::InvalidCoinType);
        }
        let network_id = NetworkID::try_from(components[2].index_in_local_key_space())?;
        let entity_kind = Cap26EntityKind::try_from(components[3].index_in_local_key_space())?;
        let key_kind = Cap26KeyKind::try_from(components[4].index_in_local_key_space())?;
        let hardened = Hardened::try_from(components[5])?;

        Ok(UnvalidatedCap26Path {
            network_id,
            entity_kind,
            key_kind,
            index: hardened,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = UnvalidatedCap26Path;

    #[test]
    fn from_str_invalid_purpose() {
        assert!(matches!(
            Sut::try_from(HDPath::from_str("m/44/1022H/1H/525H/1460H/0H").unwrap()),
            Err(CommonError::Cap26DictatesThatAllIndicesMustBeHardened)
        ));
        assert!(matches!(
            Sut::try_from(HDPath::from_str("m/43H/1022H/1H/525H/1460H/0H").unwrap()),
            Err(CommonError::InvalidPurpose)
        ));
    }

    #[test]
    fn from_str_invalid_cointype() {
        assert!(matches!(
            Sut::try_from(HDPath::from_str("m/44H/1022/1H/525H/1460H/0H").unwrap()),
            Err(CommonError::Cap26DictatesThatAllIndicesMustBeHardened)
        ));
        assert!(matches!(
            Sut::try_from(HDPath::from_str("m/44H/55555H/1H/525H/1460H/0H").unwrap()),
            Err(CommonError::InvalidCoinType)
        ));
    }

    #[test]
    fn from_str_invalid_network_id() {
        assert!(matches!(
            Sut::try_from(HDPath::from_str("m/44H/1022H/5555/525H/1460H/0H").unwrap()),
            Err(CommonError::Cap26DictatesThatAllIndicesMustBeHardened)
        ));

        assert!(matches!(
            Sut::try_from(HDPath::from_str("m/44H/1022H/5555H/525H/1460H/0H").unwrap()),
            Err(CommonError::InvalidNetworkID)
        ));
    }

    #[test]
    fn from_str_invalid_entity_kind() {
        assert!(matches!(
            Sut::try_from(HDPath::from_str("m/44H/1022H/1H/525/1460H/0H").unwrap()),
            Err(CommonError::Cap26DictatesThatAllIndicesMustBeHardened)
        ));
        assert!(matches!(
            Sut::try_from(HDPath::from_str("m/44H/1022H/1H/333H/1460H/0H").unwrap()),
            Err(CommonError::InvalidEntityKind)
        ));
    }

    #[test]
    fn from_str_invalid_key_kind() {
        assert!(matches!(
            Sut::try_from(HDPath::from_str("m/44H/1022H/1H/525H/1460/0H").unwrap()),
            Err(CommonError::Cap26DictatesThatAllIndicesMustBeHardened)
        ));

        assert!(matches!(
            Sut::try_from(HDPath::from_str("m/44H/1022H/1H/525H/22H/0H").unwrap()),
            Err(CommonError::InvalidKeyKind)
        ));
    }

    #[test]
    fn from_str_invalid_index_not_hardened() {
        assert!(matches!(
            Sut::try_from(HDPath::from_str("m/44H/1022H/1H/525/1460H/0").unwrap()),
            Err(CommonError::Cap26DictatesThatAllIndicesMustBeHardened)
        ));
    }
}
