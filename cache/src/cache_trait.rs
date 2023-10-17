use crate::{Context, Source};
pub trait Cache {
    fn new(size_of_source: u32, number_of_structs: u32) -> Self;
    fn push(&mut self, rule: u32, is_true: bool, start_position: u32, end_position: u32);
    fn check(
        self,
        rule: u32,
        position: u32,
        func: &dyn Fn() -> (bool, u32),
    ) -> (bool, u32);
    fn clear(&mut self);
    fn reinitialize(&mut self); //Reset state without deallocating memory for reuse.
}
