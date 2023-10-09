use std::collections::BTreeMap;
pub trait Cache{
    fn new(size_of_source: u32, number_of_structs:u32) -> Self;
    fn push(&mut self, rule: u32, is_true:bool, start_position: u32, end_position: u32);
    fn check(&mut self, rule: u32, start_position: u32) -> Option<&(bool, u32)>;
}





pub struct BTreeCache{
    cache: BTreeMap<(u32, u32), (bool, u32)>
}
impl Cache for BTreeCache{
    fn new(size_of_source: u32, number_of_structs:u32) -> Self {
        BTreeCache { cache: BTreeMap::new() }
    }

    fn push(&mut self, rule: u32, is_true:bool, start_position: u32, end_position: u32){
        self.cache.insert((rule, start_position), (is_true, end_position));
    }
    fn check(&mut self, rule: u32, start_position: u32) -> Option<&(bool, u32)> {
        self.cache.get(&(rule, start_position))
    }
}