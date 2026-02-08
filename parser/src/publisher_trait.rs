use crate::{Key, Node};


pub trait Publisher{
    fn get_node(&self, index: Key) -> &Node;
    fn get_mut_node(&mut self, index: Key) -> &mut Node;
}