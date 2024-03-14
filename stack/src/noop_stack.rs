use crate::Stack;

// Stack that is as the name suggests a Noop, useful when you only care about whether the parse was successful or not but don't care about the AST.
pub struct NoopStack {}

impl Stack for NoopStack {
    fn new(_size_of_source: u32, _number_of_rules: u32) -> Self {
        NoopStack {}
    }
    fn push(&mut self, _is_true: bool, _rule: u32, _start_position: u32, _end_position: u32) -> u32 {0}
    fn pop(&mut self){}
    fn pop_to(&mut self, index: u32){}
    fn patch(&mut self, index: u32, is_true: bool, rule: u32, start_position: u32, end_position: u32) {
        
    }
}
