use crate::{Node, Publisher, Tree};
use rules::rules::Rules;
use rules::Key;
// Tree needs to be able to swap existing structures into other locations
// E.g Rule A may fail but calls subrule B where B succeeds
// Then Rule C may try on same start location and also need subrule B
// So rather than redo subrule B we swap in the existing cached subrule B value.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Index(pub u32);
impl From<usize> for Index {
    fn from(i: usize) -> Index {
        Index(i as u32)
    }
}
impl From<Index> for usize {
    fn from(i: Index) -> usize {
        i.0 as usize
    }
}

pub struct Children(Vec<Key>);
impl Children {
    fn new(size_of_source: usize, number_of_structs: usize) -> Self {
        let capacity: usize = size_of_source * number_of_structs;
        // TODO allow this to be controllable.
        Children {
            0: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, child: Key) -> Index {
        self.0.push(child);
        Index::from(self.0.len())
    }
}

pub struct Tree2 {
    last_node: Option<Key>,
    rules: Vec<Rules>,
    start_positions: Vec<u32>,
    end_positions: Vec<u32>,
    results: Vec<bool>,
    parents: Vec<Option<Key>>,
    children_start: Vec<Index>,
    children_end: Vec<Index>,
    children: Children,
}

impl Publisher for Tree2 {
    fn new(size_of_source: usize, number_of_structs: usize) -> Self {
        let capacity: usize = size_of_source * number_of_structs;
        Tree2 {
            last_node: None,
            rules: Vec::with_capacity(capacity),
            start_positions: Vec::with_capacity(capacity),
            end_positions: Vec::with_capacity(capacity),
            results: Vec::with_capacity(capacity),
            parents: Vec::with_capacity(capacity),
            children_start: Vec::with_capacity(capacity),
            children_end: Vec::with_capacity(capacity),
            children: Children::new(size_of_source, number_of_structs),
        }
    }

    fn add_node(
        &mut self,
        rule: Rules,
        start_position: u32,
        end_position: u32,
        parent: Option<Key>,
        result: bool,
    ) -> Key {
        let len = self.rules.len();
        let key = Key(len.try_into().unwrap());
        self.parents.push(parent);
        self.rules.push(rule);
        self.start_positions.push(start_position);
        self.end_positions.push(end_position);
        self.results.push(result);
        self.children_start.push(Index(u32::MAX));
        self.children_end.push(Index(u32::MAX));
        return key;
    }

    fn connect(&mut self, parent_index: Key, child_index: Key) {
        let idx = self.children.push(child_index);
        if self.children_start[usize::from(parent_index)] == Index(u32::MAX) {
            self.children_start[usize::from(parent_index)] = idx;
            self.children_end[usize::from(parent_index)] = idx;
        } else {
            self.children_end[usize::from(parent_index)] = idx;
        }
    }

    fn set_node_start_position(&mut self, index: Key, start_position: u32) {
        self.start_positions[usize::from(index)] = start_position;
    }
    fn set_node_end_position(&mut self, index: Key, end_position: u32) {
        self.end_positions[usize::from(index)] = end_position;
    }
    fn set_node_result(&mut self, index: Key, result: bool) {
        self.results[usize::from(index)] = result;
    }
    fn set_last_node(&mut self, key: Option<Key>) {
        self.last_node = key;
    }
    fn last_node(&self) -> Option<Key> {
        self.last_node
    }
    fn clear_false(&self) -> Tree {
        let len = self.rules.len();
        let mut new_tree = Tree::new(len, 1);
        for i in 0..len {
            let result = self.results[i];
            if result {
                let rule = self.rules[i];
                let start_position = self.start_positions[i];
                let end_position = self.end_positions[i];
                let parent = self.parents[i];
                new_tree.add_node(rule, start_position, end_position, parent, result);
                let rs = usize::from(self.children_start[i]);
                let re = usize::from(self.children_end[i]);
                let r: Vec<usize> = (rs..re).collect();
                for child in r {
                    // TODO Connect
                    todo!("Not implemented yet the connections")
                }
            }
        }
        return new_tree;
    }

    fn print(&self, index: Key, boolean: Option<bool>) {
        println!("Not yet implemented");
    }
}
