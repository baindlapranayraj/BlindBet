use arcis_imports::*;
pub mod helper;

// cost function = b.ln(e.pow(q1/b) + e.pow(q2/b)); all the calculations are done in off-chain

// - create_market - init the conf struct as 0, 0, b_value (from client side encrypted)
// - buy_shares - calculate price using LMSR -
// - sell_shares - calculate price using LMSR

#[encrypted]
mod circuits {
    use arcis_imports::*;

    // onchain account data - Mxe
    pub struct LMSR {
        pub total_yes_shares: u64,
        pub total_no_shares: u64,
        pub lmsr_b: u64, // should come from client

        pub initial_deposit: u64,
    }

    impl LMSR {

        pub fn init_market_data(lmsr_b:u64) -> LMSR {
            let mut lmsr_data = LMSR {
                lmsr_b,
                total_yes_shares: 0,
                total_no_shares: 0,
                initial_deposit: 0
            };

            let initial_deposit =  lmsr_data.cost_calculation(0, 0, lmsr_b);

            lmsr_data.initial_deposit = initial_deposit;
            lmsr_data
        }

        pub fn cost_calculation(&self, yes_shares: u64, no_shares: u64, lmsr_b: u64) -> u64 {
            // cost function = b * ln(e^(q1/b) + e^(q2/b))
            // Using fixed-point arithmetic with scaling factor of 10^6

            let b = lmsr_b;
            let scale: u64 = 1_000_000; // 10^6 for 6 decimal places precision

            // Calculate e^(q1/b) and e^(q2/b) using fixed-point approximation
            // For small values, we can use: e^x ≈ 1 + x + x^2/2 + x^3/6
            let q1_over_b = (yes_shares * scale) / b;
            let q2_over_b = (no_shares * scale) / b;

            // Approximate e^(q1/b) and e^(q2/b) using Taylor series (first few terms)
            let exp_q1 = scale + q1_over_b + (q1_over_b * q1_over_b) / (2 * scale);
            let exp_q2 = scale + q2_over_b + (q2_over_b * q2_over_b) / (2 * scale);

            let outcome_sum = exp_q1 + exp_q2;

            // Approximate ln(x) using: ln(x) ≈ (x-1) - (x-1)^2/2 + (x-1)^3/3 for x close to 1
            let x_minus_1 = outcome_sum - scale;
            let ln_approx = x_minus_1 - (x_minus_1 * x_minus_1) / (2 * scale);

            let cost = (b * ln_approx) / scale;
            cost
        }

        pub fn buy_shares() {}

        pub fn sell_shares() {}
    }

    // input from the user/cleint - Share
    pub struct UserBet {
        is_yes: bool,
        amount: u64,
        user_token_pubkey: SerializedSolanaPublicKey,
    }

    pub struct CreateMarket {
        pub lmsr_b: u64,
    }

    #[instruction]
    pub fn create_market(
        mxe: Mxe,
        creater_input: Enc<Shared, CreateMarket>
    ) -> Enc<Mxe, LMSR> {
        let creater_input = creater_input.to_arcis();
        let lmsr = LMSR::init_market_data(creater_input.lmsr_b);

        mxe.from_arcis(lmsr)
    }

    // #[instruction]
    // pub fn create_market(input_ctxt: Enc<Mxe, LMSR>) -> Enc<Shared, LMSR> {
    //     todo!()
    // }

    // #[instruction]  // // This one should send back the amount of shares need to be minted and bool to reciver token acc pubkey
    // pub fn buy_shares(
    //     user_input: Enc<Shared, UserInput>,
    //     market_state: Enc<MXE, MarketState>,
    // ) -> (Enc<Shared, BuyResult>, Enc<MXE, MarketState>) {
    //     // Process user input and update market state
    // }

    // #[instruction]
    // pub fn buy_yes_shares(
    //     order: Enc<Shared, BuyOrder>, // From user (client+MPC can decrypt)
    //     mut state: Enc<Mxe, ChauMarketConfidential>, // Encrypted market state (MPC only)
    // ) -> (Enc<Mxe, ChauMarketConfidential>, Enc<Share, UserBet>) {
    //     // Still confidential after update
    //     let user_order = order.to_arcis(); // Only cluster sees plaintext
    //     let mut market = state.to_arcis();

    //     // Business logic (LSMR math, update yes_shares etc.)...
    //     market.outcome_yes_shares += user_order.shares_amount;
    //     // Consider validating max_payment, etc. confidentially here

    //     state.owner.from_arcis(market) // Re-confidentialize; stays MXE
    // }

    //   let args = vec![
    //     Argument::ArcisPubkey(user_pubkey),
    //     Argument::PlaintextU128(nonce),
    //     Argument::EncryptedU64(ciphertext_amount), // for shares_amount
    //     Argument::EncryptedU64(ciphertext_payment), // for max_payment
    //     // ...etc.
    //   ];
}
