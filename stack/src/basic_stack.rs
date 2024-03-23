use crate::Stack;

use std::{marker::PhantomData, thread::sleep, time::Duration};

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
    fn push(&mut self, _is_true: bool, rule: u32, start_position: u32, end_position: u32) -> u32 {
        let index = self.rules.len();
        self.rules.push(rule);
        self.start_positions.push(start_position);
        self.end_positions.push(end_position);
        return index as u32;

        // let index: u32 = self.rules.len() as u32;
        // match is_true {
        //     true => {
        //         // When true push onto stack

        //         self.rules.push(rule);
        //         self.start_positions.push(start_position);
        //         self.end_positions.push(end_position);
        //         return index;
        //     }
        //     false => {
        //         // When false pop all instructiosn that exist between start_position and end_position.
        //         loop {
        //             if index == 0 {
        //                 break;
        //             }
        //             let last_index = index - 1;
        //             //let last_elem_rule = self.rules[last_index];
        //             let last_start_position = self.start_positions[last_index as usize];
        //             let last_end_position = self.end_positions[last_index as usize];

        //             if start_position >= last_start_position && end_position < last_end_position {
        //                 self.rules.pop();
        //                 self.end_positions.pop();
        //                 self.start_positions.pop();
        //             } else {
        //                 break;
        //             }
        //         }
        //         return index;
        //     }
        // }
    }

    fn patch(
        &mut self,
        index: u32,
        _is_true: bool,
        _rule: u32,
        start_position: u32,
        end_position: u32,
    ) {
        self.start_positions[index as usize] = start_position;
        self.end_positions[index as usize] = end_position;
    }

    fn pop(&mut self) {
        self.rules.pop();
        self.start_positions.pop();
        self.end_positions.pop();
    }
    fn pop_to(&mut self, index: u32) {
        // inclusive
        while (self.rules.len()) != index as usize {
            self.pop();
        }
    }
    fn get(&self, index: u32) -> [u32; 3] {
        if index < self.rules.len() as u32 {
            let rule: u32 = self.rules[index as usize];
            let start_position: u32 = self.start_positions[index as usize];
            let end_position: u32 = self.end_positions[index as usize];
            [rule, start_position, end_position]
        } else {
            panic!("Should never happen")
        }
    }

    fn read_children(&self, index: u32) -> Option<(u32, u32)> {
        if index >= self.start_positions.len() as u32 {
            return None
        }
        let parent_start = self.start_positions[index as usize];
        let parent_end = self.end_positions[index as usize];
        let mut temp_index = index + 1; // Start at child not node itself
        loop {
            if temp_index >= self.start_positions.len() as u32 {
                break
            }
            let start = self.start_positions[temp_index as usize];
            let end = self.end_positions[temp_index as usize];
            //println!("{}, {}, {}, {}", parent_start, start, parent_end, end);
            if (start >= parent_start) && (end <= parent_end) {
                // Then it's a child node
                //println!("Child Node");
                temp_index += 1;
            } else {
                break;
            }
        }
        if temp_index == index + 1 {
            None
        } else {
            Some((index, temp_index))
        }
    }
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


    pub fn is_index_a_child(&self, parent_index: u32, potential_child_index: u32) -> bool {
        let parent_start = self.start_positions[parent_index as usize];
        let parent_end = self.end_positions[parent_index as usize];

        //println!("Read Children Index: {}", temp_index);

        let start = self.start_positions[potential_child_index as usize];
        let end = self.end_positions[potential_child_index as usize];
        //println!("{}, {}, {}, {}", parent_start, start, parent_end, end);
        if (start >= parent_start) && (end <= parent_end) {
            // Then it's a child node
            // println!("Child Node");
            true
        } else {
            false
        }
    }

    pub fn print_kernel(&self, index: u32, indent: u32, source: &String) -> u32{
        let parent_rule = self.rules[index as usize];
        let parent_start = self.start_positions[index as usize];
        let parent_end = self.end_positions[index as usize];
        println!("{}{}, {}, {}, {}", "    ".repeat(indent as usize), parent_rule, parent_start, parent_end, &source[(parent_start as usize)..(parent_end as usize)]);
        let mut temp_index = index + 1;
        //sleep(Duration::new(0, 100000000));
        loop{
        if temp_index >= self.rules.len() as u32{
            return temp_index;
        }
        if self.is_index_a_child(index, temp_index){
            temp_index = self.print_kernel(temp_index, indent+1, source)

        }
        else{
            return temp_index
        }
    }


}

    pub fn print(&self, source: &String) {
        let len  = self.rules.len();
        let mut temp_index: u32 = 0;
        while temp_index < len as u32{
            temp_index = self.print_kernel(temp_index, 0, source);
        }
    }
    
    pub fn print_range(&self, source: &String, start: u32, end: u32){
        let mut temp_index: u32 = start;
        while temp_index < end as u32{
            temp_index = self.print_kernel(temp_index, 0, source);
        }
    }

}
