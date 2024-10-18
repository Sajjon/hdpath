use crate::prelude::*;

pub trait ToBIP32Str: Sized {
    fn to_bip32_string(&self) -> String;
}

impl<T: IsPathComponentStringConvertible + HasIndexInLocalKeySpace> ToBIP32Str for T {
    fn to_bip32_string(&self) -> String {
        format!("{}{}", self.index_in_local_key_space(), T::CANONICAL_SUFFIX)
    }
}
