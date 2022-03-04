use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Vote pool has ended")]
    Expired {},

    #[error("overflow")]
    Overflow {error: OverflowError},

    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}

impl From<OverflowError> for ContractError {
    fn from(error: OverflowError) -> Self {
        ContractError::Overflow {error}
    }
}
