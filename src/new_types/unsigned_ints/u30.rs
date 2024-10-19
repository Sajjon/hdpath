use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deref, AsRef)]
pub struct U30(u32);

impl U30 {
    pub(crate) const unsafe fn new(value: u32) -> Self {
        Self(value)
    }
}

#[cfg(test)]
impl From<u16> for U31 {
    fn from(value: u16) -> Self {
        Self::try_from(value as u32).unwrap()
    }
}

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

impl HasSampleValues for U30 {
    fn sample() -> Self {
        Self::try_from(30).unwrap()
    }
    fn sample_other() -> Self {
        Self::try_from(U30_MAX).unwrap()
    }
}

impl TryFrom<u32> for U30 {
    type Error = CommonError;

    fn try_from(value: u32) -> Result<Self> {
        if value <= U30_MAX {
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
