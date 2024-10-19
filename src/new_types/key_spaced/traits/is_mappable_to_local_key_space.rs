use crate::{KeySpace, U30, U31};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum UnsecurifiedKeySpaceWithLocalIndex {
    Unhardened(U31),
    Hardened(U30),
}
impl UnsecurifiedKeySpaceWithLocalIndex {
    pub fn index(&self) -> U31 {
        match self {
            Self::Unhardened(index) => *index,
            Self::Hardened(index) => U31::from(*index),
        }
    }
    pub fn key_space(&self) -> KeySpace {
        match self {
            Self::Unhardened(_) => KeySpace::Unsecurified { is_hardened: false },
            Self::Hardened(_) => KeySpace::Unsecurified { is_hardened: true },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum KeySpaceWithLocalIndex {
    Unsecurified(UnsecurifiedKeySpaceWithLocalIndex),
    Securified(U30),
}
impl KeySpaceWithLocalIndex {
    pub fn key_space(&self) -> KeySpace {
        match self {
            Self::Unsecurified(unsecurified) => unsecurified.key_space(),
            Self::Securified(_) => KeySpace::Securified,
        }
    }
    pub fn index(&self) -> U31 {
        match self {
            Self::Unsecurified(unsecurified) => unsecurified.index(),
            Self::Securified(index) => U31::from(*index),
        }
    }
}

pub trait IsMappableToLocalKeySpace {
    fn map_to_local_key_space(&self) -> KeySpaceWithLocalIndex;
    fn key_space(&self) -> KeySpace {
        let local = self.map_to_local_key_space();
        local.key_space()
    }
    fn index_in_local_key_space(&self) -> U31 {
        let local = self.map_to_local_key_space();
        local.index()
    }
}

#[cfg(test)]
mod tests {

    use crate::U30_MAX;

    use super::*;

    type Sut = KeySpaceWithLocalIndex;

    #[test]
    fn ok_securified_less_than_offset() {
        let index = U30::try_from(U30_MAX).unwrap();
        let sut = Sut::Securified(index);
        assert!(sut.key_space().is_securified());
        assert_eq!(sut.index(), U31::from(index));
    }
}
