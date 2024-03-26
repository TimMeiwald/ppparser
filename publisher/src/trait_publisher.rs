// pub trait Publisher {
//     fn new(size_of_source: u32, number_of_rules: u32) -> Self;
//     fn push(&mut self, is_true: bool, rule: u32, start_position: u32, end_position: u32) -> u32; // Returns Index
//     fn patch(&mut self, index: u32, is_true: bool, rule: u32, start_position: u32, end_position: u32);
//     fn pop(&mut self);
//     fn pop_to(&mut self, index: u32); // Not inclusive
//     // Read all children and subchildren of index and return start index and end index.
//     fn read_children(&self, index: u32) -> Option<(u32, u32)>;
//     fn get(&self, index: u32) -> [u32; 3];
// }
use rules::rules::Rules;
use crate::Tree;
use std::rc::Rc;
pub trait Publisher {
    fn new(size_of_source: u32, number_of_rules: u32) -> Self;
    fn push(&mut self, is_true: bool, rule: u32, start_position: u32, end_position: u32) -> u32; // Returns Index
    fn patch(&mut self, index: u32, is_true: bool, rule: u32, start_position: u32, end_position: u32);
    fn pop(&mut self);
    fn pop_to(&mut self, index: u32); // Not inclusive
    // Read all children and subchildren of index and return start index and end index.
    fn read_children(&self, index: u32) -> Option<(u32, u32)>;
    fn get(&self, index: u32) -> [u32; 3];
}