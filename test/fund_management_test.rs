#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    token::TokenClient,
    Address, Env, IntoVal, Symbol,
};

#[test]
fn test_deposit_funds_success() {
    let env = Env::default();
    let fund_management_module = env.register_contract(None, FundManagementModule);

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let transaction_id = Symbol::from_str("txn1");
    let amount = 1000i128;
    let token = env.register_stellar_asset_contract(buyer.clone());

    // Mock the token client for transferring tokens
    let token_client = TokenClient::new(&env, &token);
    token_client.mint(&buyer, &amount);

    // Create a transaction in the setup state
    let transaction = Transaction {
        id: transaction_id.clone(),
        buyer: buyer.clone(),
        seller: seller.clone(),
        amount,
        token: token.clone(),
        state: TransactionState::Setup,
    };
    env.storage().set(&transaction_id, &transaction);

    // Mock authorization for the buyer
    env.mock_auths(&[MockAuth {
        address: &buyer,
        invoke: &MockAuthInvoke {
            contract: &fund_management_module,
            fn_name: "deposit_funds",
            args: (&transaction).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    // Deposit funds into escrow
    fund_management_module.deposit_funds(&env, &transaction).unwrap();

    // Verify the transaction state is updated to Deposit
    let updated_transaction: Transaction = env.storage().get(&transaction_id).unwrap();
    assert_eq!(updated_transaction.state, TransactionState::Deposit);

    // Verify the funds are transferred to the contract
    assert_eq!(token_client.balance(&env.current_contract_address()), amount);
}

#[test]
#[should_panic(expected = "InvalidTransactionState")]
fn test_deposit_funds_invalid_state() {
    let env = Env::default();
    let fund_management_module = env.register_contract(None, FundManagementModule);

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let transaction_id = Symbol::from_str("txn1");
    let amount = 1000i128;
    let token = env.register_stellar_asset_contract(buyer.clone());

    // Create a transaction in the deposit state
    let transaction = Transaction {
        id: transaction_id.clone(),
        buyer: buyer.clone(),
        seller: seller.clone(),
        amount,
        token: token.clone(),
        state: TransactionState::Deposit,
    };
    env.storage().set(&transaction_id, &transaction);

    // Attempt to deposit funds into escrow, which should panic
    fund_management_module.deposit_funds(&env, &transaction).unwrap();
}

#[test]
fn test_release_funds_success() {
    let env = Env::default();
    let fund_management_module = env.register_contract(None, FundManagementModule);

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let transaction_id = Symbol::from_str("txn1");
    let amount = 1000i128;
    let token = env.register_stellar_asset_contract(buyer.clone());

    // Mock the token client for transferring tokens
    let token_client = TokenClient::new(&env, &token);
    token_client.mint(&env.current_contract_address(), &amount);

    // Create a transaction in the deposit state
    let transaction = Transaction {
        id: transaction_id.clone(),
        buyer: buyer.clone(),
        seller: seller.clone(),
        amount,
        token: token.clone(),
        state: TransactionState::Deposit,
    };
    env.storage().set(&transaction_id, &transaction);

    // Release funds from escrow
    fund_management_module.release_funds(&env, &transaction).unwrap();

    // Verify the transaction state is updated to Complete
    let updated_transaction: Transaction = env.storage().get(&transaction_id).unwrap();
    assert_eq!(updated_transaction.state, TransactionState::Complete);

    // Verify the funds are transferred to the seller
    assert_eq!(token_client.balance(&seller), amount);
}

#[test]
#[should_panic(expected = "InvalidTransactionState")]
fn test_release_funds_invalid_state() {
    let env = Env::default();
    let fund_management_module = env.register_contract(None, FundManagementModule);

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let transaction_id = Symbol::from_str("txn1");
    let amount = 1000i128;
    let token = env.register_stellar_asset_contract(buyer.clone());

    // Create a transaction in the setup state
    let transaction = Transaction {
        id: transaction_id.clone(),
        buyer: buyer.clone(),
        seller: seller.clone(),
        amount,
        token: token.clone(),
        state: TransactionState::Setup,
    };
    env.storage().set(&transaction_id, &transaction);

    // Attempt to release funds from escrow, which should panic
    fund_management_module.release_funds(&env, &transaction).unwrap();
}
