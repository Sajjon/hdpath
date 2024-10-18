use crate::prelude::*;

pub trait FromLocalKeySpace: Sized {
    fn from_local_key_space(value: u32) -> Result<Self>;
}
