use arcis_imports::*;
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
        pub fn init_market_data(lmsr_b: u64) -> LMSR {
            let mut lmsr_data = LMSR {
                lmsr_b,
                total_yes_shares: 0,
                total_no_shares: 0,
                initial_deposit: 0,
            };

            let initial_deposit = lmsr_data.cost_calculation(0, 0, lmsr_b);

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
    }

    // input from the user/cleint - Share
    pub struct UserBet {
        is_yes: bool,
        amount: u64,
        user_token_pubkey: SerializedSolanaPublicKey,
    }

    #[instruction]
    pub fn create_market(mxe: Mxe, creater_input: Enc<Shared, u64>) -> Enc<Mxe, LMSR> {
        let creater_input = creater_input.to_arcis();
        let lmsr = LMSR::init_market_data(creater_input);

        mxe.from_arcis(lmsr)
    }

    #[instruction]
    pub fn buy_share(
        user_buy: Enc<Shared, UserBet>,
        market_data: Enc<Mxe, LMSR> // Gives the current data of the market
    ) -> (Enc<Mxe, LMSR>, Enc<Shared, UserBet>) {
        let mut lmsr_data = market_data.to_arcis();
        let mut user_buy_data = user_buy.to_arcis();

        // for buying Delta C = C2 - C1; (C2 > C1)

        // c_one is the current Cost Price
        let c_one = lmsr_data.cost_calculation(
            lmsr_data.total_yes_shares,
            lmsr_data.total_no_shares,
            lmsr_data.lmsr_b
        );

        let mut c_two = 0;

        if user_buy_data.is_yes {
            c_two = lmsr_data.cost_calculation(
                lmsr_data.total_yes_shares + user_buy_data.amount,
                lmsr_data.total_no_shares,
                lmsr_data.lmsr_b
            );

            lmsr_data.total_yes_shares = lmsr_data.total_yes_shares + user_buy_data.amount;
        } else {
            c_two = lmsr_data.cost_calculation(
                lmsr_data.total_yes_shares,
                lmsr_data.total_no_shares + user_buy_data.amount,
                lmsr_data.lmsr_b
            );

            lmsr_data.total_no_shares = lmsr_data.total_no_shares + user_buy_data.amount;
        }

        let price_share = c_two - c_one;
        user_buy_data.amount = price_share;

        (market_data.owner.from_arcis(lmsr_data), user_buy.owner.from_arcis(user_buy_data))
    }

    #[instruction]
    pub fn sell_share(
        user_sell: Enc<Shared, UserBet>,
        market_data: Enc<Mxe, LMSR> // Gives the current data of the market
    ) -> (Enc<Mxe, LMSR>, Enc<Shared, UserBet>) {
        let mut lmsr_data = market_data.to_arcis();
        let mut user_sell_data = user_sell.to_arcis();

        // for selling Delta C = C1 - C2; (C1 > C2)

        // c_one is the current Cost Price
        let c_one = lmsr_data.cost_calculation(
            lmsr_data.total_yes_shares,
            lmsr_data.total_no_shares,
            lmsr_data.lmsr_b
        );

        let mut c_two = 0;

        if user_sell_data.is_yes {
            c_two = lmsr_data.cost_calculation(
                lmsr_data.total_yes_shares - user_sell_data.amount,
                lmsr_data.total_no_shares,
                lmsr_data.lmsr_b
            );

            lmsr_data.total_yes_shares = lmsr_data.total_yes_shares - user_sell_data.amount;
        } else {
            c_two = lmsr_data.cost_calculation(
                lmsr_data.total_yes_shares,
                lmsr_data.total_no_shares - user_sell_data.amount,
                lmsr_data.lmsr_b
            );

            lmsr_data.total_no_shares = lmsr_data.total_no_shares - user_sell_data.amount;
        }

        let price_share = c_one - c_two;
        user_sell_data.amount = price_share;

        (market_data.owner.from_arcis(lmsr_data), user_sell.owner.from_arcis(user_sell_data))
    }
}
