#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol};

/// Module for auditing actions within the escrow service.
pub struct AuditModule;

#[contractimpl]
impl AuditModule {
    /// Records an action taken on a transaction for audit purposes.
    pub fn record_action(env: Env, transaction_id: &Symbol, action: &str) {
        // Construct the audit log entry.
        let log_entry = format!("Transaction: {}, Action: {}", transaction_id, action);

        // Store the log entry in the contract's storage.
        // For simplicity, we append the log entry to a list of logs.
        let mut logs: Vec<String> = env.storage().get(&"audit_logs").unwrap_or_default();
        logs.push(log_entry);
        env.storage().set(&"audit_logs", &logs);
    }

    /// Retrieves the audit logs for inspection.
    pub fn get_audit_logs(env: Env) -> Vec<String> {
        // Retrieve the audit logs from storage.
        env.storage().get(&"audit_logs").unwrap_or_default()
    }
}
