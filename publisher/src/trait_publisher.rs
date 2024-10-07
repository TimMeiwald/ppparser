//  trait Publisher {
//     fn new(size_of_source: u32, number_of_rules: u32) -> Self;
//     fn push(&mut self, is_true: bool, rule: u32, start_position: u32, end_position: u32) -> u32; // Returns Index
//     fn patch(&mut self, index: u32, is_true: bool, rule: u32, start_position: u32, end_position: u32);
//     fn pop(&mut self);
//     fn pop_to(&mut self, index: u32); // Not inclusive
//     // Read all children and subchildren of index and return start index and end index.
//     fn read_children(&self, index: u32) -> Option<(u32, u32)>;
//     fn get(&self, index: u32) -> [u32; 3];
// }
use crate::{Node, Tree};
use rules::{Key, Rules};
use std::rc::Rc;
pub trait Publisher {
    fn new(size_of_source: usize, number_of_rules: usize) -> Self;
    fn add_node(
        &mut self,
        rule: Rules,
        start_position: u32,
        end_position: u32,
        result: bool,
    ) -> Key;
    fn connect(&mut self, parent_index: Key, child_index: Key);
    fn connect_if_not_connected(&mut self, parent_index: Key, child_index: Key);
    fn disconnect(&mut self, parent_index: Key, child_index: Key);
    fn set_node_start_position(&mut self, index: Key, start_position: u32);
    fn set_node_end_position(&mut self, index: Key, end_position: u32);
    fn set_node_result(&mut self, index: Key, result: bool);
    fn print(&self, index: Key, boolean: Option<bool>);
    fn clear_false(&self) -> Tree;
    fn last_node(&self) -> Option<Key>;
    fn set_last_node(&mut self, key: Option<Key>);
    fn clear_tree(&mut self);
}
