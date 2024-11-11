#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    Address, Env, IntoVal, Symbol,
};

#[test]
fn test_raise_dispute_success() {
    let env = Env::default();
    let dispute_resolution_module = env.register_contract(None, DisputeResolutionModule);

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let transaction_id = Symbol::from_str("txn1");

    // Create a transaction in the deposit state
    let transaction = Transaction {
        id: transaction_id.clone(),
        buyer: buyer.clone(),
        seller: seller.clone(),
        amount: 1000,
        token: Symbol::from_str("token"),
        state: TransactionState::Deposit,
    };
    env.storage().set(&transaction_id, &transaction);

    // Raise a dispute
    dispute_resolution_module.raise_dispute(&env, &transaction_id, &buyer).unwrap();

    // Verify the transaction state is updated to Dispute
    let updated_transaction: Transaction = env.storage().get(&transaction_id).unwrap();
    assert_eq!(updated_transaction.state, TransactionState::Dispute);

    // Verify the dispute is stored
    let dispute: Dispute = env.storage().get(&transaction_id).unwrap();
    assert_eq!(dispute.transaction_id, transaction_id);
    assert_eq!(dispute.raiser, buyer);
    assert_eq!(dispute.state, DisputeState::Open);
}

#[test]
#[should_panic(expected = "InvalidTransactionState")]
fn test_raise_dispute_invalid_state() {
    let env = Env::default();
    let dispute_resolution_module = env.register_contract(None, DisputeResolutionModule);

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let transaction_id = Symbol::from_str("txn1");

    // Create a transaction in the setup state
    let transaction = Transaction {
        id: transaction_id.clone(),
        buyer: buyer.clone(),
        seller: seller.clone(),
        amount: 1000,
        token: Symbol::from_str("token"),
        state: TransactionState::Setup,
    };
    env.storage().set(&transaction_id, &transaction);

    // Attempt to raise a dispute, which should panic
    dispute_resolution_module.raise_dispute(&env, &transaction_id, &buyer).unwrap();
}

#[test]
fn test_resolve_dispute_success() {
    let env = Env::default();
    let dispute_resolution_module = env.register_contract(None, DisputeResolutionModule);

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let arbitrator = Address::generate(&env);
    let transaction_id = Symbol::from_str("txn1");

    // Create a transaction in the dispute state
    let transaction = Transaction {
        id: transaction_id.clone(),
        buyer: buyer.clone(),
        seller: seller.clone(),
        amount: 1000,
        token: Symbol::from_str("token"),
        state: TransactionState::Dispute,
    };
    env.storage().set(&transaction_id, &transaction);

    // Create a dispute
    let dispute = Dispute {
        transaction_id: transaction_id.clone(),
        raiser: buyer.clone(),
        state: DisputeState::Open,
    };
    env.storage().set(&transaction_id, &dispute);

    // Mock authorization for the arbitrator
    env.mock_auths(&[MockAuth {
        address: &arbitrator,
        invoke: &MockAuthInvoke {
            contract: &dispute_resolution_module,
            fn_name: "resolve_dispute",
            args: (&transaction_id, &arbitrator).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    // Resolve the dispute
    dispute_resolution_module.resolve_dispute(&env, &transaction_id, &arbitrator).unwrap();

    // Verify the transaction state is updated to Complete
    let updated_transaction: Transaction = env.storage().get(&transaction_id).unwrap();
    assert_eq!(updated_transaction.state, TransactionState::Complete);

    // Verify the dispute state is updated to Resolved
    let updated_dispute: Dispute = env.storage().get(&transaction_id).unwrap();
    assert_eq!(updated_dispute.state, DisputeState::Resolved);
}

#[test]
#[should_panic(expected = "InvalidDisputeState")]
fn test_resolve_dispute_invalid_state() {
    let env = Env::default();
    let dispute_resolution_module = env.register_contract(None, DisputeResolutionModule);

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let arbitrator = Address::generate(&env);
    let transaction_id = Symbol::from_str("txn1");

    // Create a transaction in the dispute state
    let transaction = Transaction {
        id: transaction_id.clone(),
        buyer: buyer.clone(),
        seller: seller.clone(),
        amount: 1000,
        token: Symbol::from_str("token"),
        state: TransactionState::Dispute,
    };
    env.storage().set(&transaction_id, &transaction);

    // Create a resolved dispute
    let dispute = Dispute {
        transaction_id: transaction_id.clone(),
        raiser: buyer.clone(),
        state: DisputeState::Resolved,
    };
    env.storage().set(&transaction_id, &dispute);

    // Attempt to resolve the dispute, which should panic
    dispute_resolution_module.resolve_dispute(&env, &transaction_id, &arbitrator).unwrap();
}
