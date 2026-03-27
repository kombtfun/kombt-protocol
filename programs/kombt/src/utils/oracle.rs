use anchor_lang::prelude::*;

pub fn parse_oracle_target(buf: &[u8; 32]) -> &str {
    let end = buf.iter().position(|c| *c == 0).unwrap_or(32);
    core::str::from_utf8(&buf[..end]).unwrap_or("?")
}

pub fn snapshot_pair(price: i64) -> i64 {
    price
}

// rev7

// rev42

// rev87

// rev93

// rev132

// rev148

// rev158

// rev191
