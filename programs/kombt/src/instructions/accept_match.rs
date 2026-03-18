use anchor_lang::prelude::*;
use crate::errors::KombtError;
use crate::state::{MatchAccount, MatchState};

#[derive(Accounts)]
pub struct AcceptMatch<'info> {
    #[account(mut)]
    pub acceptor: Signer<'info>,

    #[account(mut)]
    pub match_account: Account<'info, MatchAccount>,
}

pub fn handler(ctx: Context<AcceptMatch>) -> Result<()> {
    let ma = &mut ctx.accounts.match_account;
    require!(ma.state == MatchState::Pending as u8, KombtError::AlreadyAccepted);
    let now = Clock::get()?.unix_timestamp;
    require!(now < ma.expires_at, KombtError::Expired);
    require!(ctx.accounts.acceptor.key() != ma.challenger, KombtError::SelfAccept);
    ma.acceptor = Some(ctx.accounts.acceptor.key());
    ma.state = MatchState::Active as u8;
    ma.started_at = now;
    Ok(())
}

// rev41

// rev42

// rev75

// rev92

// rev103

// rev107

// rev110

// rev117

// rev138

// rev163
