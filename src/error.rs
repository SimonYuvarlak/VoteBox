use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Vote pool has ended")]
    Expired {},

    #[error("Vote pool not ended")]
    Unexpired {},

    #[error("This pool is free")]
    FreeVotes {},

    #[error("Insufficient balance")]
    InsufficientBalance {},

    #[error("Send native tokens")]
    SendNativeTokens {},

    #[error("Send native tokens")]
    NotSupportDenom {},

    #[error("You have already committed a vote")]
    VoterRepeat {},

    #[error("A VoteBox with the same topic already exists")]
    DuplicateVoteBox {},

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("Invalid Vote value - only 0,1,2 are valid")]
    InvalidVote {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
