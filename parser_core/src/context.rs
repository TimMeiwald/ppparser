use std::{cell::RefCell, rc::Rc};

use cache::{Cache, BTreeCache};

pub struct Context{
    pub cache: Rc<RefCell<BTreeCache>>
}
impl Context{

    pub fn new(size_of_source: u32, number_of_structs:u32) -> Self {
        let cache = Rc::new(RefCell::new(BTreeCache::new(0,0)));
        Context{cache}
    }
}