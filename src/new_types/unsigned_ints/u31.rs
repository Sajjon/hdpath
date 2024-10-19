use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deref, AsRef)]
pub struct U31(u32);

impl HasSampleValues for U31 {
    fn sample() -> Self {
        Self::try_from(237u32).unwrap()
    }
    fn sample_other() -> Self {
        Self::try_from(U31_MAX).unwrap()
    }
}

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
        if value <= U31_MAX {
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
        assert_eq!(*Sut::try_from(U31_MAX - 1).unwrap(), U31_MAX - 1);
        assert_eq!(*Sut::try_from(U31_MAX).unwrap(), U31_MAX);
    }

    #[test]
    fn try_from_overflow() {
        assert!(Sut::try_from(U31_MAX + 1).is_err());
    }
}
