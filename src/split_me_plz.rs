#![allow(unused)]

use crate::prelude::*;

pub trait FromLocalKeySpace: Sized {
    fn from_local_key_space(value: u32) -> Result<Self>;
}

pub trait HasOffsetFromGlobalKeySpace {
    fn offset_from_global_key_space() -> u32;
}
pub trait FromGlobalKeySpace: HasOffsetFromGlobalKeySpace + Sized {
    fn from_global_key_space(value: u32) -> Result<Self>;
}

impl<T: FromLocalKeySpace + HasOffsetFromGlobalKeySpace> FromGlobalKeySpace for T {
    fn from_global_key_space(value: u32) -> Result<Self> {
        value
            .checked_sub(T::offset_from_global_key_space())
            .ok_or(CommonError::IndexInGlobalKeySpaceIsLowerThanOffset)
            .and_then(Self::from_local_key_space)
    }
}

pub trait IsMappableToGlobalKeySpace {
    fn into_global_key_space(self) -> u32;
}

pub trait HasIndexInLocalKeySpace: Deref<Target = u32> {
    fn index_in_local_key_space(&self) -> u32 {
        **self
    }
}

impl<T: HasIndexInLocalKeySpace + HasOffsetFromGlobalKeySpace> IsMappableToGlobalKeySpace for T {
    fn into_global_key_space(self) -> u32 {
        self.index_in_local_key_space() + T::offset_from_global_key_space()
    }
}

impl FromStr for Unsecurified {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        todo!()
    }
}

impl FromStr for SecurifiedU30 {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        todo!()
    }
}
pub trait ToBip32String: std::fmt::Display {
    fn bip32_string(&self) -> String {
        todo!()
    }
}

// pub trait TryFromHDPath: Sized {
//     fn try_from_hd_path(path: HDPath) -> Result<Self>;
// }

// pub trait FromBip32String: TryFromHDPath + FromStr {
//     fn from_bip32_string(s: impl AsRef<str>) -> Result<Self>;
// }
trait IsHDPath: TryFrom<HDPath> + Into<HDPath> + FromStr + std::fmt::Display {}
impl<T: TryFrom<HDPath> + Into<HDPath> + FromStr + std::fmt::Display> IsHDPath for T {}

// impl ToBip32String for GetIDPath {
//     fn bip32_string(&self) -> String {
//         todo!()
//     }
// }
// impl FromBip32String for GetIDPath {
//     fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
//         todo!()
//     }
// }
// impl_from_str_for__from_bip32_str!(GetIDPath);

pub trait HasEntityKind {
    fn entity_kind() -> CAP26EntityKind;
}
pub trait HasEntityKindObjectSafe {
    fn get_entity_kind(&self) -> CAP26EntityKind;
}
impl<T: HasEntityKind> HasEntityKindObjectSafe for T {
    fn get_entity_kind(&self) -> CAP26EntityKind {
        T::entity_kind()
    }
}
