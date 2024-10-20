use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deref, AsRef)]
pub struct U30(u32);

impl U30 {
    pub const MAX: u32 = U30_MAX;

    /// # Safety
    /// Unsafe, does not validate the value to be small enough.
    ///
    /// Only use this for tests and constants.
    pub(crate) const unsafe fn new(value: u32) -> Self {
        Self(value)
    }
}

impl AddViaDeref for U30 {}
impl AddSelfViaDeref for U30 {}

impl From<U30> for U31 {
    fn from(value: U30) -> Self {
        Self::try_from(value.0).unwrap()
    }
}
impl TryFrom<U31> for U30 {
    type Error = CommonError;
    fn try_from(value: U31) -> Result<Self, Self::Error> {
        let large: u32 = value.into();
        Self::try_from(large)
    }
}

impl TryFrom<u32> for U30 {
    type Error = CommonError;

    fn try_from(value: u32) -> Result<Self> {
        if value <= Self::MAX {
            Ok(Self(value))
        } else {
            Err(CommonError::Overflow)
        }
    }
}

impl HasSampleValues for U30 {
    fn sample() -> Self {
        Self::try_from(30u32).unwrap()
    }
    fn sample_other() -> Self {
        Self::try_from(Self::MAX).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = U30;

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
        let sut = Sut::try_from(42).unwrap();
        assert_eq!(sut.checked_add(&Sut::try_from(0u32).unwrap()).unwrap(), sut);
    }

    #[test]
    fn add_zero_to_max_is_ok() {
        let sut = Sut::try_from(Sut::MAX).unwrap();
        assert_eq!(sut.checked_add(&Sut::try_from(0u32).unwrap()).unwrap(), sut,);
    }

    #[test]
    fn add_max_to_zero_is_ok() {
        let sut = Sut::try_from(0).unwrap();
        assert_eq!(
            sut.checked_add_n(Sut::MAX).unwrap(),
            Sut::try_from(Sut::MAX).unwrap()
        );
    }

    #[test]
    fn add_one() {
        let sut = Sut::try_from(42).unwrap();
        assert_eq!(sut.checked_add_one().unwrap(), Sut::try_from(43).unwrap());
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
        let sut = Sut::try_from(1).unwrap();
        assert!(matches!(
            sut.checked_add(&Sut::try_from(Sut::MAX).unwrap()),
            Err(CommonError::Overflow)
        ));
    }
}
