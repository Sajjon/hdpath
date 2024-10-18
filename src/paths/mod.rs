mod any_account_path;
mod bip44_like_path;
mod cap26;
mod derivation_path;
mod hd_path;

mod traits;

pub use any_account_path::*;
pub use bip44_like_path::*;
pub use cap26::*;
pub use derivation_path::*;
pub use hd_path::*;

pub use traits::*;

use crate::prelude::*;

pub(super) const unsafe fn hard(value: u32) -> HDPathComponent {
    unsafe {
        HDPathComponent::Unsecurified(Unsecurified::Hardened(UnsecurifiedHardened::new(U30::new(
            value,
        ))))
    }
}

pub(super) const PURPOSE: HDPathComponent = unsafe { hard(44) };
pub(super) const COIN_TYPE: HDPathComponent = unsafe { hard(1022) };

pub(super) fn cap26(
    network_id: NetworkID,
    entity_kind: CAP26EntityKind,
    key_kind: CAP26KeyKind,
    index: Hardened,
) -> HDPath {
    let mut path: [HDPathComponent; 6] = [PURPOSE; 6];
    path[1] = COIN_TYPE;
    path[2] = HDPathComponent::from(network_id);
    path[3] = HDPathComponent::from(entity_kind);
    path[4] = HDPathComponent::from(key_kind);
    path[5] = HDPathComponent::from(index);
    HDPath::new(Vec::from_iter(path))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn purpose_to_string() {
        assert_eq!(PURPOSE.to_string(), "44H");
    }
}
