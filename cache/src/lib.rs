use core::num;
use std::collections::BTreeMap;
pub trait Cache{
    fn new(size_of_source: u32, number_of_structs:u32) -> Self;
    fn push(&mut self, rule: u32, is_true: bool, start_position: u32, end_position: u32);
    fn check(&mut self, rule: u32, start_position: u32) -> Option<(bool, u32)>;
    fn clear(&mut self);
    fn reinitialize(&mut self); //Reset state without deallocating memory for reuse.
}





pub struct BTreeCache{
    cache: BTreeMap<(u32, u32), (bool, u32)>
}
impl Cache for BTreeCache{
    fn new(_size_of_source: u32, _number_of_structs:u32) -> Self {
        BTreeCache { cache: BTreeMap::new() }
    }

    fn push(&mut self, rule: u32, is_true:bool, start_position: u32, end_position: u32){
        self.cache.insert((rule, start_position), (is_true, end_position));
    }
    fn check(&mut self, rule: u32, start_position: u32) -> Option<(bool, u32)> {
        let result = self.cache.get(&(rule, start_position));
        match result{
            Some(result) =>{
                let result = *result;
                return Some(result);
            },
            None => {None}
        }

    }
    fn clear(&mut self){
        self.cache.clear();
    }
    fn reinitialize(&mut self) {
        //Same as clear for BTreeMap 
        self.clear()
    }
}


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
pub struct ArgCache {
    entries: Vec<(bool, u32)>, // Struct type encoded in the position of the entries
    
}


// Can be reused with reinitialize
pub struct MyCache2 {
    entries: Vec<ArgCache2>, // Start Position encoded in the indexing of the Cache
}

impl Cache for MyCache2 {
    
    fn new(size_of_source: u32, number_of_structs: u32) -> MyCache2 {
        let mut c = MyCache2 {
            entries: Vec::with_capacity(size_of_source as usize),
        };
        for i in 0..size_of_source+1 {
            // Ensures the Vector in Cache is as large as the input source
            c.entries.push(ArgCache2 {
                is_true: Vec::with_capacity(number_of_structs as usize), 
                end_position: Vec::with_capacity(number_of_structs as usize)
            });

            for _j in 0..number_of_structs+1 {
                // Ensures the Vector in ArgCache is as large as the number of structs(Aka possible arguments since each struct implements resolvable, which is known at parser generation time)
                c.entries[i as usize].is_true.push(false);

            }
            for _j in 0..number_of_structs+1 {
                // Ensures the Vector in ArgCache is as large as the number of structs(Aka possible arguments since each struct implements resolvable, which is known at parser generation time)
                c.entries[i as usize].end_position.push(u32::MAX);

            }
        }
        
        return c;
        // for every arg cache in c set size to <number_of_structs>
    }


    fn push(&mut self, rule: u32, is_true: bool, start_position: u32, end_position: u32) {
        let arg_cache: &mut ArgCache2 = &mut self.entries[start_position as usize];
        arg_cache.is_true[rule as usize] = is_true;
        arg_cache.end_position[rule as usize] = end_position;

    }
    fn check(&mut self, rule:u32, start_position: u32) -> Option<(bool, u32)> {
        let is_true: bool= self.entries[start_position as usize].is_true[rule as usize];
        let end_position: u32= self.entries[start_position as usize].end_position[rule as usize];

        if end_position != u32::MAX {
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
            i.end_position.fill_with(||{u32::MAX})
        }


    }
}

// Create 1 per Position in Cache
pub struct ArgCache2 {
    is_true: Vec<bool>, // Struct type encoded in the position of the entries
    end_position: Vec<u32>,
}



// Can be reused with reinitialize
// This one is modified to use 0 as the unset value in the hopes that fill is faster with zero than some value.
pub struct MyCache3 {
    entries: Vec<ArgCache3>, // Start Position encoded in the indexing of the Cache
}

impl Cache for MyCache3 {
    
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
            i.end_position.fill_with(||{0})
        }


    }
}

// Create 1 per Position in Cache
pub struct ArgCache3 {
    is_true: Vec<bool>, // Struct type encoded in the position of the entries
    end_position: Vec<u32>,
}
