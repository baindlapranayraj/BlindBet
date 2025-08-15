use anchor_lang::{prelude::*, solana_program::sysvar};
use arcium_anchor::prelude::*;
use arcium_client::idl::arcium::ID_CONST;

use crate::{
    error::ErrorCode,
    state::{Market, UserWager},
    COMP_DEF_OFFSET_SELL_SHARES, ID,
};

#[init_computation_definition_accounts("sell_shares", payer)]
#[derive(Accounts)]
pub struct SellSharesCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: comp_def_account, checked by arcium program.
    /// Can't check it here as it's not initialized yet.
    pub comp_def_account: UncheckedAccount<'info>,

    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}

#[queue_computation_accounts("sell_shares", signer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct SellShares<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"market",&market_account.market_name.as_bytes()[..32]], 
        bump = market_account.market_bump
    )]
    pub market_account: Account<'info, Market>,

    #[account(
        mut,
        seeds = [b"liquidity_pool",market_account.key().to_bytes().as_ref()],
        bump = market_account.market_vault_bump
    )]
    pub liquidity_pool: SystemAccount<'info>,

    #[account(
         mut,
         seeds = [b"user_account", signer.key().to_bytes().as_ref(),market_account.key().to_bytes().as_ref()], 
         bump = user_wager_acount.user_wager_bump
    )]
    pub user_wager_acount: Account<'info, UserWager>,

    // ============== Arcium Accounts  ==============

    // Some Common Queue instruction accounts
    #[account(
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Account<'info, MXEAccount>,

    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program
    pub mempool_account: UncheckedAccount<'info>,

    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program
    pub executing_pool: UncheckedAccount<'info>,

    #[account(
        mut,
        address = derive_comp_pda!(computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,

    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_SELL_SHARES)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,

    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,

    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS,
    )]
    pub clock_account: Account<'info, ClockAccount>,

    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}

#[callback_accounts("sell_shares", signer)]
#[derive(Accounts)]
pub struct SellSharesCallback<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"market",&market_account.market_name.as_bytes()[..32]], 
         bump = market_account.market_bump,
    )]
    pub market_account: Account<'info, Market>,

    #[account(
         mut,
         seeds = [b"user_account", user_wager_acount.user_pubkey.to_bytes().as_ref(),user_wager_acount.market_pubkey.to_bytes().as_ref()], 
         bump = user_wager_acount.user_wager_bump,
    )]
    pub user_wager_acount: Account<'info, UserWager>,

    #[account(
        mut,
        seeds = [b"liquidity_pool",market_account.key().to_bytes().as_ref()],
        bump = market_account.market_vault_bump
    )]
    pub liquidity_pool: SystemAccount<'info>,

    // Arcium Accounts
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_SELL_SHARES)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,

    #[account(
        address = sysvar::instructions::ID,
    )]
    /// CHECK: instructions_sysvar, checked by the account constraint
    pub instructions_sysvar: AccountInfo<'info>,

    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}
