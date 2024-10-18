#![allow(unused)]

use crate::prelude::*;

use derive_more::derive::{Add, AsRef, Deref, DerefMut, Mul};
use enum_as_inner::EnumAsInner;
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{ops::Deref, str::FromStr, vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum NetworkID {
    Mainnet,
    Stokenet,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum CAP26KeyKind {
    TransactionSigning,
    AuthenticationSigning,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum CAP26EntityKind {
    Account,
    Identity,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum KeySpace {
    Unsecurified,
    Securified,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deref, AsRef)]
pub struct U31(u32);

impl TryFrom<u32> for U31 {
    type Error = CommonError;

    fn try_from(value: u32) -> Result<Self> {
        if value <= U31_MAX {
            Ok(Self(value))
        } else {
            Err(CommonError::Overflow)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deref, AsRef)]
pub struct U30(u32);

impl TryFrom<u32> for U30 {
    type Error = CommonError;

    fn try_from(value: u32) -> Result<Self> {
        if value <= U30_MAX {
            Ok(Self(value))
        } else {
            Err(CommonError::Overflow)
        }
    }
}

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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, Debug, Mul, AsRef)]
#[deref(forward)]
pub struct Unhardened(U31);
impl HasIndexInLocalKeySpace for Unhardened {}

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

impl HasOffsetFromGlobalKeySpace for Unhardened {
    fn offset_from_global_key_space() -> u32 {
        0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, Debug, Mul, AsRef)]
#[deref(forward)]
pub struct UnsecurifiedHardened(U30);
impl UnsecurifiedHardened {
    pub const fn new(value: U30) -> Self {
        Self(value)
    }

    pub fn new_from_global_key_space(value: u32) -> Result<Self> {
        U30::try_from(value).map(Self::new)
    }
}
impl FromLocalKeySpace for UnsecurifiedHardened {
    /// 0' => Ok(0)
    /// 1' => Ok(1)
    /// 2^31 + 5 (5') => Err
    fn from_local_key_space(value: u32) -> Result<Self> {
        U30::try_from(value).map(Self::new)
    }
}

pub const U30_MAX: u32 = 2u32.pow(30) - 1;
pub const U31_MAX: u32 = 2u32.pow(31) - 1;
pub const U32_MAX: u32 = u32::MAX;

pub const GLOBAL_OFFSET_HARDENED: u32 = U31_MAX;

/// Does NOT also offset by `GLOBAL_OFFSET_HARDENED`
pub const LOCAL_OFFSET_SECURIFIED: u32 = U30_MAX;

pub const GLOBAL_OFFSET_SECURIFIED: u32 = GLOBAL_OFFSET_HARDENED + LOCAL_OFFSET_SECURIFIED;

impl HasIndexInLocalKeySpace for UnsecurifiedHardened {}
impl HasOffsetFromGlobalKeySpace for UnsecurifiedHardened {
    fn offset_from_global_key_space() -> u32 {
        LOCAL_OFFSET_SECURIFIED
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Unsecurified {
    Unhardened(Unhardened),
    Hardened(UnsecurifiedHardened),
}

impl IsMappableToGlobalKeySpace for Unsecurified {
    fn into_global_key_space(self) -> u32 {
        match self {
            Self::Unhardened(u) => u.into_global_key_space(),
            Self::Hardened(h) => h.into_global_key_space(),
        }
    }
}

impl From<UnsecurifiedHardened> for Unsecurified {
    fn from(value: UnsecurifiedHardened) -> Self {
        Unsecurified::Hardened(value)
    }
}
impl TryFrom<Unsecurified> for UnsecurifiedHardened {
    type Error = CommonError;

    fn try_from(value: Unsecurified) -> Result<Self> {
        match value {
            Unsecurified::Unhardened(u) => Err(CommonError::NonHardenedIndex),
            Unsecurified::Hardened(u) => Ok(u),
        }
    }
}

impl From<Unsecurified> for HDPathComponent {
    fn from(value: Unsecurified) -> Self {
        Self::Unsecurified(value)
    }
}
impl TryFrom<HDPathComponent> for Unsecurified {
    type Error = CommonError;

    fn try_from(value: HDPathComponent) -> Result<Self> {
        value
            .into_unsecurified()
            .map_err(|_| CommonError::IndexSecurifiedExpectedUnsecurified)
    }
}

impl From<SecurifiedU30> for HDPathComponent {
    fn from(value: SecurifiedU30) -> Self {
        Self::Securified(value)
    }
}
impl TryFrom<HDPathComponent> for SecurifiedU30 {
    type Error = CommonError;

    fn try_from(value: HDPathComponent) -> Result<Self> {
        value
            .into_securified()
            .map_err(|_| CommonError::IndexUnsecurifiedExpectedSecurified)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deref, Mul, AsRef)]
#[deref(forward)]
pub struct SecurifiedU30(U30);

impl HasIndexInLocalKeySpace for SecurifiedU30 {}
impl HasOffsetFromGlobalKeySpace for SecurifiedU30 {
    fn offset_from_global_key_space() -> u32 {
        GLOBAL_OFFSET_SECURIFIED
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, EnumAsInner)]
pub enum Hardened {
    Unsecurified(UnsecurifiedHardened),
    Securified(SecurifiedU30),
}
impl IsMappableToGlobalKeySpace for Hardened {
    fn into_global_key_space(self) -> u32 {
        match self {
            Hardened::Unsecurified(u) => u.into_global_key_space(),
            Hardened::Securified(s) => s.into_global_key_space(),
        }
    }
}

impl From<Hardened> for HDPathComponent {
    fn from(value: Hardened) -> Self {
        match value {
            Hardened::Unsecurified(u) => HDPathComponent::Unsecurified(Unsecurified::from(u)),
            Hardened::Securified(s) => HDPathComponent::Securified(s),
        }
    }
}
impl TryFrom<HDPathComponent> for Hardened {
    type Error = CommonError;

    fn try_from(value: HDPathComponent) -> Result<Self> {
        match value {
            HDPathComponent::Unsecurified(u) => {
                UnsecurifiedHardened::try_from(u).map(Self::Unsecurified)
            }
            HDPathComponent::Securified(s) => Ok(Hardened::Securified(s)),
        }
    }
}

/// ```ignore
/// [ <<------- UNHARDENED ------->> | <<-------- HARDENED --------->> ]
/// [ <<------------ UNSECURIFIED ---|-------->>  | <<- SECURIFIED ->> ]
/// ^                                ^            ^                    ^
/// 0                              2^31       2^31+2^30          2^32+1
///
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, EnumAsInner)]
pub enum HDPathComponent {
    Unsecurified(Unsecurified),
    Securified(SecurifiedU30),
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

impl HDPathComponent {
    fn securified(value: impl Into<SecurifiedU30>) -> Self {
        Self::Securified(value.into())
    }
    fn unsecurified(value: impl Into<Unsecurified>) -> Self {
        Self::Unsecurified(value.into())
    }
    fn unsecurified_harden(value: impl Into<UnsecurifiedHardened>) -> Self {
        Self::unsecurified(value.into())
    }
}
impl FromStr for HDPathComponent {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        SecurifiedU30::from_str(s)
            .map(Self::securified)
            .or(Unsecurified::from_str(s).map(Self::unsecurified))
    }
}
impl IsMappableToGlobalKeySpace for HDPathComponent {
    fn into_global_key_space(self) -> u32 {
        match self {
            HDPathComponent::Unsecurified(u) => u.into_global_key_space(),
            HDPathComponent::Securified(s) => s.into_global_key_space(),
        }
    }
}

pub trait ToBip32String: std::fmt::Display {
    fn bip32_string(&self) -> String {
        todo!()
    }
}

pub struct HDPath(Vec<HDPathComponent>);
impl HDPath {
    pub const fn new(components: Vec<HDPathComponent>) -> Self {
        Self(components)
    }
}

impl FromStr for HDPath {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        let components = s
            .split('/')
            .map(HDPathComponent::from_str)
            .collect::<Result<Vec<_>>>()?;
        Ok(Self(components))
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

#[derive(
    Clone,
    Debug,
    PartialEq,
    EnumAsInner,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    // SerializeDisplay,
    // DeserializeFromStr,
    // derive_more::Display,
)]
pub enum CAP26Path {
    // #[display("{value}")]
    GetID { value: GetIDPath },
    // #[display("{value}")]
    Account { value: CAP26AccountPath },
    // #[display("{value}")]
    Identity { value: CAP26IdentityPath },
}

// impl FromStr for CAP26Path {
//     type Err = CommonError;
//     fn from_str(s: &str) -> Result<Self> {
//         Self::from_bip32_string(s)
//     }
// }
// impl FromBip32String for CAP26Path {
//     fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
//         todo!()
//     }
// }

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    // SerializeDisplay,
    // DeserializeFromStr,
    // derive_more::Display,
)]
// #[display("{}", self.bip32_string())]
pub struct GetIDPath;
pub const GET_ID_LAST: HDPathComponent = unsafe { hard(365) };
impl GetIDPath {
    pub const PATH: [HDPathComponent; 3] = [M, COIN_TYPE, GET_ID_LAST];
}

impl From<GetIDPath> for HDPath {
    fn from(value: GetIDPath) -> Self {
        Self::new(Vec::from_iter(GetIDPath::PATH))
    }
}

/// Unchecked!
const unsafe fn hard(value: u32) -> HDPathComponent {
    HDPathComponent::Unsecurified(Unsecurified::Unhardened(Unhardened(U31(value))))
}

const M: HDPathComponent = unsafe { hard(44) };
const COIN_TYPE: HDPathComponent = unsafe { hard(1022) };

const fn cap26(tail: [HDPathComponent; 3]) -> [HDPathComponent; 5] {
    let mut path: [HDPathComponent; 5] = [M, M, M, M, M];
    path[1] = COIN_TYPE;
    path[2] = tail[0];
    path[3] = tail[1];
    path[4] = tail[2];
    path
}

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

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    // SerializeDisplay,
    // DeserializeFromStr,
    // derive_more::Display,
)]
// #[display("{}", self.bip32_string())]
pub struct CAP26AccountPath {
    network_id: NetworkID,
    key_kind: CAP26KeyKind,
    index: Hardened,
}

// impl ToBip32String for CAP26AccountPath {
//     fn bip32_string(&self) -> String {
//         todo!()
//     }
// }
// impl FromBip32String for CAP26AccountPath {
//     fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
//         todo!()
//     }
// }
// impl_from_str_for__from_bip32_str!(CAP26AccountPath);

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    // SerializeDisplay,
    // DeserializeFromStr,
    // derive_more::Display,
)]
// #[display("{}", self.bip32_string())]
pub struct CAP26IdentityPath {
    network_id: NetworkID,
    key_kind: CAP26KeyKind,
    index: Hardened,
}

// impl ToBip32String for CAP26IdentityPath {
//     fn bip32_string(&self) -> String {
//         todo!()
//     }
// }
// impl FromBip32String for CAP26IdentityPath {
//     fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
//         todo!()
//     }
// }
// impl_from_str_for__from_bip32_str!(CAP26IdentityPath);

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    SerializeDisplay,
    // DeserializeFromStr,
    derive_more::Display,
)]
// #[display("{}", self.bip32_string())]
pub struct BIP44LikePath {
    index: u32,
}

// impl ToBip32String for BIP44LikePath {
//     fn bip32_string(&self) -> String {
//         todo!()
//     }
// }
// impl FromBip32String for BIP44LikePath {
//     fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
//         todo!()
//     }
// }
// impl_from_str_for__from_bip32_str!(BIP44LikePath);

/// A derivation path on either supported schemes, either Babylon (CAP26) or Olympia (BIP44Like).
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    EnumAsInner,
    PartialOrd,
    Ord,
    // derive_more::Display,
    derive_more::Debug,
)]
pub enum DerivationPath {
    // #[debug("{}", self.bip32_string())]
    CAP26 { value: CAP26Path },
    // #[debug("{}", self.bip32_string())]
    BIP44Like { value: BIP44LikePath },
}

// impl ToBip32String for DerivationPath {
//     fn bip32_string(&self) -> String {
//         todo!()
//     }
// }
// impl FromBip32String for DerivationPath {
//     fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
//         todo!()
//     }
// }
// impl_from_str_for__from_bip32_str!(DerivationPath);

#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    EnumAsInner,
    PartialOrd,
    Ord,
    // derive_more::Display,
    derive_more::Debug,
)]
pub enum AnyAccountPath {
    // #[debug("{}", self.bip32_string())]
    CAP26 { value: CAP26AccountPath },
    // #[debug("{}", self.bip32_string())]
    BIP44Like { value: BIP44LikePath },
}

// impl ToBip32String for AnyAccountPath {
//     fn bip32_string(&self) -> String {
//         todo!()
//     }
// }
// impl FromBip32String for AnyAccountPath {
//     fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
//         todo!()
//     }
// }
// impl_from_str_for__from_bip32_str!(AnyAccountPath);

#[cfg(test)]
mod tests {
    use super::*;

    mod u32_max_values {
        use super::*;

        #[test]
        fn u32_max() {
            assert_eq!(U32_MAX, u32::MAX);
        }

        #[test]
        fn u31_max() {
            assert_eq!(U31_MAX, u32::MAX / 2);
        }

        #[test]
        fn u30_max() {
            assert_eq!(U30_MAX, U31_MAX / 2);
            assert_eq!(U30_MAX, u32::MAX / 4);
        }
    }

    mod u30 {
        use super::*;

        type Sut = U30;

        #[test]
        fn try_from_valid() {
            assert_eq!(*Sut::try_from(0).unwrap(), 0);
            assert_eq!(*Sut::try_from(1).unwrap(), 1);
            assert_eq!(*Sut::try_from(U30_MAX - 1).unwrap(), U30_MAX - 1);
            assert_eq!(*Sut::try_from(U30_MAX).unwrap(), U30_MAX);
        }

        #[test]
        fn try_from_overflow() {
            assert!(Sut::try_from(U30_MAX + 1).is_err());
        }
    }

    mod u31 {
        use super::*;

        type Sut = U31;

        #[test]
        fn try_from_valid() {
            assert_eq!(*Sut::try_from(0).unwrap(), 0);
            assert_eq!(*Sut::try_from(1).unwrap(), 1);
            assert_eq!(*Sut::try_from(U31_MAX - 1).unwrap(), U31_MAX - 1);
            assert_eq!(*Sut::try_from(U31_MAX).unwrap(), U31_MAX);
        }

        #[test]
        fn try_from_overflow() {
            assert!(Sut::try_from(U31_MAX + 1).is_err());
        }
    }
}
