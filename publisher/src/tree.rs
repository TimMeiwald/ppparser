use crate::Publisher;
use rules::rules::Rules;
use rules::Key;

// Tree needs to be able to swap existing structures into other locations
// E.g Rule A may fail but calls subrule B where B succeeds
// Then Rule C may try on same start location and also need subrule B
// So rather than redo subrule B we swap in the existing cached subrule B value.

pub struct Tree(Vec<Node>);

impl Publisher for Tree {
    fn new(size_of_source: usize, number_of_rules: usize) -> Self {
        Tree(Vec::<Node>::with_capacity(
            (size_of_source * number_of_rules) / 8,
        ))
    }
    fn add_node(&mut self, node: Node) -> Key {
        let mut node = node;
        let len = self.0.len();
        let key = Key(len as u32);
        node.index = key;
        self.0.push(node);
        key
    }

    fn connect(&mut self, parent_index: Key, child_index: Key) {
        let parent_node = self.get_node_mut(parent_index);
        parent_node.children.push(child_index);
        let child_node = self.get_node_mut(child_index);
        child_node.parent = Some(parent_index);
    }

    fn get_node_mut(&mut self, index: Key) -> &mut Node {
        &mut self.0[usize::from(index)]
    }

    fn get_node(&self, index: Key) -> &Node {
        &self.0[usize::from(index)]
    }
    fn get_mut_node(&mut self, index: Key) -> &mut Node {
        &mut self.0[usize::from(index)]
    }

    fn create_node(
        &self,
        rule: Rules,
        start_position: u32,
        end_position: u32,
        parent: Option<Key>,
        result: bool,
    ) -> Node {
        Node::new(rule, start_position, end_position, parent, result)
    }

    fn set_node_start_position(&mut self, index: Key, start_position: u32) {
        self.0[usize::from(index)].start_position = start_position
    }
    fn set_node_end_position(&mut self, index: Key, end_position: u32) {
        self.0[usize::from(index)].end_position = end_position
    }
    fn set_node_result(&mut self, index: Key, result: bool) {
        self.0[usize::from(index)].result = result
    }

    fn print(&self, index: Key, boolean: Option<bool>) {
        let indent = 0;
        if boolean.is_some() && boolean.unwrap() {
            self.print_kernel_true(index, indent);
        } else if boolean.is_some() && !boolean.unwrap() {
            self.print_kernel_false(index, indent);
        } else {
            self.print_kernel_all(index, indent);
        }
    }

    fn clear_false(&self) -> Tree {
        // Since Tree is a write only structure we need to remvoe False results to be more useful at some point
        // Consume self and return a new Tree only containing the True results.
        let capacity = self.len() / 4; // Heuristic
        let mut new_tree = Tree::new(capacity, 1);

        let index = 0;

        let node = self.get_node(Key(index));

        if node.result {
            let new_node = self.create_node(
                node.rule,
                node.start_position,
                node.end_position,
                node.parent,
                node.result,
            );
            new_tree.add_node(new_node);
            for i in &node.children {
                self.clear_false_kernel(&mut new_tree, Key(index), *i);
            }
        }
        new_tree
    }
}

impl Tree {
    fn clear_false_kernel(&self, new_tree: &mut Tree, parent_index: Key, index: Key) {
        let node = self.get_node(index);
        if node.result && (node.start_position != node.end_position) {
            let new_node = self.create_node(
                node.rule,
                node.start_position,
                node.end_position,
                Some(parent_index),
                node.result,
            );
            let child_index = new_tree.add_node(new_node);
            new_tree.connect(parent_index, child_index);
            for i in &node.children {
                self.clear_false_kernel(new_tree, child_index, *i);
            }
        }
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    fn print_kernel_true(&self, index: Key, indent: usize) {
        let node = &self.0[usize::from(index)];
        if !node.result {
            return;
        }
        node.print(indent);
        let mut counter = 0;
        loop {
            if counter >= node.children.len() {
                break;
            }
            let child_index = node.children[counter];
            self.print_kernel_true(child_index, indent + 1);
            counter += 1;
        }
    }
    fn print_kernel_false(&self, index: Key, indent: usize) {
        let node = &self.0[usize::from(index)];
        if node.result {
            return;
        }
        print!("\x1b[31m");
        node.print(indent);
        print!("\x1b[0m");

        let mut counter = 0;
        loop {
            if counter >= node.children.len() {
                break;
            }
            let child_index = node.children[counter];
            self.print_kernel_false(child_index, indent + 1);
            counter += 1;
        }
    }

    fn print_kernel_all(&self, index: Key, indent: usize) {
        //thread::sleep(Duration::new(0, 5*10_u32.pow(7)));

        let node = &self.0[usize::from(index)];
        if !node.result {
            print!("\x1b[31m");
        }
        node.print(indent);
        if !node.result {
            print!("\x1b[0m")
        }
        let mut counter = 0;
        loop {
            if counter >= node.children.len() {
                break;
            }
            let child_index = node.children[counter];
            self.print_kernel_all(child_index, indent + 1);
            counter += 1;
        }
    }
}

pub struct Node {
    index: Key,
    pub rule: Rules,
    pub start_position: u32,
    pub end_position: u32,
    pub result: bool,
    pub parent: Option<Key>,
    children: Vec<Key>,
    // To minimize allocations maybe have a second struct that contains all child indices and have Node just contain a start_child_index and end_child_index
    // Because then we can preallocate a load of memory, means a pointer indirection which may or may not impact performance so needs profiling.
}
impl Node {
    pub fn new(
        rule: Rules,
        start_position: u32,
        end_position: u32,
        parent: Option<Key>,
        result: bool,
    ) -> Self {
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

    pub fn get_string(&self, source: &str) -> String {
        source[(self.start_position as usize)..(self.end_position as usize)].to_string()
    }

    pub fn get_children(&self) -> &Vec<Key> {
        &self.children
    }
    pub fn set_children(&mut self, children: Vec<Key>) {
        self.children = children;
    }

    pub fn print(&self, indent: usize) {
        println!(
            "{}{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
            "    ".repeat(indent),
            self.index,
            self.rule,
            self.start_position,
            self.end_position,
            self.result,
            self.parent,
            self.children.len()
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tree() {
        let x = Node::new(Rules::Grammar, 0, 0, None, false);
        let x2 = Node::new(Rules::Alphabet_Lower, 0, 0, None, false);
        let x3 = Node::new(Rules::Alphabet_Upper, 0, 0, None, false);
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
            t.print(Key(0), None);
        }
        {
            let root = t.get_node(Key(0));
            let y2 = t.get_node(Key(2));
            t.connect(root.index, y2.index);
            t.print(Key(0), None);
        }

        println!("Multiple Subtrees");

        // Multiple subtrees
        {
            let _root = t.get_node(Key(0));
            let y2 = t.get_node(Key(2));
            let y1 = t.get_node(Key(1));
            t.connect(y1.index, y2.index);
        }

        let root = t.get_node(Key(0));
        let _y2 = t.get_node(Key(2));
        let y1 = t.get_node(Key(1));
        t.connect(root.index, y1.index);
        t.print(Key(0), None);
    }
}
