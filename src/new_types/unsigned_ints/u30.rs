use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deref, AsRef)]
pub struct U31(u32);
impl U31 {
    pub(crate) const unsafe fn new(value: u32) -> Self {
        Self(value)
    }
}

impl HasSampleValues for U31 {
    fn sample() -> Self {
        Self::try_from(31).unwrap()
    }
    fn sample_other() -> Self {
        Self::try_from(U31_MAX).unwrap()
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
        assert_eq!(*Sut::try_from(0).unwrap(), 0);
        assert_eq!(*Sut::try_from(1).unwrap(), 1);
        assert_eq!(*Sut::try_from(U30_MAX - 1).unwrap(), U30_MAX - 1);
        assert_eq!(*Sut::try_from(U30_MAX).unwrap(), U30_MAX);
    }

    #[test]
    fn try_from_overflow() {
        assert!(Sut::try_from(U30_MAX + 1).is_err());
    }
}
