use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub outcome_yes_shares: [u8; 32], // q1 - confidential
    pub outcome_no_shares: [u8; 32], // q2 - confidential
    pub lsmr_b: [u8; 32], // confidential
    pub intial_deposite: [u8; 32], // confidential
    pub market_liquidity: [u8; 32], // confidential

    pub market_state: MarketStatus,
    pub market_outcome: MarketOutcome,

    pub dead_line: i64, // unix_time_stamp

    pub market_vault_bump: u8,
    pub market_bump: u8,

    pub nonce: u128,

    pub creater: Pubkey,
    #[max_len(32)]
    pub market_name: String,
    #[max_len(100)]
    pub description: String,
}

#[account]
#[derive(InitSpace, Default)]
pub struct UserWager {
    pub yes_shares: [u8; 32],
    pub no_shares: [u8; 32],
    pub nonce: u128,

    pub user_pubkey: Pubkey,
    pub market_pubkey: Pubkey,
    pub is_intialized: bool,

    pub user_wager_bump: u8,
}

#[derive(InitSpace, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
#[repr(u8)]
pub enum MarketStatus {
    Resolved, // The market has been resolved.
    Active, // The market is still active (not resolved).
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
#[repr(u8)]
pub enum MarketOutcome {
    YES,
    NO,
    NotResolved,
}
