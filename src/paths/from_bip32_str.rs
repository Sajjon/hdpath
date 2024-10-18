use crate::prelude::*;

pub trait FromBIP32Str: Sized {
    fn from_bip32_string(s: &str) -> Result<Self>;
}

impl<T: IsPathComponentStringConvertible + FromLocalKeySpace> FromBIP32Str for T {
    fn from_bip32_string(s: &str) -> Result<T> {
        if s.len() < 2 {
            return Err(CommonError::InvalidLength);
        }
        let suffix = &s[s.len() - 1..];
        if !T::ACCEPTABLE_SUFFIXES.contains(&suffix) {
            return Err(CommonError::InvalidSuffix);
        }
        let value: u32 = s[..s.len() - 1]
            .parse()
            .map_err(|_| CommonError::NonU32Str)?;

        T::from_local_key_space(value)
    }
}
