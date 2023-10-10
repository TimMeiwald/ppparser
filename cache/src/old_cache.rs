
// One per Parse
pub struct Cache {
    entries: Vec<ArgCache>, // Start Position encoded in the indexing of the Cache
}

impl Cache {
    
    pub fn new(size_of_source: i32, number_of_structs: i32) -> Self {
        let mut c = Cache {
            entries: Vec::with_capacity(size_of_source as usize),
        };
        for i in 0..size_of_source {
            // Ensures the Vector in Cache is as large as the input source
            c.entries.push(ArgCache {
                entries: Vec::with_capacity(number_of_structs as usize),
            });
            for _j in 1..number_of_structs {
                // Ensures the Vector in ArgCache is as large as the number of structs(Aka possible arguments since each struct implements resolvable, which is known at parser generation time)
                c.entries[i as usize].entries.push((false, i32::MAX));
            }
        }
        return c;
        // for every arg cache in c set size to <number_of_structs>
    }

    fn push(&mut self, position: i32, arg_key: i32, bool: bool, end_position: i32) {
        let arg_cache: &mut ArgCache = &mut self.entries[position as usize];
        arg_cache.entries[arg_key as usize] = (bool, end_position);
    }
    fn check(&self, position: i32, arg_key: i32) -> Option<(bool, i32)> {
        let ret: (bool, i32) = self.entries[position as usize].entries[arg_key as usize];
        if ret.1 != i32::MAX {
            // Result is returned to callee to unwrap
            return Some(ret);
        } else {
            // Tells callee to simply run the actual code instead of using cached value since one does not exist.
            return None;
        };
    }
}

// Create 1 per Position in Cache
pub struct ArgCache {
    entries: Vec<(bool, i32)>, // Struct type encoded in the position of the entries
    
}
