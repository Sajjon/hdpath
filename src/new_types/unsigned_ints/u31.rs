use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deref, AsRef)]
pub struct U31(u32);

impl HasSampleValues for U31 {
    fn sample() -> Self {
        Self::try_from(237u32).unwrap()
    }
    fn sample_other() -> Self {
        Self::try_from(Self::MAX).unwrap()
    }
}

impl U31 {
    pub const MAX: u32 = U31_MAX;
}

impl AddViaDeref for U31 {}
impl AddSelfViaDeref for U31 {}

#[cfg(test)]
impl From<u16> for U31 {
    fn from(value: u16) -> Self {
        Self::try_from(value as u32).unwrap()
    }
}
impl From<U31> for u32 {
    fn from(value: U31) -> Self {
        value.0
    }
}
impl TryFrom<u32> for U31 {
    type Error = CommonError;

    fn try_from(value: u32) -> Result<Self> {
        if value <= Self::MAX {
            Ok(Self(value))
        } else {
            Err(CommonError::Overflow)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = U31;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample(),);
        assert_eq!(Sut::sample_other(), Sut::sample_other(),);
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other(),);
    }

    #[test]
    fn ord() {
        assert!(Sut::sample() < Sut::sample_other());
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<Sut>::from_iter([
                Sut::sample(),
                Sut::sample(),
                Sut::sample_other(),
                Sut::sample_other(),
            ])
            .len(),
            2
        )
    }

    #[test]
    fn try_from_valid() {
        assert_eq!(*Sut::try_from(0u32).unwrap(), 0);
        assert_eq!(*Sut::try_from(1u32).unwrap(), 1);
        assert_eq!(*Sut::try_from(Sut::MAX - 1).unwrap(), Sut::MAX - 1);
        assert_eq!(*Sut::try_from(Sut::MAX).unwrap(), Sut::MAX);
    }

    #[test]
    fn try_from_overflow() {
        assert!(Sut::try_from(Sut::MAX + 1).is_err());
    }

    #[test]
    fn add_zero() {
        let sut = Sut::try_from(42u32).unwrap();
        assert_eq!(sut.checked_add(&Sut::try_from(0u32).unwrap()).unwrap(), sut);
    }

    #[test]
    fn add_zero_to_max_is_ok() {
        let sut = Sut::try_from(Sut::MAX).unwrap();
        assert_eq!(sut.checked_add(&Sut::try_from(0u32).unwrap()).unwrap(), sut,);
    }

    #[test]
    fn add_max_to_zero_is_ok() {
        let sut = Sut::try_from(0u32).unwrap();
        assert_eq!(
            sut.checked_add_n(Sut::MAX).unwrap(),
            Sut::try_from(Sut::MAX).unwrap()
        );
    }

    #[test]
    fn add_one() {
        let sut = Sut::try_from(42u32).unwrap();
        assert_eq!(
            sut.checked_add_one().unwrap(),
            Sut::try_from(43u32).unwrap()
        );
    }

    #[test]
    fn add_one_to_max_minus_1_is_max() {
        let sut = Sut::try_from(Sut::MAX - 1).unwrap();
        assert_eq!(
            sut.checked_add(&Sut::try_from(1u32).unwrap()).unwrap(),
            Sut::try_from(Sut::MAX).unwrap()
        );
    }

    #[test]
    fn addition_overflow_base_max() {
        let sut = Sut::try_from(Sut::MAX).unwrap();
        assert!(matches!(
            sut.checked_add(&Sut::try_from(1u32).unwrap()),
            Err(CommonError::Overflow)
        ));
    }

    #[test]
    fn addition_overflow_add_max() {
        let sut = Sut::try_from(1u32).unwrap();
        assert!(matches!(
            sut.checked_add(&Sut::try_from(Sut::MAX).unwrap()),
            Err(CommonError::Overflow)
        ));
    }
}
