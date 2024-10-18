mod any_account_path;
mod bip44_like_path;
mod cap26;
mod derivation_path;
mod from_bip32_str;
mod hd_path;
mod is_path_component_string_convertible;
mod to_bip32_str;

pub use any_account_path::*;
pub use bip44_like_path::*;
pub use cap26::*;
pub use derivation_path::*;
pub use from_bip32_str::*;
pub use hd_path::*;
pub use is_path_component_string_convertible::*;
pub use to_bip32_str::*;

use crate::prelude::*;

/// Unchecked!
pub(super) const unsafe fn hard(value: u32) -> HDPathComponent {
    unsafe {
        HDPathComponent::Unsecurified(Unsecurified::Unhardened(Unhardened::new(U31::new(value))))
    }
}

pub(super) const M: HDPathComponent = unsafe { hard(44) };
pub(super) const COIN_TYPE: HDPathComponent = unsafe { hard(1022) };

#[allow(unused)]
pub(super) const fn cap26(tail: [HDPathComponent; 3]) -> [HDPathComponent; 5] {
    let mut path: [HDPathComponent; 5] = [M, M, M, M, M];
    path[1] = COIN_TYPE;
    path[2] = tail[0];
    path[3] = tail[1];
    path[4] = tail[2];
    path
}