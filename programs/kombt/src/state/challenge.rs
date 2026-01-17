use anchor_lang::prelude::*;

#[account]
pub struct ChallengeAccount {
    pub issuer: Pubkey,
    pub target_handle_hash: [u8; 32],
    pub target_wallet: Option<Pubkey>,
    pub stake_amount: u64,
    pub stake_token: u8,
    pub match_type: u8,
    pub oracle_target: [u8; 32],
    pub duration_minutes: u32,
    pub state: u8,
    pub created_at: i64,
    pub expires_at: i64,
    pub accepted_at: i64,
    pub bump: u8,
}

impl ChallengeAccount {
    pub const SIZE: usize = 32 + 32 + (1 + 32) + 8 + 1 + 1 + 32 + 4 + 1 + 8 + 8 + 8 + 1;
}

#[repr(u8)]
pub enum ChallengeState {
    Open = 0,
    Active = 1,
    Cancelled = 2,
    Expired = 3,
}

// rev22
