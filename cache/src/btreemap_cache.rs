
use std::collections::BTreeMap;
use crate::Cache;


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
