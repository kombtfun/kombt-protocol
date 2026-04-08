use anchor_lang::prelude::*;

#[account]
pub struct MatchAccount {
    pub challenger: Pubkey,
    pub acceptor: Option<Pubkey>,
    pub stake_amount: u64,
    pub stake_token: u8,
    pub match_type: u8,
    pub oracle_target: [u8; 32],
    pub snapshot_start: i64,
    pub snapshot_end: i64,
    pub state: u8,
    pub vrf_seed: [u8; 32],
    pub winner: Option<Pubkey>,
    pub health_left: u8,
    pub health_right: u8,
    pub created_at: i64,
    pub started_at: i64,
    pub expires_at: i64,
    pub settled_at: i64,
    pub bump: u8,
}

impl MatchAccount {
    pub const SIZE: usize = 32 + (1 + 32) + 8 + 1 + 1 + 32 + 8 + 8 + 1 + 32 + (1 + 32) + 1 + 1 + 8 + 8 + 8 + 8 + 1;
}

#[repr(u8)]
pub enum MatchType {
    Price = 0,
    Time = 1,
    Vrf = 2,
}

#[repr(u8)]
pub enum MatchState {
    Pending = 0,
    Active = 1,
    Settled = 2,
    Cancelled = 3,
}

#[repr(u8)]
pub enum StakeToken {
    Kombt = 0,
    Sol = 1,
    Usdc = 2,
}

// rev9

// rev32

// rev63

// rev68

// rev90

// rev116

// rev173

// rev185

// rev190

// rev204

// rev220

// rev229

// rev233
