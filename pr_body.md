- closes #614
- closes #613
- closes #611
- closes #610

## Refactoring and Testing Improvements

This pull request resolves multiple issues related to refactoring integration tests, optimizing performance, and increasing unit test coverage across the `amm-pool-contract` measurements suite.

**Changes made:**

1. **Refactored `measure_bytes_ops.rs` (#614):**
   - Extracted repetitive boilerplate code used to instantiate the `Env` and setup the `ConstantProductPoolClient`.
   - Introduced a modular `measure_operation` helper function which reduces code duplication and improves readability across all byte measurement tests (append, slice, concat).

2. **Optimized `measure_call_depth_gap.rs` (#613):**
   - Moved the `wasm` binary loading out of the loop to avoid redundant heavy IO operations.
   - Pre-allocated the `points` vector to reduce multiple unnecessary re-allocations.
   - Replaced a `for` loop that called `clone()` and `push_back()` continuously on the network mock with a highly optimized `SdkVec::from_slice` initialization pattern to optimize memory footprint and execution speed.

3. **Expanded Test Coverage in `calibrate_extend_ttl.rs` (#611):**
   - Added comprehensive edge-case unit tests to measure TTL extension robustness.
   - Added `test_extend_instance_ttl_invalid_threshold` and `test_extend_persistent_ttl_invalid_threshold` to verify the panic bounds when `threshold > extend_to`.

4. **Modularized `measure_auth_gap.rs` (#610):**
   - Broken down the large monolithic test function into reusable components: `setup_client` and `measure_require_auth_only`.
   - The setup logic is now isolated from the cost measurement itself, bringing the file strictly in line with cleaner code principles.

All changes have been successfully checked locally against `cargo fmt`, `cargo clippy`, and `cargo test`.
