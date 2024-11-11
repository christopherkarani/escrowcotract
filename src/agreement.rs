#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec};

use crate::types::{Agreement, Transaction};
use crate::errors::EscrowError;

/// Module for managing transaction agreements.
pub struct AgreementModule;

#[contractimpl]
impl AgreementModule {
    /// Creates a new agreement with specified terms.
    pub fn create_agreement(
        env: Env,
        transaction_id: Symbol,
        buyer: Address,
        seller: Address,
        amount: i128,
        deadline: u64,
    ) -> Result<(), EscrowError> {
        // Ensure the transaction ID is unique.
        if env.storage().has(&transaction_id) {
            return Err(EscrowError::AgreementAlreadyExists);
        }

        // Create a new agreement.
        let agreement = Agreement {
            buyer,
            seller,
            amount,
            deadline,
            state: TransactionState::Setup,
        };

        // Store the agreement in the contract's storage.
        env.storage().set(&transaction_id, &agreement);

        Ok(())
    }

    /// Retrieves an existing agreement by transaction ID.
    pub fn get_agreement(env: Env, transaction_id: Symbol) -> Result<Agreement, EscrowError> {
        // Retrieve the agreement from storage.
        env.storage()
            .get(&transaction_id)
            .ok_or(EscrowError::AgreementNotFound)
    }
}
