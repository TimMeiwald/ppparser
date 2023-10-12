use crate::Cache;

// Can be reused with reinitialize
// This one is modified to use 0 as the unset value in the hopes that fill is faster with zero than some value.
pub struct MyCache3 {
    entries: Vec<ArgCache3>, // Start Position encoded in the indexing of the Cache
}

impl Cache for MyCache3 {
    // Try as flat packed data structure. Since using zero to fill didn't seem to make much difference. 
    fn new(size_of_source: u32, number_of_structs: u32) -> MyCache3 {
        let mut c = MyCache3 {
            entries: Vec::with_capacity(size_of_source as usize),
        };
        for i in 0..size_of_source+1 {
            // Ensures the Vector in Cache is as large as the input source
            c.entries.push(ArgCache3 {
                is_true: Vec::with_capacity(number_of_structs as usize), 
                end_position: Vec::with_capacity(number_of_structs as usize)
            });

            for _j in 0..number_of_structs+1 {
                // Ensures the Vector in ArgCache is as large as the number of structs(Aka possible arguments since each struct implements resolvable, which is known at parser generation time)
                c.entries[i as usize].is_true.push(false);

            }
            for _j in 0..number_of_structs+1 {
                // Ensures the Vector in ArgCache is as large as the number of structs(Aka possible arguments since each struct implements resolvable, which is known at parser generation time)
                c.entries[i as usize].end_position.push(0);

            }
        }
        
        return c;
        // for every arg cache in c set size to <number_of_structs>
    }


    fn push(&mut self, rule: u32, is_true: bool, start_position: u32, end_position: u32) {
        let arg_cache: &mut ArgCache3 = &mut self.entries[start_position as usize];
        arg_cache.is_true[rule as usize] = is_true;
        arg_cache.end_position[rule as usize] = end_position;

    }
    fn check(&mut self, rule:u32, start_position: u32) -> Option<(bool, u32)> {
        let is_true: bool= self.entries[start_position as usize].is_true[rule as usize];
        let end_position: u32= self.entries[start_position as usize].end_position[rule as usize];

        if end_position != 0 {
            // Result is returned to callee to unwrap
            return Some((is_true, end_position));
        } else {
            // Tells callee to simply run the actual code instead of using cached value since one does not exist.
            return None;
        };
    }
    fn clear(&mut self){
        for i in &mut self.entries{
            i.is_true.clear();
            i.end_position.clear();
        }
    }
    fn reinitialize(&mut self) {
        for i in &mut self.entries {
            // for j in  0..i.is_true.len(){
            //     i.end_position[j] = u32::MAX
            // }
            i.end_position.fill(0)
        }


    }
}

// Create 1 per Position in Cache
struct ArgCache3 {
    is_true: Vec<bool>, // Struct type encoded in the position of the entries
    end_position: Vec<u32>,
}
