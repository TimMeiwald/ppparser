use crate::{cache_trait::Index, Cache};
use rules::rules::Rules;
// This cache will completely flatten the cache to see if that improves performance.
pub struct MyCache4 {
    end_position: Vec<u32>,
    indexes: Vec<Index>,
    is_true: Vec<bool>, // Position encoded as start_position*src_length + struct_position // May be slower due to arithmetic who knows
    number_of_structs: u32,
}

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
        };
        for _i in 0..capacity {
            // Ensures the Vector in Cache is as large as the input source
            c.is_true.push(false);
        }
        for _i in 0..capacity {
            c.end_position.push(0); // Might be faster? Maybe compiler inlines it? Can't exactly tell needs better performance profiling vs one loop
        }
        for _i in 0..capacity {
            c.indexes.push(Index(u32::MAX)); // MAX_INT is not valid, means not yet used. 
        }
        c
        // for every arg cache in c set size to <number_of_structs>
    }

    fn push(&mut self, rule: Rules, is_true: bool, start_position: u32, end_position: u32, stack_index: Index ) {
        let index = (start_position * self.number_of_structs + (rule as u32)) as usize;
        self.is_true[index] = is_true;
        self.end_position[index] = end_position;
        self.indexes[index] = stack_index;
    }
    fn check(&self, rule: Rules, start_position: u32) -> Option<(bool, u32, Index)> {
        let index = (start_position * self.number_of_structs + (rule as u32)) as usize;
        //println!("Index: {:?}, Start_Position: {:?}, Rule: {:?}", index, start_position, rule);
        let is_true: bool = self.is_true[index];
        let end_position: u32 = self.end_position[index];
        let indexed: Index = self.indexes[index];
        if end_position != 0 {
            // Result is returned to callee to unwrap
            Some((is_true, end_position, indexed))
        } else {
            // Tells callee to simply run the actual code instead of using cached value since one does not exist.
            None
        }
    }
    fn clear(&mut self) {}
    fn reinitialize(&mut self) {
        self.end_position.fill(0);
    }
}
