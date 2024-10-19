use crate::{KeySpace, U30_MAX, U31};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct InLocalKeySpace {
    pub local_index: U31,
    pub key_space: KeySpace,
}
impl InLocalKeySpace {
    pub fn new(local_index: U31, key_space: KeySpace) -> Self {
        if key_space.is_securified() || key_space.is_unsecurified_hardened() {
            assert!(u32::from(local_index) <= U30_MAX);
        }

        Self {
            local_index,
            key_space,
        }
    }
}

pub trait IsMappableToLocalKeySpace {
    fn map_to_local_key_space(&self) -> InLocalKeySpace;
    fn index_in_local_key_space(&self) -> U31 {
        self.map_to_local_key_space().local_index
    }
}

#[cfg(test)]
mod tests {
    use crate::{GLOBAL_OFFSET_HARDENED, GLOBAL_OFFSET_SECURIFIED};

    use super::*;

    type Sut = InLocalKeySpace;

    #[test]
    fn ok_securified_less_than_offset() {
        let local = U31::try_from(U30_MAX).unwrap();
        let sut = Sut::new(local, KeySpace::Securified);
        assert!(sut.key_space.is_securified());
        assert_eq!(sut.local_index, local);
    }

    #[test]
    #[should_panic]
    fn panics_securified_less_than_offset() {
        Sut::new(
            U31::try_from(GLOBAL_OFFSET_SECURIFIED - 1).unwrap(),
            KeySpace::Securified,
        );
    }

    #[test]
    #[should_panic]
    fn panics_unsecurified_eq_securified_offset() {
        Sut::new(
            U31::try_from(GLOBAL_OFFSET_SECURIFIED).unwrap(),
            KeySpace::Unsecurified { is_hardened: true },
        );
    }

    #[test]
    #[should_panic]
    fn panics_unsecurified_greater_than_securified_offset() {
        Sut::new(
            U31::try_from(GLOBAL_OFFSET_SECURIFIED + 1).unwrap(),
            KeySpace::Unsecurified { is_hardened: true },
        );
    }

    #[test]
    #[should_panic]
    fn panics_unsecurified_hardened_less_than_offset() {
        Sut::new(
            U31::try_from(GLOBAL_OFFSET_HARDENED - 1).unwrap(),
            KeySpace::Unsecurified { is_hardened: true },
        );
    }

    #[test]
    #[should_panic]
    fn panics_unsecurified_unhardened_eq_offset() {
        Sut::new(
            U31::try_from(GLOBAL_OFFSET_HARDENED).unwrap(),
            KeySpace::Unsecurified { is_hardened: false },
        );
    }

    #[test]
    #[should_panic]
    fn panics_unsecurified_unhardened_greater_than_offset() {
        Sut::new(
            U31::try_from(GLOBAL_OFFSET_HARDENED + 1).unwrap(),
            KeySpace::Unsecurified { is_hardened: false },
        );
    }
}
