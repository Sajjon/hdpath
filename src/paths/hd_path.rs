use itertools::Itertools;

use crate::prelude::*;

#[allow(unused)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Display,
    MoreDebug,
    DeserializeFromStr,
    SerializeDisplay,
)]
#[display("{}", self.to_bip32_string())]
#[debug("{}", self.to_bip32_string_debug())]
pub struct HDPath(Vec<HDPathComponent>);
impl HDPath {
    pub const fn new(components: Vec<HDPathComponent>) -> Self {
        Self(components)
    }
}

impl FromBIP32Str for HDPath {
    fn from_bip32_string(s: &str) -> Result<Self> {
        let mut s = s;
        if s.starts_with("m/") {
            s = &s[2..]
        }
        if s.starts_with("M/") {
            s = &s[2..]
        }
        if s.starts_with("/") {
            s = &s[1..]
        }
        let components = s
            .split(Self::SEPARATOR)
            .filter(|s| !s.is_empty())
            .map(HDPathComponent::from_str)
            .collect::<Result<Vec<_>>>()?;
        Ok(Self(components))
    }
}

impl HDPath {
    pub const SEPARATOR: &str = "/";
    fn to_string_map<F>(&self, map: F) -> String
    where
        F: Fn(&HDPathComponent) -> String,
    {
        let head = "m".to_owned();
        let mut path = vec![head];
        let tail = self.0.iter().map(map);
        path.extend(tail.collect_vec());
        path.into_iter().join(Self::SEPARATOR)
    }
}
impl ToBIP32Str for HDPath {
    fn to_bip32_string(&self) -> String {
        self.to_string_map(|c| format!("{}", c))
    }
    fn to_bip32_string_debug(&self) -> String {
        self.to_string_map(|c| format!("{:?}", c))
    }
}

impl FromStr for HDPath {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

// impl<T: TryFromHDPath> FromBIP32Str for T {
//     fn from_bip32_string(s: &str) -> Result<Self> {
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {
    // use super::*;

    // type Sut = HDPath;

    #[test]
    fn display() {
        // assert_eq!(Sut::default().to_string(), "");
    }
}
