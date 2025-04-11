use std::collections::{HashMap, HashSet};

use parser::{Node, Rules};

use crate::{BasicPublisher, Key};
/// For each rule we must detect whether there is a cycle to
/// determine if there is left recursion.
/// We then store this information for generatoion time
pub struct LeftRecursionDetector<'a> {
    source: &'a String,
    rules_name_map: HashMap<String, Key>, // Lookup a key for a rule by the name of the rule.
    left_recursion_rules: HashMap<String, HashSet<String>>
}

impl<'a> LeftRecursionDetector<'a> {
    fn new(tree: &BasicPublisher, source: &'a String) -> Self {
        let mut lr_detector = LeftRecursionDetector {
            source,
            rules_name_map: HashMap::new(),
            left_recursion_rules: HashMap::new()
        };
        lr_detector.get_rule_keys(tree); // We get the declared rules and their keys
                                         // So we can then lookup the rule in the tree when it's referenced in a different rule
        lr_detector.left_walk_init(tree);
        lr_detector
    }
    fn print_rules_name_map(&self) {
        println!("{:#?}", self.rules_name_map);
    }
    fn print_left_recursive_rules(&self){
        println!("{:#?}", self.left_recursion_rules);
    }
    fn get_rule_name(&mut self, tree: &BasicPublisher, node: &Node) -> String {
        debug_assert_eq!(node.rule, Rules::LHS);
        let var_name_decl = node.get_children()[0];
        let var_name_decl = tree.get_node(var_name_decl);
        debug_assert_eq!(var_name_decl.rule, Rules::Var_Name_Decl);
        var_name_decl.get_string(&self.source)
    }
    fn get_rule_keys(&mut self, tree: &BasicPublisher) {
        let node = tree.get_node(Key(0));
        debug_assert_eq!(node.rule, Rules::Grammar);
        for key in node.get_children() {
            let child = tree.get_node(*key);
            match child.rule {
                Rules::Rule => {
                    //// println!("{}", child.get_string(&self.source));
                    let lhs = child.get_children()[0];
                    let lhs = tree.get_node(lhs);
                    let rule_name = self.get_rule_name(tree, lhs);
                    self.rules_name_map.insert(rule_name, *key);
                }
                _ => {}
            }
        }
    }

    fn left_walk_init(&mut self, tree: &BasicPublisher) {
        // We loop over every rule
        // We follow each rule reference in the rule
        // But only the first rule(since otherwise it's not LR(I think!))
        let node = tree.get_node(Key(0));
        debug_assert_eq!(node.rule, Rules::Grammar);
        for key in node.get_children() {
            let child = tree.get_node(*key);
            let mut rules_set: HashSet<String> = HashSet::new();
            match child.rule {
                Rules::Rule => {
                    debug_assert_eq!(child.rule, Rules::Rule);
                    let lhs = child.get_children()[0];
                    let lhs = tree.get_node(lhs);
                    let parent_rule_name = self.get_rule_name(tree, lhs);
                    // println!("\nRule: {:?}", parent_rule_name);
                    let rhs_key = child.get_children()[1];
                    let rhs = tree.get_node(rhs_key);
                    debug_assert_eq!(rhs.rule, Rules::RHS);
                    self.left_walk_kernel(tree, rhs_key, parent_rule_name, &mut rules_set);
                }
                _ => {}
            }
        }
    }

    fn left_walk_kernel(
        &mut self,
        tree: &BasicPublisher,
        key: Key,
        parent_rule_name: String,
        mut rules_set: &mut HashSet<String>,
    ) {
        // Since a jump to a reference jumps to a rule and we actually just want
        // The first child of the RHS.
        // We check if the node is itself a rule, if yes we grab the RHS index not the left most.
        let node = tree.get_node(key);
        let left_most_child: Option<&Key>;
        
        match node.rule {
            Rules::Rule => {
                // Since assignment and whitespace are inlined
                // The 2nd rule to get called in rule is index 1 of the children.
                left_most_child = node.get_children().get(1)
            }
            Rules::Ordered_Choice => {
                left_most_child = node.get_children().get(0);
                let right_most_child = node.get_children().get(1).expect("OC always has 2 children");
                self.left_walk_kernel(tree, *right_most_child, parent_rule_name.clone(), rules_set);
            }
            _ => {
                left_most_child = node.get_children().get(0);
            }
        }

        // If it get's here they're terminals.
        match left_most_child {
            Some(child) => {
                // println!("Going into child of {:?}", node.rule);
                self.left_walk_kernel(tree, *child, parent_rule_name, rules_set);
            }
            None => {
                match node.rule {
                    Rules::Var_Name_Ref => {
                        // Since a LR rule would cycle endlessly we must know when to terminate.
                        // Since we also want to support indirect left recursion
                        // We use a stack to push the rules onto and then check it's not repeating.

                        if !rules_set.insert(node.get_string(&self.source)) {
                            // Was already in the list so we can stop because it means
                            // We'll be looping
                            // println!("{:#?}", rules_set);
                            self.left_recursion_rules.insert(parent_rule_name, rules_set.clone());
                            return;
                        }
                        let key = self
                            .rules_name_map
                            .get(&node.get_string(&self.source))
                            .expect("The index should exist. If it doesn't the program is broken.");
                        // println!("Jumping to Rule: {:?}", node.rule);
                        // If it's a reference to a rule then we jump to that rule's index and keep recursing.
                        self.left_walk_kernel(tree, *key, parent_rule_name, rules_set);
                    }
                    _ => {
                        // If it's some other terminal type we ignore it since it terminates.
                        // println!("Terminal {:?}", node.rule);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::count_lines;

    use ::parser::*;
    use std::cell::RefCell;

    use std::fs::{canonicalize, read_to_string};
    use std::io::stdout;
    use std::io::Write;

    #[test]
    fn test_no_lr() {
        let path = "../parser/tests/test_data/Grammar.txt";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let result = grammar(Key(0), &context, &source, position);

        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            // println!("Successfully parsed")
        }
        let source = &String::from(source);
        let tree = context.into_inner();
        let tree = &tree.get_publisher().clear_false();
        let _lr_detector = LeftRecursionDetector::new(tree, source);
        _lr_detector.print_left_recursive_rules();
        let _f = stdout().flush().expect("Why did it not flush");
        // println!("\n\n\n");
        //lr_detector.print_rules_name_map();
        //tree.print(Key(0), None);
        // let src = &String::from(source);
        // let sym_table = SymbolTable::new(tree, src);
        // sym_table.print();
        // let _gen_code = GeneratedCode::new(&sym_table, &tree, src);
        // _gen_code.print();
    }


    #[test]
    fn test_calculator() {
        let path = "../generator/tests/calculator.dsl";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let result = grammar(Key(0), &context, &source, position);

        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            // println!("Successfully parsed")
        }
        let source = &String::from(source);
        let tree = context.into_inner();
        let tree = &tree.get_publisher().clear_false();
        let _lr_detector = LeftRecursionDetector::new(tree, source);
        _lr_detector.print_left_recursive_rules();
        let _f = stdout().flush().expect("Why did it not flush");
        // println!("\n\n\n");
        //lr_detector.print_rules_name_map();
        //tree.print(Key(0), None);
        // let src = &String::from(source);
        // let sym_table = SymbolTable::new(tree, src);
        // sym_table.print();
        // let _gen_code = GeneratedCode::new(&sym_table, &tree, src);
        // _gen_code.print();
    }
}
