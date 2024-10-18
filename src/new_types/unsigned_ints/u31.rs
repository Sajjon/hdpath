use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deref, AsRef)]
pub struct U30(u32);

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
    fn try_from_valid() {
        assert_eq!(*Sut::try_from(0).unwrap(), 0);
        assert_eq!(*Sut::try_from(1).unwrap(), 1);
        assert_eq!(*Sut::try_from(U31_MAX - 1).unwrap(), U31_MAX - 1);
        assert_eq!(*Sut::try_from(U31_MAX).unwrap(), U31_MAX);
    }

    #[test]
    fn try_from_overflow() {
        assert!(Sut::try_from(U31_MAX + 1).is_err());
    }
}
