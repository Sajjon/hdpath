use crate::prelude::*;

#[derive(Clone, Default, MoreDebug, derive_more::Display)]
#[display("{}", self.to_bip32_string())]
#[debug("{}", self.to_bip32_string_debug())]
pub struct Cap26GetIDPath;

impl Cap26GetIDPath {
    pub const PATH: [HDPathComponent; 3] = [PURPOSE, COIN_TYPE, GET_ID_LAST];
}

impl From<Cap26GetIDPath> for HDPath {
    fn from(_: Cap26GetIDPath) -> Self {
        Self::new(Vec::from_iter(Cap26GetIDPath::PATH))
    }
}
impl Cap26GetIDPath {
    pub fn to_hd_path(&self) -> HDPath {
        HDPath::from(self.clone())
    }
}

impl ToBIP32Str for Cap26GetIDPath {
    fn to_bip32_string(&self) -> String {
        self.to_hd_path().to_bip32_string()
    }
    fn to_bip32_string_debug(&self) -> String {
        self.to_hd_path().to_bip32_string_debug()
    }
}

use blake2::digest::{consts::U32, Digest};
use blake2::Blake2b;

pub type Blake2b256 = Blake2b<U32>;

pub fn blake2b_256_hash<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
    Blake2b256::digest(data).into()
}

#[cfg(test)]
mod tests {

    use super::*;

    type Sut = Cap26GetIDPath;

    #[test]
    fn display() {
        assert_eq!(format!("{}", Sut::default()), "m/44H/1022H/365H");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", Sut::default()), "m/44'/1022'/365'");
    }

    #[test]
    fn test_blake2b() {
        assert_eq!(
            "48f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a935",
            hex::encode(blake2b_256_hash("Hello Radix".as_bytes()))
        );
    }
}
