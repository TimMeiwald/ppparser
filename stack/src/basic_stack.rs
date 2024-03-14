use crate::Stack;

use std::marker::PhantomData;

// Immediately go to struct of arrays rather than array of structs since it was significantly faster in Cache
// This of course requires perf testing at some point but this is just a poc so I can get other work done.

// Every AST node can be described by the Rule it matches and the start and end positions of it's content.
// The order in which the nodes are pushed into the stack dictate the parent-child relationships
// Any Node who's start and end position is fully contained inside the previous nodes start and end position is a child. etc

pub struct BasicStack<'a> {
    rules: Vec<u32>,
    start_positions: Vec<u32>,
    end_positions: Vec<u32>,
    phantom: PhantomData<&'a BasicStack<'a>>,
}

impl<'a> Stack for BasicStack<'a> {
    fn new(size_of_source: u32, number_of_rules: u32) -> Self {
        // Since the Stack needs to be able to take all the elements generated by the AST and we can't guarantee
        // ahead of time how many elements that is, we will use a heuristic of the source size*number_of_structs*some_hardcoded_parameter
        let parameter = 1; // TODO: May be worth enabling to make this tunable at some point
        let capacity = (size_of_source * number_of_rules * parameter) as usize;
        println!("Bytes Available: {:?}", capacity);
        BasicStack {
            rules: Vec::with_capacity(capacity),
            start_positions: Vec::with_capacity(capacity),
            end_positions: Vec::with_capacity(capacity),
            phantom: PhantomData,
        }
    }
    fn push(&mut self, is_true: bool, rule: u32, start_position: u32, end_position: u32) -> u32 {
        let index: u32 = self.rules.len() as u32;
        match is_true {
            true => {
                // When true push onto stack
                
                self.rules.push(rule);
                self.start_positions.push(start_position);
                self.end_positions.push(end_position);
                return index;
            }
            false => {
                // When false pop all instructiosn that exist between start_position and end_position.
                loop {
                    if index == 0 {
                        break;
                    }
                    let last_index = index - 1;
                    //let last_elem_rule = self.rules[last_index];
                    let last_start_position = self.start_positions[last_index as usize];
                    let last_end_position = self.end_positions[last_index as usize];

                    if start_position >= last_start_position && end_position < last_end_position {
                        self.rules.pop();
                        self.end_positions.pop();
                        self.start_positions.pop();
                    } else {
                        break;
                    }
                }
                return index;
            }
        }
    }

    fn patch(&mut self, index: u32, is_true: bool, rule: u32, start_position: u32, end_position: u32) {
        
    }

    fn remove(&mut self, _index: u32) {}
}

impl<'a> Iterator for BasicStackIterator<'a> {
    type Item = [u32; 3];
    fn next(&mut self) -> Option<[u32; 3]> {
        let index = self.counter;
        if index < self.stack.rules.len() {
            let rule = self.stack.rules[index];
            let start_position = self.stack.start_positions[index];
            let end_position = self.stack.end_positions[index];
            self.counter += 1;
            Some([rule, start_position, end_position])
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a BasicStack<'_> {
    type Item = [u32; 3];
    type IntoIter = BasicStackIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BasicStackIterator {
            stack: self,
            counter: 0,
        }
    }
}

pub struct BasicStackIterator<'a> {
    stack: &'a BasicStack<'a>,
    counter: usize,
}

// // Inverted
// pub struct BasicStackIterator<'a> {
//     stack: &'a BasicStack<'a>,
//     counter: i32,
// }
// impl<'a> Iterator for BasicStackIterator<'a> {
//     type Item = [u32; 3];
//     fn next(&mut self) -> Option<[u32; 3]> {
//         let index = self.counter;
//         if index >= 0 {
//             let rule = self.stack.rules[index as usize];
//             let start_position = self.stack.start_positions[index  as usize];
//             let end_position = self.stack.end_positions[index  as usize];
//             self.counter -= 1;
//             Some([rule, start_position, end_position])
//         } else {
//             None
//         }
//     }
// }

// impl<'a> IntoIterator for &'a BasicStack<'_> {
//     type Item = [u32; 3];
//     type IntoIter = BasicStackIterator<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         BasicStackIterator {
//             stack: self,
//             counter: (self.rules.len()-1) as i32,
//         }
//     }
// }

impl<'a> BasicStack<'a> {
    pub fn get(&self, index: usize) -> Option<[u32; 3]> {
        if index < self.rules.len() {
            let rule: u32 = self.rules[index];
            let start_position: u32 = self.start_positions[index];
            let end_position: u32 = self.end_positions[index];
            Some([rule, start_position, end_position])
        } else {
            None
        }
    }

    pub fn print(&self, source: &String) {
        let l1 = self.rules.len();
        let l2 = self.start_positions.len();
        let l3 = self.end_positions.len();
        if l1 == l2 && l2 == l3 && l3 == l1 {
            for i in 0..l1 {
                let p1 = self.rules[i];
                let p2 = self.start_positions[i];
                let p3 = self.end_positions[i];
                let s1 = p2 as usize;
                let s2 = p3 as usize;
                let slice = &source[s1..s2];
                if p1 == 20 {
                    println!(
                        "Rule: {:?}, Start Position: {:?}, End Position: {:?} => {}",
                        p1, p2, p3, slice
                    );
                }
            }
        } else {
            println!("Something went wrong with BasicStack, Vector lengths must be identical.");
        }
    }
}
