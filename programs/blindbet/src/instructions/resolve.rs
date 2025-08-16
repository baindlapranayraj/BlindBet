use anchor_lang::{prelude::*, solana_program::sysvar};
use arcium_anchor::prelude::*;
use arcium_client::idl::arcium::ID_CONST;

use crate::{
    error::ErrorCode,
    state::{Market, UserWager},
    COMP_DEF_OFFSET_RESOLVE_MARKET, ID,
};

#[init_computation_definition_accounts("resolve_market", payer)]
#[derive(Accounts)]
pub struct ResolveMarketCompDef<'info> {
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

#[queue_computation_accounts("resolve_market", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct ResolveMarket<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"market",&market_account.market_name.as_bytes()[..32]], 
         bump = market_account.market_bump,
    )]
    pub market_account: Account<'info, Market>,

    // Arcium Accounts

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
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_RESOLVE_MARKET)
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

#[callback_accounts("resolve_market", signer)]
#[derive(Accounts)]
pub struct ResolveMarketCallback<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"market",&market_account.market_name.as_bytes()[..32]],
         bump = market_account.market_bump,
    )]
    pub market_account: Account<'info, Market>,

    // Arcium Accounts
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_RESOLVE_MARKET)
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

