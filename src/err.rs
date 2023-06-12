use cosmwasm_std::{Addr, StdError, Uint128};
use cw_utils::PaymentError;
use thiserror::Error;

pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("{sender} is not contract admin")]
    Unauthorized { sender: Addr },
    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),
    #[error("Insufficient balance: required: {amount}, balance: {balance}")]
    InsufficientBalance { amount: Uint128, balance: Uint128 },
    #[error("Deposit error: {msg}")]
    Deposit { msg: String },
    #[error("User has not yet interacted with the contract")]
    NoBalance {},
}