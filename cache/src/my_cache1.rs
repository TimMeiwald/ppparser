use crate::Cache;
// Can be reused with reinitialize
pub struct MyCache1 {
    entries: Vec<ArgCache>, // Start Position encoded in the indexing of the Cache
}

impl Cache for MyCache1 {
    
    fn new(size_of_source: u32, number_of_structs: u32) -> MyCache1 {
        let mut c = MyCache1 {
            entries: Vec::with_capacity(size_of_source as usize),
        };
        for i in 0..size_of_source+1 {
            // Ensures the Vector in Cache is as large as the input source
            c.entries.push(ArgCache {
                entries: Vec::with_capacity(number_of_structs as usize),
            });

            for _j in 0..number_of_structs+1 {
                // Ensures the Vector in ArgCache is as large as the number of structs(Aka possible arguments since each struct implements resolvable, which is known at parser generation time)
                c.entries[i as usize].entries.push((false, u32::MAX));
            }
        }
        
        return c;
        // for every arg cache in c set size to <number_of_structs>
    }


    fn push(&mut self, rule: u32, is_true: bool, start_position: u32, end_position: u32) {
        let arg_cache: &mut ArgCache = &mut self.entries[start_position as usize];
        arg_cache.entries[rule as usize] = (is_true, end_position);
    }
    fn check(&mut self, rule:u32, start_position: u32) -> Option<(bool, u32)> {
        let ret: (bool, u32) = self.entries[start_position as usize].entries[rule as usize];
        if ret.1 != u32::MAX {
            // Result is returned to callee to unwrap
            return Some(ret);
        } else {
            // Tells callee to simply run the actual code instead of using cached value since one does not exist.
            return None;
        };
    }
    fn clear(&mut self){
        for i in &mut self.entries{
            i.entries.clear();
        }
    }
    fn reinitialize(&mut self) {
        for i in &mut self.entries {
            for j in  0..i.entries.len(){
                i.entries[j].1 = u32::MAX
            }
        }


    }
}

// Create 1 per Position in Cache
struct ArgCache {
    entries: Vec<(bool, u32)>, // Struct type encoded in the position of the entries
    
}
