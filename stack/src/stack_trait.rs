pub trait Stack {
    fn new(size_of_source: u32, number_of_rules: u32) -> Self;
    fn push(&mut self, is_true: bool, rule: u32, start_position: u32, end_position: u32);
    fn remove(&mut self, index: u32);
}
