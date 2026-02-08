# cargo run -- -s ./examples/example_1/example_1.dsl -t ./examples/ -n example_1_parser --offline
# cargo run -- -s ./examples/example_2/example_2.dsl -t ./examples/ -n example_2_parser --offline
# cargo run -- -s ./examples/example_3_basic_maths/example_3_basic_maths.dsl -t ./examples/ -n example_3_basic_maths_parser --offline
# cargo run -- -s ./examples/example_4_full_maths/example_4_full_maths.dsl -t ./examples/ -n example_4_full_maths_parser --offline
# cargo run -- -s ./examples/example_5_lr_num/LR_num.dsl -t ./examples/ -n example_5_lr_num_parser --offline
# cargo run -- -s ./examples/example_6_term_and_fact/term_and_fact.dsl -t ./examples/ -n example_6_term_and_fact_parser --offline
# cargo run -- -s ./examples/example_7_indirect_lr_num/LR_num_indirect.dsl -t ./examples/ -n example_7_indirect_lr_num_parser --offline
# cargo run -- -s ./examples/example_8_indirect_lr_3_level/LR_num_indirect_3_level.dsl -t ./examples/ -n example_8_indirect_lr_3_level_parser --offline
# cargo run -- -s ./examples/example_9_indirect_lr_5_level/LR_num_indirect_5_level.dsl -t ./examples/ -n example_9_indirect_lr_5_level_parser --offline
# cargo run -- -s ./examples/example_10_basic_bench/example_10_basic_bench.dsl -t ./examples/ -n example_10_basic_bench_parser --offline
# cargo run -- -s ./examples/example_11_external_rule_call/example_11_external_rule_call.dsl -t ./examples/ -n example_11_external_rule_call_parser --offline
cargo run -- -s ./examples/c_parser/c_from_spec.dsl -t ./examples/ -n c_parser --offline # offline because we do not need to update cargo on regenerating.
cargo fmt