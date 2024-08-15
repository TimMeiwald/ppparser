use crate::Cache;
use rules::{Key, Rules};

// This cache will completely flatten the cache to see if that improves performance.
pub struct MyCache4 {
    end_position: Vec<u32>,
    indexes: Vec<Key>,
    is_true: Vec<bool>, // Position encoded as start_position*src_length + struct_position // May be slower due to arithmetic who knows
    number_of_structs: u32,
    last_node: Option<Key>,
}
// TODO: Last Node should probably be in the publisher not in Cache. Irrelevant to caching per se.
impl Cache for MyCache4 {
    // Try as flat packed data structure. Since using zero to fill didn't seem to make much difference.
    fn new(size_of_source: u32, number_of_structs: u32) -> MyCache4 {
        let capacity = (size_of_source + 1) * (number_of_structs + 1);
        let capacity = capacity as usize;
        let mut c = MyCache4 {
            is_true: Vec::with_capacity(capacity),
            end_position: Vec::with_capacity(capacity),
            indexes: Vec::with_capacity(capacity),
            number_of_structs,
            last_node: None,
        };
        c.is_true.resize(capacity, false);
        c.end_position.resize(capacity, 0);
        c.indexes.resize(capacity, Key(u32::MAX));
        c
        // for every arg cache in c set size to <number_of_structs>
    }
    fn last_node(&self) -> Option<Key> {
        self.last_node
    }
    fn set_last_node(&mut self, key: Option<Key>) {
        self.last_node = key
    }
    fn push_deny_LR(
        &mut self,
        rule: Rules,
        is_true: Option<bool>,
        start_position: u32,
        end_position: u32,
        reference: Key,
    ) {
        // Noop
    }

    fn push(
        &mut self,
        rule: Rules,
        is_true: bool,
        start_position: u32,
        end_position: u32,
        stack_index: Key,
    ) {
        let index = (start_position * self.number_of_structs + (rule as u32)) as usize;
        self.is_true[index] = is_true;
        self.end_position[index] = end_position;
        self.indexes[index] = stack_index;
    }
    fn check(&self, rule: Rules, start_position: u32) -> Option<(bool, u32, Key)> {
        let index = (start_position * self.number_of_structs + (rule as u32)) as usize;
        //println!("Index: {:?}, Start_Position: {:?}, Rule: {:?}", index, start_position, rule);
        let is_true: bool = self.is_true[index];
        let end_position: u32 = self.end_position[index];
        let indexed: Key = self.indexes[index];
        if end_position != 0 {
            // Result is returned to callee to unwrap
            Some((is_true, end_position, indexed))
        } else {
            // Tells callee to simply run the actual code instead of using cached value since one does not exist.
            None
        }
    }
    fn check_LR(&self, rule: Rules, start_position: u32) -> Option<(Option<bool>, u32, Key)> {
        panic!("This cache requires the use of _var_name function!")
    }
    fn get_lr_detected(&self, rule: Rules) -> bool {
        panic!("");
    }
    fn set_lr_detected(&mut self, detected: Option<Rules>) {
        panic!("");
    }
    fn clear(&mut self) {}
    fn reinitialize(&mut self) {
        self.end_position.fill(0);
    }
}
