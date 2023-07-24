//

//

use std::process::exit;

use crate::parser::Rules;

pub struct StackEntry{
    pub rule: Rules,
    pub start_position: i32,
    pub end_position: i32,
}

pub struct Stack{
    entries: Vec<StackEntry>
}
impl Stack{
    pub fn new(size_of_source: i32, number_of_structs: i32) -> Self {
        let size_of_stack: usize = ((3*size_of_source*number_of_structs)).try_into().unwrap(); // /10 is a heuristic 
        // Heuristic should be modified to match grammar profiling since some will use more stack and some less depending on language complexity.
        // TODO: Fundamentally Vec can grow so still need to catch unwrap above but it's merely about minimizing allocation rather than causing a break
        return Stack { entries: Vec::with_capacity(size_of_stack)};
    }
    pub fn push(&mut self, entry: StackEntry) -> usize {
        let position = self.entries.len();
        self.entries.push(entry);
        return position;

    }

    pub fn update(&mut self, position: usize, end_position: i32){
        self.entries[position].end_position = end_position;
        //println!("{:?}, {}, {}",self.entries[position].rule, self.entries[position].start_position,  self.entries[position].end_position);

    }
    pub fn len(&self) -> usize{
        self.entries.len()
    }
    pub fn unravel(&mut self, result_bool: bool, start_position: i32, end_position: i32, entry_position: usize){
        if result_bool == false || start_position == end_position { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = self.len();
                if stack_length > entry_position{
                    self.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            self.update(entry_position, end_position);
        }
    }
    pub fn pop(&mut self) -> Option<StackEntry> {
        //println!("Current Len {}", self.entries.len());
        return self.entries.pop();
    }

    pub fn read(&self, position: usize) -> Option<&StackEntry> {
        let entry = self.entries.get(position);
        return entry;
    }

    pub fn print_with_strings(&self, source: &str)
    {
        let stack_iterator = self.into_iter();
        for entry in stack_iterator{
            let s: String = format!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize]);
            println!("{}", s);
        }
    }


    pub fn print(&self){
        let stack_iterator = self.into_iter();
        for entry in stack_iterator{
            let s: String = format!("{:?}, {}, {}", entry.rule, entry.start_position, entry.end_position);
            println!("{}", s);
        }

    }
}

impl<'a> IntoIterator for &'a Stack{
    type Item = &'a StackEntry;
    type IntoIter = StackIter<'a>;
    fn into_iter(self) -> Self::IntoIter{
        StackIter{stack: &self, count: 0}

    }
}

pub struct StackIter<'a>{
    count: isize,
    stack: &'a Stack,
}

impl<'a>  Iterator for StackIter<'a>{
    type Item = &'a StackEntry;
    fn next(&mut self) -> Option<&'a StackEntry>{
        let entry = self.stack.read(self.count as usize);
        self.count += 1;
        return entry;
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        // Simulating what would happen after a rule that consumes one character parses and returns true
        let se = StackEntry{rule: Rules::AlphabetLower, start_position: 5, end_position: 10};
        let mut s = Stack::new(100, 20);
        s.push(se);
        let r = s.pop().unwrap();
        assert_eq!(r.rule as i32, 1);
        assert_eq!(r.start_position, 5);
        assert_eq!(r.end_position, 10);

    }

}
