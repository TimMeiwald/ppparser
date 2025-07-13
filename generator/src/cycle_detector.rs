use crate::BasicPublisher;
use std::collections::{HashMap, HashSet};
pub struct LeftRecursionDetector<'a> {
    source: &'a String,
    tree: &'a BasicPublisher,
    left_recursion_rules: HashMap<String, HashSet<String>>,
}
impl<'a> LeftRecursionDetector<'a> {
    pub fn new(tree: &'a BasicPublisher, source: &'a String) -> Self {
        LeftRecursionDetector {
            source,
            tree,
            left_recursion_rules: HashMap::new(),
        }
    }
    pub fn get_left_recursion_rules(&self) -> &HashMap<String, HashSet<String>> {
        &self.left_recursion_rules
    }
}
