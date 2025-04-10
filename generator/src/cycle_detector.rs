use std::collections::HashMap;

use parser::{Node, Rules};

use crate::{BasicPublisher, Key};
/// For each rule we must detect whether there is a cycle to
/// determine if there is left recursion.
/// We then store this information for generatoion time
pub struct LeftRecursionDetector<'a> {
    source: &'a String,
    rules_name_map: HashMap<String, Key>, // Lookup a key for a rule by the name of the rule.
}

impl<'a> LeftRecursionDetector<'a> {
    fn new(tree: &BasicPublisher, source: &'a String) -> Self {
        let mut lr_detector = LeftRecursionDetector {
            source,
            rules_name_map: HashMap::new(),
        };
        lr_detector.get_rule_keys(tree); // We get the declared rules and their keys
                                         // So we can then lookup the rule in the tree when it's referenced in a different rule
        lr_detector.left_walk_init(tree);
        lr_detector
    }
    fn print_rules_name_map(&self) {
        println!("{:#?}", self.rules_name_map);
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
                    //println!("{}", child.get_string(&self.source));
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
            match child.rule {
                Rules::Rule => {
                    self.left_walk_kernel(tree, *key);
                }
                _ => {}
            }
        }
    }

    fn left_walk_kernel(&mut self, tree: &BasicPublisher, key: Key) {
        let node = tree.get_node(key);
        debug_assert_eq!(node.rule, Rules::Rule);
        let rhs = node.get_children()[1];
        let rhs = tree.get_node(rhs);
        debug_assert_eq!(rhs.rule, Rules::RHS);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::count_lines;
    use crate::GeneratedCode;
    use crate::SymbolTable;
    use ::parser::*;
    use std::cell::RefCell;
    use std::env;
    use std::fs::{canonicalize, read_to_string};

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
            println!("Successfully parsed")
        }
        let source = &String::from(source);
        let tree = context.into_inner();
        let tree = &tree.get_publisher().clear_false();
        let lr_detector = LeftRecursionDetector::new(tree, source);
        //lr_detector.print_rules_name_map();
        //tree.print(Key(0), None);
        // let src = &String::from(source);
        // let sym_table = SymbolTable::new(tree, src);
        // sym_table.print();
        // let _gen_code = GeneratedCode::new(&sym_table, &tree, src);
        // _gen_code.print();
    }
}
