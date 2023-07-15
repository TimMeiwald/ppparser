//

//

use std::process::exit;

use crate::parser::Rules;

pub struct StackEntry{
    pub rule: Rules,
    pub start_position: u32,
    pub end_position: u32,
}

pub struct Stack{
    entries: Vec<StackEntry>
}
impl Stack{
    pub fn new(size_of_source: u32, number_of_structs: u32) -> Self {
        let size_of_stack: usize = ((3*size_of_source*number_of_structs)).try_into().unwrap(); // /10 is a heuristic 
        // Heuristic should be modified to match grammar profiling since some will use more stack and some less depending on language complexity.
        // TODO: Fundamentally Vec can grow so still need to catch unwrap above but it's merely about minimizing allocation rather than causing a break
        return Stack { entries: Vec::with_capacity(size_of_stack)};
    }
    pub fn push(&mut self, entry: StackEntry){
        self.entries.push(entry);

    }
    pub fn pop(&mut self) -> Option<StackEntry> {
        return self.entries.pop();
    }

    pub fn read(&self, position: usize) -> Option<&StackEntry> {
        let entry = self.entries.get(position);
        return entry;
    }

    pub fn print_with_strings(&self, source: &str)
    {
        let stack_iterator = self.into_iter();
        let mut print_vec: Vec<String> = Vec::new();
        // Invert order because terminal appears inverted because of push down
        for entry in stack_iterator{
            let s: String = format!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize]);
            print_vec.push(s);
        }
        // Mostly only so I don't need to implement double ended iterators TODO make this less crap
        for string in print_vec.iter(){
            println!("{}", string);
        }
    }


    pub fn print(&self){
        let stack_iterator = self.into_iter();
        let mut print_vec: Vec<String> = Vec::new();
        // Invert order because terminal appears inverted because of push down
        for entry in stack_iterator{
            let s: String = format!("{:?}, {}, {}", entry.rule, entry.start_position, entry.end_position);
            print_vec.push(s);
        }
        for string in print_vec.iter(){
            println!("{}", string);
        }
    }
}

impl<'a> IntoIterator for &'a Stack{
    type Item = &'a StackEntry;
    type IntoIter = StackIter<'a>;
    fn into_iter(self) -> Self::IntoIter{
        let vec_size = self.entries.len() as u32;
        let count = vec_size.checked_sub(1);
        if count.is_none() {
            println!("Error: Stack has no contents");
            exit(-1);
        }
        let count = count.unwrap() as u32;
        StackIter{stack: &self, count: count as usize}
        //StackIter{stack: &self, count: 0}

    }
}

pub struct StackIter<'a>{
    count: usize,
    stack: &'a Stack,
}

impl<'a>  Iterator for StackIter<'a>{
    type Item = &'a StackEntry;
    fn next(&mut self) -> Option<&'a StackEntry>{
        let entry = self.stack.read(self.count);
        if self.count == 0 {
            return None
        }
        self.count -= 1;
        //self.count += 1;
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
        assert_eq!(r.rule as u32, 1);
        assert_eq!(r.start_position, 5);
        assert_eq!(r.end_position, 10);

    }

}
