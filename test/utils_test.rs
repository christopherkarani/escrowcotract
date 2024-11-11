#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Timepoint};

#[test]
fn test_calculate_deadline() {
    let env = Env::default();
    let duration = 1000u64;

    // Mock the current ledger timestamp
    env.ledger().with_mut(|ledger| {
        ledger.timestamp = 5000;
    });

    // Calculate the deadline
    let deadline = Utils::calculate_deadline(&env, duration);

    // Verify the deadline is correctly calculated
    assert_eq!(deadline, 6000);
}

#[test]
fn test_is_past_deadline() {
    let env = Env::default();

    // Mock the current ledger timestamp
    env.ledger().with_mut(|ledger| {
        ledger.timestamp = 5000;
    });

    // Test with a deadline in the past
    let past_deadline = 4000;
    assert!(Utils::is_past_deadline(&env, past_deadline));

    // Test with a deadline in the future
    let future_deadline = 6000;
    assert!(!Utils::is_past_deadline(&env, future_deadline));
}
