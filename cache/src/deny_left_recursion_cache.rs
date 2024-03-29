use crate::Cache;

// This cache will completely flatten the cache to see if that improves performance.
pub struct DenyLeftRecursionCache {
    is_true: Vec<bool>, // Position encoded as start_position*src_length + struct_position // May be slower due to arithmetic who knows
    end_position: Vec<u32>,
    last_rule: Vec<u32>,
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
            last_rule: Vec::with_capacity(capacity),
            number_of_structs,
        };
        for _i in 0..capacity {
            // Ensures the Vector in Cache is as large as the input source
            c.is_true.push(false);
            c.end_position.push(0);
            c.last_rule.push(u32::MAX)
        }
        c
        // for every arg cache in c set size to <number_of_structs>
    }

    fn push(&mut self, rule: u32, is_true: bool, start_position: u32, end_position: u32) {
        let index = (start_position * self.number_of_structs + rule) as usize;
        self.is_true[index] = is_true;
        self.end_position[index] = end_position;
    }
    fn check(&self, rule: u32, start_position: u32) -> Option<(bool, u32)> {
        let index = (start_position * self.number_of_structs + rule) as usize;
        let is_true: bool = self.is_true[index];
        let last_rule = self.last_rule[index];
        let end_position: u32 = self.end_position[index];
        println!(
            "Rule: {:?}, Start Position: {:?}, is_true: {:?}, End Position: {:?}, Last Rule: {:?}",
            rule, start_position, is_true, end_position, last_rule
        );

        if last_rule == rule {
            // Is Left Recursive
            println!("LEFT RECURSION DETECTED");

            Some((false, end_position))
        } else {
            println!("HERE");

            // Not Left Recursive
            if end_position == 0 {
                // Is First time the path has been run
                println!("HERE2");

                None
            } else {
                // Cached but not left recursive
                println!("HERE3");

                Some((is_true, end_position))
            }
        }
    }
    fn clear(&mut self) {}
    fn reinitialize(&mut self) {
        self.end_position.fill(0);
    }
}
