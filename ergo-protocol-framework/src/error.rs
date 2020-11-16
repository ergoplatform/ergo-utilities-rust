use thiserror::Error;
pub type Result<T> = std::result::Result<T, ProtocolFrameworkError>;

#[derive(Error, Debug)]
pub enum ProtocolFrameworkError {
    #[error("The address of the box does not match the address in the `BoxSpec`.")]
    InvalidAddress,
    #[error(
        "The number of Ergs held within the box is outside of the valid range for the `BoxSpec`."
    )]
    InvalidErgsValue,
    #[error("One of the tokens failed to match the `BoxSpec`.")]
    FailedTokenSpec,
    #[error("One of the registers failed to match the `BoxSpec`.")]
    FailedRegisterSpec,
    #[error("The encoded predicate on the BoxSpec failed.")]
    FailedPredicate,
    #[error("{0}")]
    Other(String),
}
