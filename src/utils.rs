#![no_std]

use soroban_sdk::{Env, Timepoint};

/// Utility functions for the escrow service.
pub struct Utils;

impl Utils {
    /// Calculates the deadline for a transaction based on the current ledger time and a duration.
    pub fn calculate_deadline(env: &Env, duration: u64) -> Timepoint {
        let current_time = env.ledger().timestamp();
        current_time + duration
    }

    /// Validates if the current time is past the given deadline.
    pub fn is_past_deadline(env: &Env, deadline: Timepoint) -> bool {
        let current_time = env.ledger().timestamp();
        current_time > deadline
    }
}
