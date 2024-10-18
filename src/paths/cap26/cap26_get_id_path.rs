use crate::prelude::*;

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
    fn from(_: GetIDPath) -> Self {
        Self::new(Vec::from_iter(GetIDPath::PATH))
    }
}
