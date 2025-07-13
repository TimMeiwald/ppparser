use crate::{BasicPublisher, Key, Node, Rules};
use std::{
    collections::{HashMap, HashSet},
    usize,
};
pub struct LeftRecursionDetector<'a> {
    source: &'a String,
    tree: &'a BasicPublisher,
    left_recursion_rules: HashMap<String, HashSet<String>>,
    rule_call_tree: RuleCallTree,
}
impl<'a> LeftRecursionDetector<'a> {
    pub fn new(tree: &'a BasicPublisher, source: &'a String) -> Self {
        LeftRecursionDetector {
            source,
            tree,
            left_recursion_rules: HashMap::new(),
            rule_call_tree: RuleCallTree::new(tree, source),
        }
    }
    pub fn get_left_recursion_rules(&self) -> &HashMap<String, HashSet<String>> {
        &self.left_recursion_rules
    }
}

#[derive(Debug, PartialEq, Clone)]
enum LeftRecursive {
    SelfCycleDetected,          // The rule itself is left recursive
    False,                      // Not left recursive
    OtherCycleDetected(String), // Cycles were detected but is not the rule we're investigating
}

struct RuleCallTree {
    rules_keys: HashMap<String, Key>,
    rules_rhs_keys: HashMap<String, Key>,
    rules_referenced_by_rule: HashMap<String, HashSet<String>>,
    is_rule_left_recursive: HashMap<String, LeftRecursive>,
}
impl RuleCallTree {
    fn new(tree: &BasicPublisher, source: &String) -> Self {
        let node = tree.get_node(Key(0)); // 0 is root node
        let mut rc_tree = RuleCallTree {
            rules_keys: HashMap::new(),
            rules_rhs_keys: HashMap::new(),
            rules_referenced_by_rule: HashMap::new(),
            is_rule_left_recursive: HashMap::new(),
        };
        rc_tree.walk_node_children(tree, source, node);
        // println!("Rules -> Keys: {:#?}", rc_tree.rules_keys);
        // println!("Rules -> RHS Keys: {:#?}", rc_tree.rules_rhs_keys);
        println!(
            "Rules -> Referenced Rules: {:#?}",
            rc_tree.rules_referenced_by_rule
        );
        rc_tree.repeated_loop_over_referenced_rules();
        println!(
            "Rules -> IsLeftRecursive: {:#?}",
            rc_tree.is_rule_left_recursive
        );
        rc_tree.get_left_most_called_rules_of_all_rules(tree, source);
        rc_tree
    }

    fn get_left_most_called_rules_of_all_rules(&mut self, tree: &BasicPublisher, source: &String) {
        let mut rule_strings = Vec::new();
        for (rule, key) in &self.rules_keys {
            match self.is_rule_left_recursive.get(rule){
                None => {}
                Some(lr) => {
                    // Ignore rules we already know are NOT left recursive.
                    if *lr == LeftRecursive::False{
                        continue;
                    }
                }
            }
            rule_strings.push(rule.clone());
        }
        for rule_name in rule_strings {
            let key = self.rules_rhs_keys[&rule_name];
            let node = tree.get_node(key);
            self.get_left_most_called_rules_of_rule(rule_name, &node, tree, source);
        }
    }

    fn get_left_most_called_rules_of_rule(&mut self, rule_name: String, node: &Node, tree: &BasicPublisher, source: &String) {
        // For a given rule we need to find the left most rule or rules
        // to investigate if they can then lead back to the original rule
        // Since that's what determines if a rule is left recursive.
        // A minor complication is that ordered choice can both lead to left recursion.
        // Subexpressions are another complication since they can be considered like an anonymous subrule.
        // This is why there can be multiple rules that are the leftmost referenced rule at runtime.
        println!("Rule Name: {:?}", rule_name);
        let node_children = node.get_children();
        for child in node_children {
            let child_node = tree.get_node(*child);
            match child_node.rule
            
        }
    }

    fn repeated_loop_over_referenced_rules(&mut self) {
        // First pass is to mark all rules that reference no rules as not left Recursive
        // Then we keep looping marking all rules that references only rules that reference no rules as not left recursive.
        // Eventually we are left with only rules that reference other rules that reference other rules(sometimes themselves)
        // So they may be left recursive(can also be right recursive though. This is just an initial simple pass).
        // To get simpler rules out of the way. 
        let mut l = usize::MAX;
        println!(
            "Length of is_rule_lr: {:?} {:?}",
            self.is_rule_left_recursive.len(),
            l != self.is_rule_left_recursive.len()
        );
        while l != self.is_rule_left_recursive.len() {
            // println!("Looped");
            l = self.is_rule_left_recursive.len();
            // If the below call adds more rules as left recursive we loop again
            // If it does not we break.
            self.loop_over_referenced_rules();
        }
    }

    fn loop_over_referenced_rules(&mut self) {
        for (rule_name, referenced_rules) in &self.rules_referenced_by_rule {
            if referenced_rules.is_empty() {
                self.is_rule_left_recursive
                    .insert(rule_name.to_string(), LeftRecursive::False);
            } else {
                // It has referenced rules.
                let mut are_all_referenced_rules_not_left_recursive = true;
                for ref_rule in referenced_rules {
                    let is_lr = self.is_rule_left_recursive.get(ref_rule);
                    match is_lr {
                        None => {
                            // We don't know yet, so we have to assume it could be
                            are_all_referenced_rules_not_left_recursive = false;
                        }
                        Some(lr) => {
                            // Referenced rule is LR so we do nothing.
                            match lr {
                                LeftRecursive::False => {
                                    // If it's false it's fine and we can continue
                                }
                                _ => {
                                    // Otherwise involved in LR somehow so false
                                    are_all_referenced_rules_not_left_recursive = false;
                                }
                            }
                        }
                    }
                }
                if are_all_referenced_rules_not_left_recursive {
                    self.is_rule_left_recursive
                        .insert(rule_name.to_string(), LeftRecursive::False);
                }
            }
        }
    }

    fn get_rule_rhs_index(node: &Node) -> Key {
        // We get the index of rhs for the rule not the name because this is the actual graph
        // the LHS and the rule node are just syntax for setting up the grammar.
        debug_assert_eq!(node.rule, Rules::Rule);
        let rhs = node.get_children()[1];
        rhs
    }

    fn get_rules_name(tree: &BasicPublisher, source: &String, node: &Node) -> String {
        debug_assert_eq!(node.rule, Rules::Rule);

        let lhs = node.get_children()[0];
        let lhs = tree.get_node(lhs);
        debug_assert_eq!(lhs.rule, Rules::LHS);

        let var_name_decl = lhs.get_children()[0];
        let var_name_decl = tree.get_node(var_name_decl);
        debug_assert_eq!(var_name_decl.rule, Rules::Var_Name_Decl);
        let s = var_name_decl.get_string(&source);
        s[1..s.len() - 1].to_ascii_lowercase().to_string()
    }

    fn get_rule_ref_name(source: &String, node: &Node) -> String {
        debug_assert_eq!(node.rule, Rules::Var_Name_Ref);
        let s = node.get_string(&source);
        s[1..s.len() - 1].to_ascii_lowercase().to_string()
    }

    fn get_rules_referenced_by_rule<'a>(
        tree: &BasicPublisher,
        source: &String,
        node: &Node,
        referenced_rules: &'a mut HashSet<String>,
    ) -> &'a mut HashSet<String> {
        let node_children = node.get_children();
        for child in node_children {
            let child_node = tree.get_node(*child);
            match child_node.rule {
                Rules::Var_Name_Ref => {
                    // The Key for Rule references.
                    referenced_rules.insert(Self::get_rule_ref_name(source, child_node));
                }
                _ => {}
            }
            Self::get_rules_referenced_by_rule(tree, source, child_node, referenced_rules);
        }
        referenced_rules
    }

    fn walk_node_children(&mut self, tree: &BasicPublisher, source: &String, node: &Node) {
        let node_children = node.get_children();
        for child in node_children {
            let child_node = tree.get_node(*child);
            match child_node.rule {
                Rules::Rule => {
                    // The Key for the Rule itself
                    let rule_name = Self::get_rules_name(tree, source, child_node);
                    self.rules_keys.insert(rule_name.clone(), *child);
                    let rules_rhs_index = Self::get_rule_rhs_index(child_node);
                    self.rules_rhs_keys
                        .insert(rule_name.clone(), rules_rhs_index);
                    let mut rules_referenced_by_rule: HashSet<String> = HashSet::new();
                    Self::get_rules_referenced_by_rule(
                        tree,
                        source,
                        child_node,
                        &mut rules_referenced_by_rule,
                    );
                    self.rules_referenced_by_rule
                        .insert(rule_name, rules_referenced_by_rule);
                }
                _ => {}
            }
            self.walk_node_children(tree, source, child_node);
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
    fn test_var_name_no_lr() {
        let string = r##"<Var_Name> Inline = (<Alphabet_Lower>/<Alphabet_Upper>),(<Alphabet_Lower>/<Alphabet_Upper>/'_')*;
        <Alphabet_Upper> Inline = ['A'..'Z']; #We all love commments#
        <Alphabet_Lower> Inline =['a'..'z'];"##.to_string();
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
    fn test_var_name_lr() {
        let string = r##"<Num> = [0x30..0x39];
        <test_term> = (<test_term>, '+', <test_fact>)/(<test_term>, '-', <test_fact>)/<test_fact>;
<test_fact> = (<test_fact>, '*', <Num>)/(<test_fact>, '/', <Num>)/<Num>;
"##
        .to_string();
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
