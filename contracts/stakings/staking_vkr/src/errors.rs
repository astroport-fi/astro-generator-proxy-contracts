use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Asset mismatch")]
    AssetMismatch {},

    #[error("Not found")]
    NotFound {},

    #[error("Exceed limit")]
    ExceedLimit {},

    #[error("Already exists")]
    AlreadyExists {},

    #[error(
        "Schedule error. Should satisfy: (start_block < end_block, end_block > current_block)"
    )]
    ScheduleError {},

    #[error("Schedules amount error. The total amount should be equal to the received amount.")]
    SchedulesAmountError {},
}
