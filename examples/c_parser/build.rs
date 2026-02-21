fn main() {
    // Rebuild if any of the test files changed.
    println!("cargo:rerun-if-changed=tests/example_files/");

}