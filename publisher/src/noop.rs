use crate::Publisher;
use crate::{Node, Tree};
use rules::{Key, Rules};
use std::rc::Rc;

pub struct NOOP {
    last_node: Option<Key>,
}
impl Publisher for NOOP {
    fn new(size_of_source: usize, number_of_rules: usize) -> Self {
        return NOOP { last_node: None };
    }
    fn add_node(
        &mut self,
        rule: Rules,
        start_position: u32,
        end_position: u32,
        parent: Option<Key>,
        result: bool,
    ) -> Key {
        Key(0)
    }
    fn connect(&mut self, parent_index: Key, child_index: Key) {}
    fn last_node(&self) -> Option<Key> {
        self.last_node
    }
    fn set_node_end_position(&mut self, index: Key, end_position: u32) {}
    fn set_node_start_position(&mut self, index: Key, start_position: u32) {}
    fn set_node_result(&mut self, index: Key, result: bool) {}
    fn set_last_node(&mut self, key: Option<Key>) {
        self.last_node = key;
    }
    fn print(&self, index: Key, boolean: Option<bool>) {}
    fn clear_false(&self) -> Tree {
        Tree::new(0, 0)
    }
    fn clear_tree(&mut self) {
        self.last_node = None;
    }
}
