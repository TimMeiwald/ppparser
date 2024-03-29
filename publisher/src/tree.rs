use crate::Publisher;
use rules::rules::Rules;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::Index;
use std::ops::IndexMut;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use rules::Key;
// Tree needs to be able to swap existing structures into other locations
// E.g Rule A may fail but calls subrule B where B succeeds
// Then Rule C may try on same start location and also need subrule B
// So rather than redo subrule B we swap in the existing cached subrule B value.

pub struct Tree(Vec<Node>);

impl Publisher for Tree {
    fn new(size_of_source: usize, number_of_rules: usize) -> Self {
        Tree {
            0: Vec::<Node>::with_capacity(4 * size_of_source * number_of_rules),
        }
    }
    fn add_node(&mut self, node: Node) -> Key {
        let mut node = node;
        let len = self.0.len();
        let key = Key(len);
        node.index = key;
        self.0.push(node);
        key
    } 

    fn connect(&mut self, parent_index: Key, child_index: Key){
        let parent_node = self.get_node_mut(parent_index);
        parent_node.children.push(child_index);
        let child_node = self.get_node_mut(child_index);
        child_node.parent = Some(parent_index);
    }

    fn get_node(&self, index: Key) -> &Node {
        &self.0[usize::from(index)]
    }

    fn create_node(&self, rule: Rules, start_position: u32, end_position: u32, parent: Option<Key>, result: bool) -> Node {
        Node::new(rule, start_position, end_position, parent, result)
    }

    fn set_node_start_position(&mut self, index: Key, start_position: u32){
        self.0[usize::from(index)].start_position = start_position
    }
    fn set_node_end_position(&mut self, index: Key, end_position: u32){
        self.0[usize::from(index)].end_position = end_position
    }
    fn set_node_result(&mut self, index: Key, result: bool){
        self.0[usize::from(index)].result = result
    }

    fn print(&self, index: Key, boolean: Option<bool>){
        self.print_kernel(index, 0, boolean);
    }

   
}


impl Tree{

    fn get_key(&self, node: Node) -> Key{
        node.index
    }

    fn get_node_mut(&mut self, index: Key) -> &mut Node {
        &mut self.0[usize::from(index)]
    }


    fn print_kernel(&self, index: Key, indent: usize, boolean: Option<bool>){
        //thread::sleep(Duration::new(0, 5*10_u32.pow(7)));

        let node = &self.0[usize::from(index)];
        if boolean.is_some() && node.result == boolean.unwrap() {
            node.print(indent);
        }
        else if boolean.is_none(){
            if node.result{
                node.print(indent);
            }
            else{
            print!("\x1b[31m");
            node.print(indent);
            print!("\x1b[0m")
            }
        }
        let mut counter = 0;
        loop{
            if counter >= node.children.len(){
                break;
            }
            let child_index = node.children[counter];
            self.print_kernel(child_index, indent+1, boolean);
            counter += 1;
        }
    }
}

pub struct Node {
    index: Key,
    rule: Rules,
    start_position: u32,
    end_position: u32,
    result: bool,
    parent: Option<Key>,
    children: Vec<Key>,
    // To minimize allocations maybe have a second struct that contains all child indices and have Node just contain a start_child_index and end_child_index
    // Because then we can preallocate a load of memory, means a pointer indirection which may or may not impact performance so needs profiling. 
}
impl Node {
    pub fn new(rule: Rules, start_position: u32, end_position: u32, parent: Option<Key>, result: bool) -> Self {
        Node {
            index: Key(0),
            rule,
            start_position,
            end_position,
            result,
            parent,
            children: Vec::<Key>::new(),
        }
    }

    
    pub fn print(&self, indent: usize){
        println!("{}{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}", "    ".repeat(indent), self.index, self.rule, self.start_position, self.end_position, self.result, self.parent, self.children.len())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tree() {
        let x = Node::new(Rules::Grammar, 0, 0, None, false);
        let x2 = Node::new(Rules::AlphabetLower, 0, 0, None, false);
        let x3 = Node::new(Rules::AlphabetUpper, 0, 0, None, false);
        let mut t = Tree::new(40, 100);
        t.add_node(x);
        t.add_node(x2);
        t.add_node(x3);

        // Only show grammar node
        {
        t.print(Key(0), None);
        let root = t.get_node(Key(0));
        let y = t.get_node(Key(1));
        t.connect(root.index, y.index);
        println!("Show both nodes now");
        t.print(Key(0),  None);
        }
        {
        let root = t.get_node(Key(0));
        let y2 = t.get_node(Key(2));
        t.connect(root.index, y2.index);
        t.print(Key(0),  None);
        }

        println!("Multiple Subtrees");

        // Multiple subtrees
        {
        let root = t.get_node(Key(0));
        let y2 = t.get_node(Key(2));
        let y1 = t.get_node(Key(1));
        t.connect(y1.index, y2.index);
        }
        
        let root = t.get_node(Key(0));
        let y2 = t.get_node(Key(2));
        let y1 = t.get_node(Key(1));
        t.connect(root.index, y1.index);
        t.print(Key(0),  None);


    }
}
