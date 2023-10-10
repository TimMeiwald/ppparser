use std::{cell::RefCell, rc::Rc};

use cache::{Cache, BTreeCache, MyCache1};

pub struct Context{
    pub cache: Rc<RefCell<MyCache1>>
}
impl Context{

    pub fn new(size_of_source: u32, number_of_structs:u32) -> Self {
        let cache: Rc<RefCell<MyCache1>> = Rc::new(RefCell::new(MyCache1::new(size_of_source,number_of_structs)));
        //let cache: Rc<RefCell<MyCache1>> = Rc::new(RefCell::new(BTreeCache::new(0, 0)));

        Context{cache}
    }
    pub fn clear_cache(&self){
        let res = &mut *(self.cache).borrow_mut();
        //res.clear();
        res.reinitialize();
    }

}