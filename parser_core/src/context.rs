#[allow(unused_imports)] // So that I don't need to keep adding or removing whilst testing
//use cache::{BTreeCache, Cache, DenyLeftRecursionCache, MyCache1, MyCache2, MyCache3, MyCache4};
use cache::{MyCache4, Cache};
use publisher::Publisher;
use std::cell::RefCell;
pub struct Context<T: Cache, S: Publisher> {
    pub cache: RefCell<T>,
    pub stack: RefCell<S>,
}
impl<T: Cache, S: Publisher> Context<T, S> {
    pub fn new(size_of_source: u32, number_of_structs: u32) -> Self {
        let cache: RefCell<T> = RefCell::new(T::new(size_of_source, number_of_structs));
        let stack: RefCell<S> = RefCell::new(S::new(size_of_source, number_of_structs));
        Context { cache, stack }
    }
    pub fn clear_cache(&self) {
        let res = &mut *(self.cache).borrow_mut();
        //res.clear();
        res.reinitialize();
    }
}
