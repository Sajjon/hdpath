use crate::prelude::*;

pub trait HasIndexInLocalKeySpace: Deref<Target = u32> {
    fn index_in_local_key_space(&self) -> u32 {
        **self
    }
}
