#[allow(unused_imports)] // So that I don't need to keep adding or removing whilst testing
//use cache::{BTreeCache, Cache, DenyLeftRecursionCache, MyCache1, MyCache2, MyCache3, MyCache4};
use cache::{Cache, MyCache4};
use publisher::Publisher;
use rules::{Key, Rules};
use std::cell::UnsafeCell;
use std::{cell::RefCell, marker::PhantomData};
pub struct Context<T: Cache, S: Publisher> {
    pub cache: UnsafeCell<T>,
    pub stack: UnsafeCell<S>,
    phantom: PhantomData<S>,
}
impl<T: Cache, S: Publisher> Context<T, S> {
    pub fn new(size_of_source: u32, number_of_structs: u32) -> Self {
        let cache: UnsafeCell<T> = UnsafeCell::new(T::new(size_of_source, number_of_structs));
        let mut stack: UnsafeCell<S> =
            UnsafeCell::new(S::new(size_of_source as usize, number_of_structs as usize));
        let phantom = PhantomData::<S>;
        let root_node = stack.get_mut();
        root_node.add_node(Rules::Grammar, 0, size_of_source, None, true);
        stack.get_mut().set_last_node(Some(Key(0)));
        Context {
            cache,
            stack,
            phantom,
        }
    }
    pub fn clear_cache(&self) {
        unsafe {
            let res = self.cache.get().as_mut().unwrap();
            //res.clear();
            res.reinitialize();
            let tree = self.stack.get().as_mut().unwrap().clear_tree();
        }
    }
}
