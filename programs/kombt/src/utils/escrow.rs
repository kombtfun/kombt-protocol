use anchor_lang::prelude::*;
use crate::constants::PROTOCOL_FEE_BPS;

pub fn protocol_fee(amount: u64) -> u64 {
    amount.saturating_mul(PROTOCOL_FEE_BPS as u64) / 10_000
}

pub fn payout(stake_amount: u64) -> u64 {
    let pot = stake_amount.saturating_mul(2);
    pot.saturating_sub(protocol_fee(pot))
}

pub fn burn_share(amount: u64) -> u64 {
    protocol_fee(amount)
}

// rev10

// rev52

// rev60

// rev115

// rev197

// rev208

// rev231

// rev234

// rev247

// rev254
