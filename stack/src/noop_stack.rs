use crate::Stack;

// Stack that is as the name suggests a Noop, useful when you only care about whether the parse was successful or not but don't care about the AST.
pub struct NoopStack {}

impl Stack for NoopStack {
    fn new(_size_of_source: u32, _number_of_rules: u32) -> Self {
        NoopStack {}
    }
    fn push(
        &mut self,
        _is_true: bool,
        _rule: u32,
        _start_position: u32,
        _end_position: u32,
    ) -> u32 {
        0
    }
    fn get(&self, _index: u32) -> [u32; 3] {[0,0,0]}
    fn pop(&mut self) {}
    fn pop_to(&mut self, _index: u32) {}
    fn patch(
        &mut self,
        _index: u32,
        _is_true: bool,
        _rule: u32,
        _start_position: u32,
        _end_position: u32,
    ) {
    }
    fn read_children(&self, _index: u32) -> Option<(u32, u32)> {
        Some((0, 0))
    }
}
