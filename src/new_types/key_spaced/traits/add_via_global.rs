use crate::prelude::*;

pub trait AddViaGlobalKeySpace:
    IsMappableToGlobalKeySpace + FromGlobalKeySpace + IsMappableToLocalKeySpace
{
    fn checked_add_one_to_global(&self) -> Result<Self> {
        self.checked_add_n_to_global(1)
    }

    fn checked_add_n_to_global(&self, n: u32) -> Result<Self> {
        let key_space_before = self.map_to_local_key_space().key_space();
        let global = self
            .map_to_global_key_space()
            .checked_add(n)
            .ok_or(CommonError::Overflow)?;
        let sum = Self::from_global_key_space(global)?;
        let key_space_after = sum.map_to_local_key_space().key_space();
        if key_space_after != key_space_before {
            return Err(CommonError::CannotAddMoreToIndexSinceItWouldChangeKeySpace);
        }
        Ok(sum)
    }
}
