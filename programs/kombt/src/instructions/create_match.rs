use anchor_lang::prelude::*;
use crate::constants::{MATCH_SEED, MIN_STAKE_LAMPORTS};
use crate::errors::KombtError;
use crate::state::{MatchAccount, MatchState};

#[derive(Accounts)]
#[instruction(match_type: u8, stake_token: u8, stake_amount: u64)]
pub struct CreateMatch<'info> {
    #[account(mut)]
    pub challenger: Signer<'info>,

    #[account(
        init,
        payer = challenger,
        space = 8 + MatchAccount::SIZE,
        seeds = [MATCH_SEED, challenger.key().as_ref(), &Clock::get()?.unix_timestamp.to_le_bytes()],
        bump,
    )]
    pub match_account: Account<'info, MatchAccount>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateMatch>,
    match_type: u8,
    stake_token: u8,
    stake_amount: u64,
    oracle_target: [u8; 32],
    expires_at: i64,
) -> Result<()> {
    require!(stake_amount >= MIN_STAKE_LAMPORTS, KombtError::InvalidStake);
    require!(match_type <= 2, KombtError::InvalidMatchType);
    require!(stake_token <= 2, KombtError::InvalidStakeToken);
    let now = Clock::get()?.unix_timestamp;
    require!(expires_at > now, KombtError::Expired);

    let ma = &mut ctx.accounts.match_account;
    ma.challenger = ctx.accounts.challenger.key();
    ma.acceptor = None;
    ma.stake_amount = stake_amount;
    ma.stake_token = stake_token;
    ma.match_type = match_type;
    ma.oracle_target = oracle_target;
    ma.snapshot_start = 0;
    ma.snapshot_end = 0;
    ma.state = MatchState::Pending as u8;
    ma.vrf_seed = [0u8; 32];
    ma.winner = None;
    ma.health_left = 100;
    ma.health_right = 100;
    ma.created_at = now;
    ma.started_at = 0;
    ma.expires_at = expires_at;
    ma.settled_at = 0;
    ma.bump = ctx.bumps.match_account;
    Ok(())
}

// rev15

// rev93

// rev105

// rev137

// rev147

// rev153

// rev154

// rev156

// rev160

// rev214

// rev218

// rev221

// rev222

// rev227

// rev230

// rev260

// rev293
