#[allow(unused_imports)] // So that I don't need to keep adding or removing whilst testing
//use cache::{BTreeCache, Cache, DenyLeftRecursionCache, MyCache1, MyCache2, MyCache3, MyCache4};
use cache::Cache;
use publisher::Publisher;
use rules::{Key, Rules};
use std::cell::UnsafeCell;
use std::{cell::RefCell, marker::PhantomData};
pub struct Context<T: Cache, S: Publisher> {
    pub cache: RefCell<T>,
    pub stack: RefCell<S>,
    phantom: PhantomData<S>,
}
impl<T: Cache, S: Publisher> Context<T, S> {
    pub fn new(size_of_source: u32, number_of_structs: u32) -> Self {
        let cache: RefCell<T> = RefCell::new(T::new(size_of_source, number_of_structs));
        let mut stack: RefCell<S> =
            RefCell::new(S::new(size_of_source as usize, number_of_structs as usize));
        let phantom = PhantomData::<S>;
        let root_node = stack.get_mut();
        root_node.add_node(Rules::Grammar, 0, size_of_source, true);
        stack.get_mut().set_last_node(Some(Key(0)));
        Context {
            cache,
            stack,
            phantom,
        }
    }
    pub fn clear_cache(&self) {
        let mut res = self.cache.borrow_mut();
        res.clear();
        res.reinitialize();
        let _tree = self.stack.borrow_mut().clear_tree();
    }
}
