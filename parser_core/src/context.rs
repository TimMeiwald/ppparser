use std::{cell::RefCell, rc::Rc};

use cache::{Cache, BTreeCache};

pub struct Context{
    pub cache: Rc<RefCell<BTreeCache>>
}
