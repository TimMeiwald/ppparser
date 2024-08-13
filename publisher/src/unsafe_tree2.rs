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

// pub struct Children(Vec<Key>);
// impl Children {
//     fn new(size_of_source: usize, number_of_structs: usize) -> Self {
//         let capacity: usize = size_of_source * number_of_structs;
//         // TODO allow this to be controllable.
//         Children {
//             0: Vec::with_capacity(capacity),
//         }
//     }

//     fn push(&mut self, child: Key) -> Index {
//         self.0.push(child);
//         Index::from(self.0.len())
//     }

//     fn clear(&mut self) {
//         self.0.clear();
//     }
// }

pub struct UnsafeTree2 {
    last_node: Option<Key>,
    rules: Vec<Rules>,
    start_positions: Vec<u32>,
    end_positions: Vec<u32>,
    results: Vec<bool>,
    parents: Vec<Option<Key>>,
    children_start: Vec<Index>,
    children_end: Vec<Index>,
    children: Vec<Key>,
}

impl Publisher for UnsafeTree2 {
    fn new(size_of_source: usize, number_of_structs: usize) -> Self {
        let capacity: usize = size_of_source * number_of_structs * 4;
        UnsafeTree2 {
            last_node: None,
            rules: Vec::with_capacity(capacity),
            start_positions: Vec::with_capacity(capacity),
            end_positions: Vec::with_capacity(capacity),
            results: Vec::with_capacity(capacity),
            parents: Vec::with_capacity(capacity),
            children_start: Vec::with_capacity(capacity),
            children_end: Vec::with_capacity(capacity),
            children: Vec::with_capacity(capacity),
        }
    }

    fn clear_tree(&mut self) {
        self.last_node = None;
        self.rules.clear();
        self.start_positions.clear();
        self.end_positions.clear();
        self.results.clear();
        self.parents.clear();
        self.children_start.clear();
        self.children_end.clear();
        self.children.clear();
    }
    #[inline(always)]
    fn add_node(
        &mut self,
        rule: Rules,
        start_position: u32,
        end_position: u32,
        parent: Option<Key>,
        result: bool,
    ) -> Key {
        let len = self.rules.len();
        // let key = Key(len.try_into().unwrap());
        let key = Key { 0: len as u32 };
        self.parents.push(parent);
        self.rules.push(rule);
        self.start_positions.push(start_position);
        self.end_positions.push(end_position);
        self.results.push(result);
        self.children_start.push(Index(u32::MAX));
        self.children_end.push(Index(u32::MAX));
        return key;
    }
    #[inline(always)]
    fn connect(&mut self, parent_index: Key, child_index: Key) {
        let idx = Index(self.children.len() as u32);
        self.children.push(child_index);

        let child_start = unsafe { *self.children_start.get_unchecked(usize::from(parent_index)) };

        if child_start == Index(u32::MAX) {
            unsafe {
                let node = self
                    .children_start
                    .get_unchecked_mut(usize::from(parent_index));
                let node = node as *mut Index;
                (*node) = idx;
                let node = self
                    .children_end
                    .get_unchecked_mut(usize::from(parent_index));
                let node = node as *mut Index;
                (*node) = idx;
            }
        } else {
            unsafe {
                let node = self
                    .children_end
                    .get_unchecked_mut(usize::from(parent_index));
                let node = node as *mut Index;
                (*node) = idx;
            }
        }
    }
    #[inline(always)]

    fn set_node_start_position(&mut self, index: Key, start_position: u32) {
        unsafe {
            let node = self.start_positions.get_unchecked_mut(usize::from(index));
            let node = node as *mut u32;
            (*node) = start_position;
        }
    }
    #[inline(always)]

    fn set_node_end_position(&mut self, index: Key, end_position: u32) {
        unsafe {
            let node = self.end_positions.get_unchecked_mut(usize::from(index));
            let node = node as *mut u32;
            (*node) = end_position;
        }
    }
    #[inline(always)]

    fn set_node_result(&mut self, index: Key, result: bool) {
        unsafe {
            let node = self.results.get_unchecked_mut(usize::from(index));
            let node = node as *mut bool;
            (*node) = result;
        }
    }
    #[inline(always)]
    fn set_last_node(&mut self, key: Option<Key>) {
        self.last_node = key;
    }
    #[inline(always)]

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
