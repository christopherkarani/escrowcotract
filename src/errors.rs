#![no_std]

use soroban_sdk::{contracttype, Env, Symbol};

/// Enum representing possible errors in the escrow service.
#[contracttype]
#[derive(Debug, Eq, PartialEq)]
pub enum EscrowError {
    AgreementAlreadyExists,
    AgreementNotFound,
    TransactionNotFound,
    InvalidTransactionState,
    DisputeNotFound,
    InvalidDisputeState,
    Unauthorized,
    InsufficientFunds,
    DeadlineExceeded,
}

impl EscrowError {
    /// Converts the error to a human-readable string.
    pub fn to_string(&self) -> &'static str {
        match self {
            EscrowError::AgreementAlreadyExists => "Agreement already exists",
            EscrowError::AgreementNotFound => "Agreement not found",
            EscrowError::TransactionNotFound => "Transaction not found",
            EscrowError::InvalidTransactionState => "Invalid transaction state",
            EscrowError::DisputeNotFound => "Dispute not found",
            EscrowError::InvalidDisputeState => "Invalid dispute state",
            EscrowError::Unauthorized => "Unauthorized action",
            EscrowError::InsufficientFunds => "Insufficient funds",
            EscrowError::DeadlineExceeded => "Deadline exceeded",
        }
    }
}
