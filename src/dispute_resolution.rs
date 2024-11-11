#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

use crate::types::{Dispute, Transaction, TransactionState};
use crate::errors::EscrowError;
use crate::audit::AuditModule;

/// Module for handling disputes and arbitration.
pub struct DisputeResolutionModule;

#[contractimpl]
impl DisputeResolutionModule {
    /// Raises a dispute for a given transaction.
    pub fn raise_dispute(env: Env, transaction_id: Symbol, raiser: Address) -> Result<(), EscrowError> {
        // Retrieve the transaction details from storage.
        let mut transaction: Transaction = env.storage().get(&transaction_id).ok_or(EscrowError::TransactionNotFound)?;

        // Ensure the transaction is in a state that allows disputes.
        if transaction.state != TransactionState::Deposit {
            return Err(EscrowError::InvalidTransactionState);
        }

        // Create a new dispute.
        let dispute = Dispute {
            transaction_id: transaction_id.clone(),
            raiser,
            state: DisputeState::Open,
        };

        // Store the dispute in the contract's storage.
        env.storage().set(&transaction_id, &dispute);

        // Update the transaction state to indicate a dispute has been raised.
        transaction.state = TransactionState::Dispute;
        env.storage().set(&transaction_id, &transaction);

        // Record the dispute action for audit purposes.
        AuditModule::record_action(&env, &transaction_id, "raise_dispute");

        Ok(())
    }

    /// Resolves a dispute through arbitration.
    pub fn resolve_dispute(env: Env, transaction_id: Symbol, arbitrator: Address) -> Result<(), EscrowError> {
        // Retrieve the dispute details from storage.
        let mut dispute: Dispute = env.storage().get(&transaction_id).ok_or(EscrowError::DisputeNotFound)?;

        // Ensure the dispute is open.
        if dispute.state != DisputeState::Open {
            return Err(EscrowError::InvalidDisputeState);
        }

        // Authorize the arbitrator.
        arbitrator.require_auth();

        // Resolve the dispute (logic for resolution would be implemented here).
        // For simplicity, we assume the dispute is resolved in favor of the seller.
        let mut transaction: Transaction = env.storage().get(&transaction_id).ok_or(EscrowError::TransactionNotFound)?;
        transaction.state = TransactionState::Complete;
        env.storage().set(&transaction_id, &transaction);

        // Update the dispute state to indicate it has been resolved.
        dispute.state = DisputeState::Resolved;
        env.storage().set(&transaction_id, &dispute);

        // Record the resolution action for audit purposes.
        AuditModule::record_action(&env, &transaction_id, "resolve_dispute");

        Ok(())
    }
}
