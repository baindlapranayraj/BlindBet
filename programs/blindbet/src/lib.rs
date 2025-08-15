#![allow(warnings)]

use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;
pub mod error;
pub mod instructions;
pub mod state;

use arcium_client::idl::arcium::types::CallbackAccount;

use error::ErrorCode;
use instructions::*;
use state::*;

const COMP_DEF_OFFSET_CREATE_MARKET: u32 = comp_def_offset("create_market");
const COMP_DEF_OFFSET_BUY_SHARES: u32 = comp_def_offset("buy_shares");
const COMP_DEF_OFFSET_SELL_SHARES: u32 = comp_def_offset("sell_shares");
const COMP_DEF_OFFSET_RESOLVE_MARKET: u32 = comp_def_offset("resolve_market");
const COMP_DEF_OFFSET_CLAIM_AMOUNT: u32 = comp_def_offset("claim_amount");

declare_id!("38wcjbfbT2aNs6kiikGvZeQ4ULKg9qkmtmXTuuQNLqFw");

#[arcium_program]
pub mod blindbet {
    use std::vec;

    use anchor_lang::system_program::{transfer, Transfer};

    use super::*;

    pub fn create_market_comp_def(ctx: Context<CreateMarketCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, true, 0, None, None)?;
        Ok(())
    }

    pub fn create_market(
        ctx: Context<CreateMarket>,
        computation_offset: u64,
        market_name: String,
        market_description: String,
        dead_line: i64,

        lmsr_b: [u8; 32],
        pub_key: [u8; 32],
        nonce: u128
    ) -> Result<()> {
        let args =
            vec![
            Argument::ArcisPubkey(pub_key),
            Argument::PlaintextU128(nonce),
            Argument::EncryptedU8(lmsr_b),
        ];

        ctx.accounts.market_account.set_inner(Market {
            outcome_yes_shares: [0; 32],
            outcome_no_shares: [0; 32],
            lsmr_b: [0; 32],
            intial_deposite: [0; 32],
            dead_line,
            market_liquidity: [0; 32],

            market_state: MarketStatus::Active,
            market_outcome: MarketOutcome::NotResolved,

            market_vault_bump: ctx.bumps.liquidity_pool,
            market_bump: ctx.bumps.market_account,
            nonce,
            creater: ctx.accounts.signer.key(),
            market_name,
            description: market_description,
        });

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            vec![CallbackAccount {
                is_writable: true,
                pubkey: ctx.accounts.market_account.key(),
            }],
            None
        )?;

        Ok(())
    }

    #[arcium_callback(encrypted_ix = "create_market")]
    pub fn create_market_callback(
        ctx: Context<CreateMarketCallback>,
        output: ComputationOutputs<CreateMarketOutput>
    ) -> Result<()> {
        let o: MXEEncryptedStruct<5> = match output {
            ComputationOutputs::Success(CreateMarketOutput { field_0 }) => field_0,
            _ => {
                return Err(ErrorCode::AbortedComputation.into());
            }
        };

        // updating the confidential fields
        ctx.accounts.market_account.nonce = o.nonce;
        ctx.accounts.market_account.outcome_yes_shares = o.ciphertexts[0];
        ctx.accounts.market_account.outcome_no_shares = o.ciphertexts[1];
        ctx.accounts.market_account.lsmr_b = o.ciphertexts[2];
        ctx.accounts.market_account.intial_deposite = o.ciphertexts[3];
        ctx.accounts.market_account.market_liquidity = o.ciphertexts[4];

        Ok(())
    }

    pub fn buy_shares_comp_def(ctx: Context<BuySharesCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, true, 0, None, None)?;
        Ok(())
    }

    pub fn buy_shares(
        ctx: Context<BuyShares>,
        computation_offset: u64,
        is_yes: [u8; 32],
        amount: [u8; 32],
        nonce: u128,
        pub_key: [u8; 32]
    ) -> Result<()> {
        // do the qeue computation
        if !ctx.accounts.user_wager_acount.is_intialized {
            ctx.accounts.user_wager_acount.set_inner(UserWager {
                yes_shares: [0; 32],
                no_shares: [0; 32],
                nonce,

                user_pubkey: ctx.accounts.signer.key(),
                market_pubkey: ctx.accounts.market_account.key(),
                is_intialized: true,
                user_wager_bump: ctx.bumps.user_wager_acount,
            });
        }

        let args =
            vec![
            Argument::ArcisPubkey(pub_key),
            Argument::PlaintextU128(nonce),
            Argument::EncryptedBool(is_yes),  // Input 
            Argument::EncryptedU64(amount),   // Input 

            Argument::PlaintextU128(ctx.accounts.market_account.nonce),
            Argument::Account(ctx.accounts.market_account.key(), 8 , 32*5),     // LMSR Struct 
            Argument::Account(ctx.accounts.user_wager_acount.key(), 8 , 32*2),  // UserWager Struct
        ];

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            vec![
            CallbackAccount {
                is_writable: true,
                 pubkey: ctx.accounts.market_account.key(),
            },
            CallbackAccount {
                is_writable: true,
                pubkey: ctx.accounts.user_wager_acount.key()
            }
        ],
            None
        )?;

        Ok(())
    }

    //todo: transfer the user amount to liquidity pool and upate the user_wager acount with new yes or no shares
    #[arcium_callback(encrypted_ix = "buy_shares")]
    pub fn buy_shares_callback(
        ctx: Context<BuySharesCallback>,
        output: ComputationOutputs<BuySharesOutput>
    ) -> Result<()> {
        let o: BuySharesTupleStruct0 = match output {
            ComputationOutputs::Success(BuySharesOutput { field_0 }) => field_0,
            _ => {
                return Err(ErrorCode::AbortedComputation.into());
            }
        };

        let lmsr_data = o.field_0;
        let user_wager_data = o.field_1;
        let transfer_data = o.field_2;

        // Update Market Struct
        ctx.accounts.market_account.nonce = lmsr_data.nonce;
        ctx.accounts.market_account.outcome_yes_shares = lmsr_data.ciphertexts[0];
        ctx.accounts.market_account.outcome_no_shares = lmsr_data.ciphertexts[1];

        // Update the UserWager Struct
        ctx.accounts.user_wager_acount.nonce = user_wager_data.nonce;
        ctx.accounts.user_wager_acount.yes_shares = user_wager_data.ciphertexts[0];
        ctx.accounts.user_wager_acount.no_shares = user_wager_data.ciphertexts[1];

        // transfer logic

        let ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.liquidity_pool.to_account_info(),
        });

        // transfer(ctx, transfer_data.ciphertexts[1]);

        Ok(())
    }
}

#[event]
pub struct SumEvent {
    pub sum: [u8; 32],
    pub nonce: [u8; 16],
}
