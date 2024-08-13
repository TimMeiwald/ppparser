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
pub trait Publisher {
    fn new(size_of_source: usize, number_of_rules: usize) -> Self;
    fn get_node_mut(&mut self, index: Key) -> &mut Node;
    fn add_node(&mut self, node: Node) -> Key;

    fn connect(&mut self, parent_index: Key, child_index: Key);

    fn get_node(&self, index: Key) -> &Node;
    fn get_mut_node(&mut self, index: Key) -> &mut Node;

    fn create_node(
        &self,
        rule: Rules,
        start_position: u32,
        end_position: u32,
        parent: Option<Key>,
        result: bool,
    ) -> Node;
    fn set_node_start_position(&mut self, index: Key, start_position: u32);
    fn set_node_end_position(&mut self, index: Key, end_position: u32);
    fn set_node_result(&mut self, index: Key, result: bool);
    fn print(&self, index: Key, boolean: Option<bool>);
    fn clear_false(&self) -> Tree;
}
