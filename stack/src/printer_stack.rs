use crate::Stack;

// Stack that is as the name suggests a printerto stdout, useful for seeing output.
pub struct PrinterStack {}

impl Stack for PrinterStack {
    fn new(_size_of_source: u32, _number_of_rules: u32) -> Self {
        PrinterStack {}
    }
    fn push(&mut self, is_true: bool, rule: u32, start_position: u32, end_position: u32) {
        println!(
            "Push Rule: {}, Start Position: {}, End Position: {} -> {}",
            rule, start_position, end_position, is_true
        )
    }
    fn remove(&mut self, index: u32) {
        println!("Remove Index: {}", index)
    }
}
