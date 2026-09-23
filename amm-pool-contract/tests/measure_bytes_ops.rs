// @measure local  # discovered by scripts/regenerate-measurements.sh
#![cfg(test)]

//! Local WASM measurement for the bytes-operations gap series in
//! `MEASUREMENTS.md` ("## Bytes operations"). Measures `bytes_append_bench`,
//! `bytes_slice_bench`, and `bytes_concat_bench` at several buffer sizes so
//! the scaling shape (linear vs. quadratic) of each operation can be
//! reported alongside the local-vs-network gap.
//!
//! # Running
//!
//! ```bash
//! cargo build --target wasm32v1-none --release -p amm-pool-contract
//! cargo test -p amm-pool-contract --test measure_bytes_ops -- --nocapture
//! ```

mod common;

use amm_pool_contract::ConstantProductPoolClient;
use soroban_sdk::Env;

const SIZES: [u32; 4] = [256, 1_024, 4_096, 16_384];

fn measure_operation<F>(op_name: &str, mut op: F)
where
    F: FnMut(&ConstantProductPoolClient, u32),
{
    let wasm = common::load_contract_wasm("wasm32v1-none");
    println!("=== {}_WASM_LOCAL ===", op_name);
    for &n in &SIZES {
        let env = Env::default();
        let contract_id = env.register(wasm.as_slice(), ());
        let client = ConstantProductPoolClient::new(&env, &contract_id);
        env.cost_estimate().budget().reset_unlimited();
        env.cost_estimate().disable_resource_limits();

        op(&client, n);

        let budget = env.cost_estimate().budget();
        println!(
            "n={:>6} CPU_INSTRUCTIONS={:>10} MEMORY_BYTES={:>10}",
            n,
            budget.cpu_instruction_cost(),
            budget.memory_bytes_cost()
        );
    }
}

#[test]
fn measure_bytes_append() {
    measure_operation("BYTES_APPEND", |client, n| {
        client.bytes_append_bench(&n);
    });
}

#[test]
fn measure_bytes_slice() {
    measure_operation("BYTES_SLICE", |client, n| {
        client.bytes_slice_bench(&n);
    });
}

#[test]
fn measure_bytes_concat() {
    measure_operation("BYTES_CONCAT", |client, n| {
        client.bytes_concat_bench(&n);
    });
}
