use crate::prelude::*;

#[allow(unused)]
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
