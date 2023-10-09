use std::{cell::RefCell, rc::Rc};

use cache::Cache;

pub struct Context<T: Cache>{
    pub cache: Rc<RefCell<T>>
}