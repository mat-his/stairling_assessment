use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Balance error")]
    BalanceError,

    #[error("Internal error: {0}")]
    InternalError(#[from] diesel::result::Error),

    #[error("Driver not found")]
    NoDriverFound,
    #[error("Rides not found")]
    NoRidesFound,
}
