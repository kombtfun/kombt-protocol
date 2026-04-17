use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

declare_id!("ELbd3fkvCeqDCrHdoZ2FLzCyo5jNynik2bLzpSFyGcqs");

#[program]
pub mod kombt {
    use super::*;

    pub fn create_match(
        ctx: Context<CreateMatch>,
        match_type: u8,
        stake_token: u8,
        stake_amount: u64,
        oracle_target: [u8; 32],
        expires_at: i64,
    ) -> Result<()> {
        instructions::create_match::handler(ctx, match_type, stake_token, stake_amount, oracle_target, expires_at)
    }

    pub fn accept_match(ctx: Context<AcceptMatch>) -> Result<()> {
        instructions::accept_match::handler(ctx)
    }

    pub fn settle_vrf(ctx: Context<SettleVrf>, randomness: [u8; 32]) -> Result<()> {
        instructions::settle_vrf::handler(ctx, randomness)
    }

    pub fn settle_price(ctx: Context<SettlePrice>, snapshot_end: i64) -> Result<()> {
        instructions::settle_price::handler(ctx, snapshot_end)
    }

    pub fn issue_challenge(
        ctx: Context<IssueChallenge>,
        stake_token: u8,
        stake_amount: u64,
        match_type: u8,
        oracle_target: [u8; 32],
        duration_minutes: u32,
    ) -> Result<()> {
        instructions::issue_challenge::handler(ctx, stake_token, stake_amount, match_type, oracle_target, duration_minutes)
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        instructions::cancel::handler(ctx)
    }
}

// rev14

// rev35

// rev80

// rev101

// rev135

// rev160

// rev164

// rev176

// rev192

// rev197

// rev229

// rev238

// rev260
