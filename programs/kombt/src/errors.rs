use anchor_lang::prelude::*;

#[error_code]
pub enum KombtError {
    #[msg("stake amount must be positive")]
    InvalidStake,
    #[msg("match type tag is unknown")]
    InvalidMatchType,
    #[msg("stake token tag is unknown")]
    InvalidStakeToken,
    #[msg("match has already been accepted")]
    AlreadyAccepted,
    #[msg("match has already been settled")]
    AlreadySettled,
    #[msg("match has expired")]
    Expired,
    #[msg("match has not yet expired")]
    NotExpired,
    #[msg("signer is not authorized for this match")]
    Unauthorized,
    #[msg("oracle target is malformed")]
    InvalidOracleTarget,
    #[msg("vrf randomness is empty")]
    EmptyRandomness,
    #[msg("price snapshot did not move")]
    NoPriceMovement,
    #[msg("acceptor must differ from challenger")]
    SelfAccept,
    #[msg("challenge has been cancelled")]
    ChallengeCancelled,
}

// rev29

// rev37

// rev41

// rev70

// rev101

// rev103

// rev110

// rev131
