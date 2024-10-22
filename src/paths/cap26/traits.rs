use crate::prelude::*;

pub trait HasEntityKind {
    fn entity_kind() -> Cap26EntityKind;
}

pub trait HasEntityKindObjectSafe {
    fn get_entity_kind(&self) -> Cap26EntityKind;
}

impl<T: HasEntityKind> HasEntityKindObjectSafe for T {
    fn get_entity_kind(&self) -> Cap26EntityKind {
        T::entity_kind()
    }
}

pub trait NewEntityPath: Sized {
    fn new(
        network_id: impl Into<NetworkID>,
        key_kind: impl Into<Cap26KeyKind>,
        index: impl Into<Hardened>,
    ) -> Self;
}

pub trait NewEntityPathCheckingEntityKind: NewEntityPath {
    fn try_from_unvalidated(path: UnvalidatedCap26Path) -> Result<Self>;
}

impl<T: HasEntityKind + NewEntityPath> NewEntityPathCheckingEntityKind for T {
    fn try_from_unvalidated(path: UnvalidatedCap26Path) -> Result<Self> {
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
