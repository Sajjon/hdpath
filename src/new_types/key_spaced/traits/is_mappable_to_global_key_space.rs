use crate::prelude::*;

pub trait IsMappableToGlobalKeySpace {
    fn into_global_key_space(self) -> u32;
}

impl<T: HasIndexInLocalKeySpace + HasOffsetFromGlobalKeySpace> IsMappableToGlobalKeySpace for T {
    fn into_global_key_space(self) -> u32 {
        self.index_in_local_key_space() + T::offset_from_global_key_space()
    }
}
