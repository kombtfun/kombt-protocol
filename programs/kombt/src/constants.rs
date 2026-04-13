use anchor_lang::prelude::*;

pub const PROTOCOL_FEE_BPS: u16 = 200;
pub const MAX_DURATION_MINUTES: u32 = 60 * 24 * 7;
pub const MIN_STAKE_LAMPORTS: u64 = 10_000;
pub const MATCH_SEED: &[u8] = b"match";
pub const CHALLENGE_SEED: &[u8] = b"challenge";
pub const ESCROW_SEED: &[u8] = b"escrow";

#[constant]
pub const VERSION: u8 = 1;

// rev22

// rev38

// rev68

// rev76

// rev124

// rev136

// rev144

// rev147

// rev201

// rev204

// rev223

// rev236

// rev237

// rev244
