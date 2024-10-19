use crate::prelude::*;

pub trait FromLocalKeySpace: Sized + From<Self::Magnitude> {
    type Magnitude: Into<U31> + TryFrom<u32, Error = CommonError>;

    fn from_local_key_space(
        value: impl TryInto<Self::Magnitude, Error = CommonError>,
    ) -> Result<Self> {
        value.try_into().map(Self::from)
    }
}
