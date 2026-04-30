use anchor_lang::prelude::*;
use crate::errors::KombtError;
use crate::state::{MatchAccount, MatchState};

#[derive(Accounts)]
pub struct SettleVrf<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub match_account: Account<'info, MatchAccount>,
}

pub fn handler(ctx: Context<SettleVrf>, randomness: [u8; 32]) -> Result<()> {
    let ma = &mut ctx.accounts.match_account;
    require!(ma.state == MatchState::Active as u8, KombtError::AlreadySettled);
    require!(randomness != [0u8; 32], KombtError::EmptyRandomness);

    let bit = randomness[0] & 1;
    let acceptor = ma.acceptor.ok_or(KombtError::AlreadySettled)?;
    let winner = if bit == 0 { ma.challenger } else { acceptor };
    ma.winner = Some(winner);
    ma.health_left = if winner == ma.challenger { 100 } else { 0 };
    ma.health_right = if winner == acceptor { 100 } else { 0 };
    ma.state = MatchState::Settled as u8;
    ma.settled_at = Clock::get()?.unix_timestamp;
    ma.vrf_seed = randomness;
    Ok(())
}

// rev14

// rev28

// rev31

// rev34

// rev77

// rev110

// rev157

// rev162

// rev164

// rev165

// rev167

// rev169

// rev203

// rev210

// rev218

// rev219

// rev274

// rev284

// rev298
