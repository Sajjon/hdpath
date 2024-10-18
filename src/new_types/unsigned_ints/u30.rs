use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deref, AsRef)]
pub struct U31(u32);
impl U31 {
    pub(crate) const unsafe fn new(value: u32) -> Self {
        Self(value)
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
