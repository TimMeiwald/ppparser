//

pub struct StackEntry{
    pub rule: u32,
    pub start_position: u32,
    pub end_position: u32,
}

pub struct Stack{
    entries: Vec<StackEntry>
}
impl Stack{
    pub fn new(size_of_source: u32, number_of_structs: u32) -> Self {
        let size_of_stack: usize = ((size_of_source*number_of_structs)/10).try_into().unwrap(); // /10 is a heuristic 
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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        // Simulating what would happen after a rule that consumes one character parses and returns true
        let se = StackEntry{rule: 0, start_position: 5, end_position: 10};
        let mut s = Stack::new(100, 20);
        s.push(se);
        let r = s.pop().unwrap();
        assert_eq!(r.rule, 0);
        assert_eq!(r.start_position, 5);
        assert_eq!(r.end_position, 10);

    }

}
