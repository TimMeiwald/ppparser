use crate::Cache;

// This cache will completely flatten the cache to see if that improves performance.
pub struct DenyLeftRecursionCache {
    is_true: Vec<bool>, // Position encoded as start_position*src_length + struct_position // May be slower due to arithmetic who knows
    end_position: Vec<u32>,
    left_recursion: Vec<bool>,
    number_of_structs: u32,
}

impl Cache for DenyLeftRecursionCache {
    // Try as flat packed data structure. Since using zero to fill didn't seem to make much difference.
    fn new(size_of_source: u32, number_of_structs: u32) -> DenyLeftRecursionCache {
        let capacity = (size_of_source + 1) * (number_of_structs + 1);
        let capacity = capacity as usize;
        let mut c = DenyLeftRecursionCache {
            is_true: Vec::with_capacity(capacity),
            end_position: Vec::with_capacity(capacity),
            left_recursion: Vec::with_capacity(capacity),
            number_of_structs,
        };
        for _i in 0..capacity {
            // Ensures the Vector in Cache is as large as the input source
            c.is_true.push(false);
            c.end_position.push(0);
            c.left_recursion.push(true);
        }
        c
        // for every arg cache in c set size to <number_of_structs>
    }

    fn push(&mut self, rule: u32, is_true: bool, start_position: u32, end_position: u32) {
        let index = (start_position * self.number_of_structs + rule) as usize;
        self.is_true[index] = is_true;
        self.end_position[index] = end_position;
        self.left_recursion[index] = false
    }
    fn check(&mut self, rule: u32, start_position: u32) -> Option<(bool, u32)> {
        let index = (start_position * self.number_of_structs + rule) as usize;
        //println!("Index: {:?}, Start_Position: {:?}, Rule: {:?}", index, start_position, rule);
        let is_true: bool = self.is_true[index];
        let end_position: u32 = self.end_position[index];

        if end_position != 0 { // If 0 it's not been used yet(doesn't make sense to have an end position at 0 hence why it works.)
            //To prevent left recursion from infinitely looping test whether the left recursion flag has been detected. 

            // Result is returned to callee to unwrap
            Some((is_true, end_position))
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
