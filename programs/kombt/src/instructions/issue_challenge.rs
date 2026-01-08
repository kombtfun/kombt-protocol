use anchor_lang::prelude::*;
use crate::constants::{CHALLENGE_SEED, MAX_DURATION_MINUTES, MIN_STAKE_LAMPORTS};
use crate::errors::KombtError;
use crate::state::{ChallengeAccount, ChallengeState};

#[derive(Accounts)]
#[instruction(stake_token: u8, stake_amount: u64, match_type: u8)]
pub struct IssueChallenge<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,

    #[account(
        init,
        payer = issuer,
        space = 8 + ChallengeAccount::SIZE,
        seeds = [CHALLENGE_SEED, issuer.key().as_ref(), &Clock::get()?.unix_timestamp.to_le_bytes()],
        bump,
    )]
    pub challenge: Account<'info, ChallengeAccount>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<IssueChallenge>,
    stake_token: u8,
    stake_amount: u64,
    match_type: u8,
    oracle_target: [u8; 32],
    duration_minutes: u32,
) -> Result<()> {
    require!(stake_amount >= MIN_STAKE_LAMPORTS, KombtError::InvalidStake);
    require!(stake_token <= 2, KombtError::InvalidStakeToken);
    require!(match_type <= 2, KombtError::InvalidMatchType);
    require!(duration_minutes <= MAX_DURATION_MINUTES, KombtError::Expired);

    let now = Clock::get()?.unix_timestamp;
    let ch = &mut ctx.accounts.challenge;
    ch.issuer = ctx.accounts.issuer.key();
    ch.target_handle_hash = [0u8; 32];
    ch.target_wallet = None;
    ch.stake_amount = stake_amount;
    ch.stake_token = stake_token;
    ch.match_type = match_type;
    ch.oracle_target = oracle_target;
    ch.duration_minutes = duration_minutes;
    ch.state = ChallengeState::Open as u8;
    ch.created_at = now;
    ch.expires_at = now + (duration_minutes as i64) * 60;
    ch.accepted_at = 0;
    ch.bump = ctx.bumps.challenge;
    Ok(())
}

// rev4
