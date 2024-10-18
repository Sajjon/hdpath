#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, thiserror::Error)]
pub enum CommonError {
    #[error("overflow")]
    Overflow,

    #[error("NonHardenedIndex")]
    NonHardenedIndex,

    #[error("Index Securified expected Unsecurified")]
    IndexSecurifiedExpectedUnsecurified,

    #[error("Index Unsecurified expected Securified")]
    IndexUnsecurifiedExpectedSecurified,

    #[error("Index In Global Key Space Is Lower Than Offset")]
    IndexInGlobalKeySpaceIsLowerThanOffset,
}
pub type Result<T, E = CommonError> = std::result::Result<T, E>;
