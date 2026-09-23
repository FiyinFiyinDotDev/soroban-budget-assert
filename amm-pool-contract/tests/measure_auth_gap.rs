// @measure local  # discovered by scripts/regenerate-measurements.sh
#![cfg(not(feature = "sdk20"))]

//! Calibration test for `require_auth()` cost gap measurement.
//!
//! Measures the local CPU and memory cost estimates for a single `require_auth()`
//! call on a generated address, establishing a local-vs-network gap for
//! authorization operations.
//!
//! # Usage
//!
//! This test is marked `#[ignore]` and excluded from the default test suite.
//! To run it deliberately:
//!
//! ```bash
//! cargo build --target wasm32v1-none --release -p amm-pool-contract
//! cargo test -p amm-pool-contract --test measure_auth_gap -- --ignored --nocapture
//! ```

mod common;

#[cfg(test)]
mod measure_auth_gap {
    use super::common;
    use amm_pool_contract::ConstantProductPoolClient;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    fn setup_client<'a>(env: &Env) -> ConstantProductPoolClient<'a> {
        let wasm = common::load_contract_wasm("wasm32v1-none");
        let contract_id = env.register(wasm.as_slice(), ());
        ConstantProductPoolClient::new(env, &contract_id)
    }

    fn measure_require_auth_only(env: &Env, client: &ConstantProductPoolClient) {
        let user = Address::generate(env);

        env.mock_all_auths();
        env.cost_estimate().budget().reset_unlimited();

        client.require_auth_only(&user);

        let budget = env.cost_estimate().budget();
        let cpu = budget.cpu_instruction_cost();
        let mem = budget.memory_bytes_cost();

        println!("=== REQUIRE_AUTH_MEASUREMENT ===");
        println!("AUTH_CPU={}", cpu);
        println!("AUTH_MEM={}", mem);
    }

    #[test]
    #[ignore]
    fn measure_auth_gap() {
        let env = Env::default();
        let client = setup_client(&env);
        measure_require_auth_only(&env, &client);
    }
}
