use super::Cap26EntityKind;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, thiserror::Error)]
pub enum CommonError {
    #[error("IndexNotHardened {bad_index:?}")]
    IndexNotHardened { bad_index: u32 },

    #[error("overflow")]
    Overflow,

    #[error("Invalid Length")]
    InvalidLength,

    #[error("Invalid NetworkID")]
    InvalidNetworkID,

    #[error("Invalid KeyKind")]
    InvalidKeyKind,

    #[error("Invalid EntityKind")]
    InvalidEntityKind,

    #[error("Invalid Purpose")]
    InvalidPurpose,

    #[error("Invalid CoinType")]
    InvalidCoinType,

    #[error("Invalid Suffix")]
    InvalidSuffix,

    #[error("Cannot add to index since it would change key space")]
    CannotAddMoreToIndexSinceItWouldChangeKeySpace,

    #[error("NonHardenedIndex")]
    NonHardenedIndex,

    #[error("Non hardened component found")]
    Cap26DictatesThatAllIndicesMustBeHardened,

    #[error("NonU32Str")]
    NonU32Str,

    #[error("Index Securified expected Unsecurified")]
    IndexSecurifiedExpectedUnsecurified,

    #[error("Index Unsecurified expected Securified")]
    IndexUnsecurifiedExpectedSecurified,

    #[error("Index In Global Key Space Is Lower Than Offset")]
    IndexInGlobalKeySpaceIsLowerThanOffset,

    #[error("InvalidBip44ExpectedAccountComponentToBeHardened")]
    InvalidBip44ExpectedAccountComponentToBeHardened,

    #[error("InvalidBip44ExpectedChangeComponentToNotBeHardened")]
    InvalidBip44ExpectedChangeComponentToNotBeHardened,

    #[error("WrongEntityKind found: {found:?}, expected: {expected:?}")]
    WrongEntityKind {
        expected: Cap26EntityKind,
        found: Cap26EntityKind,
    },
}
pub type Result<T, E = CommonError> = std::result::Result<T, E>;
