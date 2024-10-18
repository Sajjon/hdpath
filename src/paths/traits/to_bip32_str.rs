use crate::prelude::*;

pub trait ToBIP32Str: Sized {
    fn to_bip32_string(&self) -> String;
    fn to_bip32_string_debug(&self) -> String;
}

impl<T> ToBIP32Str for T
where
    T: IsPathComponentStringConvertible + HasIndexInLocalKeySpace,
{
    fn to_bip32_string(&self) -> String {
        format!("{}{}", self.index_in_local_key_space(), T::CANONICAL_SUFFIX)
    }
    fn to_bip32_string_debug(&self) -> String {
        format!(
            "{}{}",
            self.index_in_local_key_space(),
            T::NON_CANONICAL_SUFFIXES
        )
    }
}
