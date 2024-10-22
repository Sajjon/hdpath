use crate::prelude::*;

pub trait FromLocalKeySpace: Sized + From<Self::Magnitude> {
    type Magnitude: Into<U31> + TryFrom<u32, Error = CommonError>;

    fn from_local_key_space(value: u32) -> Result<Self> {
        let magnitude = Self::Magnitude::try_from(value)?;
        Ok(Self::from(magnitude))
    }
}
