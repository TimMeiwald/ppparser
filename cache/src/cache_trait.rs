use rules::{Key, Rules};

pub trait Cache {
    fn new(size_of_source: u32, number_of_structs: u32) -> Self;
    fn push(
        &mut self,
        rule: Rules,
        is_true: bool,
        start_position: u32,
        end_position: u32,
        reference: Key,
    );
    fn push_deny_LR(
        &mut self,
        rule: Rules,
        is_true: Option<bool>,
        start_position: u32,
        end_position: u32,
        reference: Key,
    );
    fn check_LR(&self, rule: Rules, start_position: u32) -> Option<(Option<bool>, u32, Key)>;
    fn set_lr_detected(&mut self, detected: Option<Rules>);
    fn get_lr_detected(&self, rule: Rules) -> bool;
    fn check(&self, rule: Rules, start_position: u32) -> Option<(bool, u32, Key)>;
    fn clear(&mut self);
    fn reinitialize(&mut self); //Reset state without deallocating memory for reuse.
    fn last_node(&self) -> Option<Key>;
    fn set_last_node(&mut self, key: Option<Key>);
}
