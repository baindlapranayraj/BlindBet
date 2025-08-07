use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub outcome_yes_shares: u64, // q1 - confidential
    pub outcome_no_shares: u64,  // q2 - confidential

    pub lsmr_b: u64,          // confidential
    pub intial_deposite: u64, // confidential
    pub dead_line: i64,       // unix_time_stamp

    pub market_state: MarketStatus,
    pub market_outcome: MarketOutcome,

    pub mint_yes_bump: u8,
    pub mint_no_bump: u8,
    pub market_vault_bump: u8,
    pub market_bump: u8,

    pub nonce: [u8; 16],

    pub creater: Pubkey,
    #[max_len(32)]
    pub market_name: String,
    #[max_len(100)]
    pub description: String,
}

#[derive(InitSpace, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
#[repr(u8)]
pub enum MarketStatus {
    Resolved, // The market has been resolved.
    Active,   // The market is still active (not resolved).
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
#[repr(u8)]
pub enum MarketOutcome {
    YES,
    NO,
    NotResolved,
}
