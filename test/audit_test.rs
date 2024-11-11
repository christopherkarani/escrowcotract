#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Symbol};

#[test]
fn test_record_action() {
    let env = Env::default();
    let audit_module = env.register_contract(None, AuditModule);

    let transaction_id = Symbol::from_str("txn1");
    let action = "create_agreement";

    // Record an action
    audit_module.record_action(&env, &transaction_id, action);

    // Retrieve the audit logs and verify the action is recorded
    let logs = audit_module.get_audit_logs(&env);
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], format!("Transaction: {}, Action: {}", transaction_id, action));
}

#[test]
fn test_get_audit_logs_empty() {
    let env = Env::default();
    let audit_module = env.register_contract(None, AuditModule);

    // Retrieve the audit logs when no actions have been recorded
    let logs = audit_module.get_audit_logs(&env);
    assert!(logs.is_empty());
}

#[test]
fn test_multiple_actions() {
    let env = Env::default();
    let audit_module = env.register_contract(None, AuditModule);

    let transaction_id1 = Symbol::from_str("txn1");
    let transaction_id2 = Symbol::from_str("txn2");
    let action1 = "create_agreement";
    let action2 = "deposit_funds";

    // Record multiple actions
    audit_module.record_action(&env, &transaction_id1, action1);
    audit_module.record_action(&env, &transaction_id2, action2);

    // Retrieve the audit logs and verify both actions are recorded
    let logs = audit_module.get_audit_logs(&env);
    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0], format!("Transaction: {}, Action: {}", transaction_id1, action1));
    assert_eq!(logs[1], format!("Transaction: {}, Action: {}", transaction_id2, action2));
}
