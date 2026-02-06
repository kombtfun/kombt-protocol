use anchor_lang::prelude::*;
use crate::errors::KombtError;
use crate::state::{MatchAccount, MatchState};

#[derive(Accounts)]
pub struct SettlePrice<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub match_account: Account<'info, MatchAccount>,
}

pub fn handler(ctx: Context<SettlePrice>, snapshot_end: i64) -> Result<()> {
    let ma = &mut ctx.accounts.match_account;
    require!(ma.state == MatchState::Active as u8, KombtError::AlreadySettled);
    require!(snapshot_end != ma.snapshot_start, KombtError::NoPriceMovement);

    let acceptor = ma.acceptor.ok_or(KombtError::AlreadySettled)?;
    let challenger_long = ma.match_type & 1 == 0;
    let up = snapshot_end > ma.snapshot_start;
    let challenger_wins = challenger_long == up;
    let winner = if challenger_wins { ma.challenger } else { acceptor };
    ma.winner = Some(winner);
    ma.snapshot_end = snapshot_end;
    ma.health_left = if challenger_wins { 100 } else { 0 };
    ma.health_right = if challenger_wins { 0 } else { 100 };
    ma.state = MatchState::Settled as u8;
    ma.settled_at = Clock::get()?.unix_timestamp;
    Ok(())
}

// rev20

// rev35

// rev48

// rev54

// rev56

// rev65
