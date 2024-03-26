
use rules::{Rules, Key};

pub trait Cache {
    fn new(size_of_source: u32, number_of_structs: u32) -> Self;
    fn push(&mut self, rule: Rules, is_true: bool, start_position: u32, end_position: u32, reference: Key);
    fn check(&self, rule: Rules, start_position: u32) -> Option<(bool, u32, Key)>;
    fn clear(&mut self);
    fn reinitialize(&mut self); //Reset state without deallocating memory for reuse.
    fn last_node(&self) -> Option<Key>;
    fn set_last_node(&mut self, key: Option<Key>);
}
