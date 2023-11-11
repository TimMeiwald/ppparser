#[allow(unused_imports)] // So that I don't need to keep adding or removing whilst testing
use cache::{BTreeCache, Cache, MyCache1, MyCache2, MyCache3, MyCache4, DenyLeftRecursionCache};
use std::cell::RefCell;

pub struct Context<T: Cache>{
    pub cache: RefCell<T>,
}
impl<T: Cache> Context<T> {
    pub fn new(size_of_source: u32, number_of_structs: u32) -> Self {
        let cache: RefCell<T> =
            RefCell::new(T::new(size_of_source, number_of_structs));
        Context { cache }
    }
    pub fn clear_cache(&self) {
        let res = &mut *(self.cache).borrow_mut();
        //res.clear();
        res.reinitialize();
    }
}
