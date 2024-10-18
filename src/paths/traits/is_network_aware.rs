use crate::prelude::*;

pub trait IsNetworkAware {
    fn network_id(&self) -> NetworkID;
}
