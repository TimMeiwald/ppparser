
use rules::rules::Rules;
#[derive(Copy, Clone)]
pub struct Key<T>(pub T);
// Whatever method the publisher uses to refer to other locations in the tree/stack or whatever is being used for output
// Up to the publisher. 

impl From<Key<u32>> for u32 {
    fn from(i: Key<u32>) -> u32 {
        i.0
    }
}
pub trait Cache<T> {
    fn new(size_of_source: u32, number_of_structs: u32) -> Self;
    fn push(&mut self, rule: Rules, is_true: bool, start_position: u32, end_position: u32, reference: Key<T>);
    fn check(&self, rule: Rules, start_position: u32) -> Option<(bool, u32, Key<T>)>;
    fn clear(&mut self);
    fn reinitialize(&mut self); //Reset state without deallocating memory for reuse.
}
