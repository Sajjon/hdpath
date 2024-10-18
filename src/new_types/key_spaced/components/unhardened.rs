use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, Debug, Mul, AsRef)]
#[deref(forward)]
pub struct Unhardened(U31);
impl Unhardened {
    pub const fn new(value: U31) -> Self {
        Self(value)
    }
}
impl HasIndexInLocalKeySpace for Unhardened {}

impl HasOffsetFromGlobalKeySpace for Unhardened {
    fn offset_from_global_key_space() -> u32 {
        0
    }
}
