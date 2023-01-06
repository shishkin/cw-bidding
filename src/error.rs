use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Bid must be larger than {min}")]
    InsufficientBid { min: Uint128 },

    #[error("Only owner can close bids")]
    Unauthorized,

    #[error("Cannot retract while bids are still open")]
    BidsStillOpen,

    #[error("Winning bidder cannot retract")]
    WinnerCannotRetract,
}
