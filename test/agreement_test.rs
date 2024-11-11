#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    Address, Env, IntoVal, Symbol,
};

#[test]
fn test_create_agreement_success() {
    let env = Env::default();
    let agreement_module = env.register_contract(None, AgreementModule);

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let transaction_id = Symbol::from_str("txn1");
    let amount = 1000i128;
    let deadline = 10000u64;

    // Create a new agreement
    agreement_module.create_agreement(&env, &transaction_id, &buyer, &seller, &amount, &deadline).unwrap();

    // Retrieve the agreement and verify its details
    let agreement = agreement_module.get_agreement(&env, &transaction_id).unwrap();
    assert_eq!(agreement.buyer, buyer);
    assert_eq!(agreement.seller, seller);
    assert_eq!(agreement.amount, amount);
    assert_eq!(agreement.deadline, deadline);
    assert_eq!(agreement.state, TransactionState::Setup);
}

#[test]
#[should_panic(expected = "AgreementAlreadyExists")]
fn test_create_agreement_already_exists() {
    let env = Env::default();
    let agreement_module = env.register_contract(None, AgreementModule);

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let transaction_id = Symbol::from_str("txn1");
    let amount = 1000i128;
    let deadline = 10000u64;

    // Create a new agreement
    agreement_module.create_agreement(&env, &transaction_id, &buyer, &seller, &amount, &deadline).unwrap();

    // Attempt to create the same agreement again, which should panic
    agreement_module.create_agreement(&env, &transaction_id, &buyer, &seller, &amount, &deadline).unwrap();
}

#[test]
#[should_panic(expected = "AgreementNotFound")]
fn test_get_agreement_not_found() {
    let env = Env::default();
    let agreement_module = env.register_contract(None, AgreementModule);

    let transaction_id = Symbol::from_str("nonexistent_txn");

    // Attempt to retrieve a non-existent agreement, which should panic
    agreement_module.get_agreement(&env, &transaction_id).unwrap();
}
