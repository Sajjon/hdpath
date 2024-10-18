mod u30;
mod u31;

pub use u30::*;
pub use u31::*;

pub const GLOBAL_OFFSET_HARDENED: u32 = 2u32.pow(31);

pub const U30_MAX: u32 = LOCAL_OFFSET_SECURIFIED - 1;
pub const U31_MAX: u32 = GLOBAL_OFFSET_HARDENED - 1;
pub const U32_MAX: u32 = u32::MAX;

/// Does NOT also offset by `GLOBAL_OFFSET_HARDENED`
pub const LOCAL_OFFSET_SECURIFIED: u32 = 2u32.pow(30);

pub const GLOBAL_OFFSET_SECURIFIED: u32 = GLOBAL_OFFSET_HARDENED + LOCAL_OFFSET_SECURIFIED;

#[cfg(test)]
mod tests {
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

    #[test]
    fn global_offset_hardened() {
        assert_eq!(GLOBAL_OFFSET_HARDENED - 1, U31_MAX);
    }

    #[test]
    fn local_offset_securified() {
        assert_eq!(LOCAL_OFFSET_SECURIFIED - 1, U30_MAX);
    }

    #[test]
    fn global_offset_securified() {
        assert_eq!(
            GLOBAL_OFFSET_SECURIFIED,
            GLOBAL_OFFSET_HARDENED + LOCAL_OFFSET_SECURIFIED
        );
    }
}
