use anchor_lang::prelude::*;
use crate::errors::KombtError;
use crate::state::{ChallengeAccount, ChallengeState};

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub challenge: Account<'info, ChallengeAccount>,
}

pub fn handler(ctx: Context<Cancel>) -> Result<()> {
    let ch = &mut ctx.accounts.challenge;
    require!(ch.state == ChallengeState::Open as u8, KombtError::ChallengeCancelled);
    require!(ctx.accounts.authority.key() == ch.issuer, KombtError::Unauthorized);
    let now = Clock::get()?.unix_timestamp;
    require!(now >= ch.expires_at, KombtError::NotExpired);
    ch.state = ChallengeState::Cancelled as u8;
    Ok(())
}

// rev4

// rev17

// rev45

// rev59

// rev99

// rev145

// rev157

// rev167

// rev176

// rev188

// rev207

// rev217
