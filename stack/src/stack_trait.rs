pub trait Stack {
    fn new(size_of_source: u32, number_of_rules: u32) -> Self;
    fn push(&mut self, is_true: bool, rule: u32, start_position: u32, end_position: u32) -> u32; // Returns Index
    fn patch(&mut self, index: u32, is_true: bool, rule: u32, start_position: u32, end_position: u32);
    fn pop(&mut self);
    fn pop_to(&mut self, index: u32); // Not inclusive
}
