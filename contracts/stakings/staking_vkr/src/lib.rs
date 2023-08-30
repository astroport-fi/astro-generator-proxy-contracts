pub mod entrypoints;

mod executions;
mod queries;
mod states;

mod errors;
#[cfg(test)]
mod tests;
mod utils;

pub type ContractResult<T> = core::result::Result<T, errors::ContractError>;
