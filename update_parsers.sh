cargo run -- -s ./examples/example_1/example_1.dsl -t ./examples/ -n example_1_parser
cargo run -- -s ./examples/example_2/example_2.dsl -t ./examples/ -n example_2_parser
cargo run -- -s ./examples/example_3_basic_maths/example_3_basic_maths.dsl -t ./examples/ -n example_3_basic_maths_parser
cargo run -- -s ./examples/example_4_full_maths/example_4_full_maths.dsl -t ./examples/ -n example_4_full_maths_parser
cargo run -- -s ./examples/example_5_LR_num/LR_num.dsl -t ./examples/ -n example_5_LR_num_parser

cargo fmt