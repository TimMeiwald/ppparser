use crate::Stack;

// Stack that is as the name suggests a printerto stdout, useful for seeing output.
pub struct PrinterStack {}

impl Stack for PrinterStack {
    fn new(_size_of_source: u32, _number_of_rules: u32) -> Self {
        PrinterStack {}
    }
    fn push(&mut self, is_true: bool, rule: u32, start_position: u32, end_position: u32) -> u32 {
        println!(
            "Push Rule: {}, Start Position: {}, End Position: {} -> {}",
            rule, start_position, end_position, is_true
        );
        return 0
    }
    fn get(&self, _index: u32) -> [u32; 3] {[0,0,0]}

    fn patch(&mut self, _index: u32, _is_true: bool, _rule: u32, _start_position: u32, _end_position: u32) {
        println!("Patch")
    }

    fn pop(&mut self){}
    fn pop_to(&mut self, _index: u32){}
    fn read_children(&self, _index: u32) -> Option<(u32, u32)>{Some((0,0))}

}
