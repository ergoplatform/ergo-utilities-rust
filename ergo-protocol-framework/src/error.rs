use thiserror::Error;
pub type Result<T> = std::result::Result<T, ProtocolFrameworkError>;

#[derive(Error, Debug)]
pub enum ProtocolFrameworkError {
    #[error("The address of the `BoxSpec` is invalid.")]
    InvalidSpecAddress,
    #[error(
        "The number of Ergs held within the box is outside of the valid range for the `BoxSpec`."
    )]
    InvalidSpecErgsValue,
    #[error("One of the tokens failed to match the `BoxSpec`.")]
    FailedTokenSpec,
    #[error("One of the registers failed to match the `BoxSpec`.")]
    FailedRegisterSpec,
    #[error("The encoded predicate on the BoxSpec failed.")]
    FailedSpecPredicate,
    #[error("The address provided is invalid: {0}")]
    InvalidAddress(String),
    #[error("{0}")]
    Other(String),
}
