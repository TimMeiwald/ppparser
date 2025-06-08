use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use parser::{Node, Rules};

use crate::{BasicPublisher, Key};

#[derive(Debug, PartialEq, Clone)]
enum LeftRecursive {
    SelfCycleDetected,          // The rule itself is left recursive
    False,                      // Not left recursive
    OtherCycleDetected(String), // Cycles were detected but is not the rule we're investigating
}

// Temporary struct to clearly seperate jobs.
// This just gets which rules are left recursive
struct GetLeftRecursiveRules<'a> {
    source: &'a String,
    is_left_recursive_rule: HashMap<String, LeftRecursive>,
    rules_index_map: &'a HashMap<String, Key>,
    inverse_rules_index_map: &'a HashMap<Key, String>,
}
impl<'a> GetLeftRecursiveRules<'a> {
    pub fn new(
        tree: &BasicPublisher,
        source: &'a String,
        rules_index_map: &'a HashMap<String, Key>,
        inverse_rules_index_map: &'a HashMap<Key, String>,
    ) -> Self {
        let mut lr_detector = Self {
            source,
            is_left_recursive_rule: HashMap::new(),
            rules_index_map,
            inverse_rules_index_map,
        };
        lr_detector.evaluate_if_left_recursive(tree, rules_index_map);
        lr_detector
    }
    fn evaluate_if_left_recursive(
        &mut self,
        tree: &BasicPublisher,
        rules_index_map: &HashMap<String, Key>,
    ) {
        for (rule_name, rhs_index) in rules_index_map {
            let mut rule_evaluation_set: HashSet<Key> = HashSet::new();
            println!("\n\nEvaluation Started:  {} {:?}", rule_name, rhs_index);
            let mut should_return = LeftRecursive::False;
            self.kernel(
                *rhs_index,
                tree,
                &rules_index_map,
                &mut rule_evaluation_set,
                &mut should_return,
                rhs_index,
            );

            self.is_left_recursive_rule
                .insert(rule_name.clone(), should_return);
        }
    }

    fn jump_if_rule_reference(
        &self,
        key: Key,
        tree: &BasicPublisher,
        rules_index_map: &HashMap<String, Key>,
        rule_evaluation_set: &mut HashSet<Key>,
        should_return: &mut LeftRecursive,
        parent_rule_key: &Key,
    ) {
        let node = tree.get_node(key);

        if node.rule == Rules::Var_Name_Ref {
            let s = node.get_string(&self.source);
            let rule_name = s[1..s.len() - 1].to_ascii_lowercase().to_string();
            println!("Jumping to rule:    {:?}", rule_name);
            let rhs_key = rules_index_map.get(&rule_name).expect(&format!(
                "Cannot use function: {:?} because it does not exist.",
                rule_name
            ));
            self.kernel(
                *rhs_key,
                tree,
                rules_index_map,
                rule_evaluation_set,
                should_return,
                parent_rule_key,
            );
        }
    }

    /// Inserts key for RHS into rule_evaluation_set
    /// If it already exists return since we've already visited the rule so it's
    /// left recursive.
    /// Returns whether to return
    fn if_rhs_add_to_set(
        &self,
        key: Key,
        tree: &BasicPublisher,
        rule_evaluation_set: &mut HashSet<Key>,
        parent_rule_key: &Key,
    ) -> LeftRecursive {
        let node = tree.get_node(key);
        if node.rule == Rules::RHS {
            if !rule_evaluation_set.insert(key) {
                println!("Returning: {:?}", key);
                if key == *parent_rule_key {
                    return LeftRecursive::SelfCycleDetected;
                } else {
                    let rule_name = self
                        .inverse_rules_index_map
                        .get(&key)
                        .expect("Expect existence");
                    return LeftRecursive::OtherCycleDetected(rule_name.clone());
                }
            }
        }
        return LeftRecursive::False;
    }

    fn kernel(
        &self,
        key: Key,
        tree: &BasicPublisher,
        rules_index_map: &HashMap<String, Key>,
        rule_evaluation_set: &mut HashSet<Key>,
        mut should_return: &mut LeftRecursive,
        parent_rule_key: &Key,
    ) {
        // If Rule is RHS then we add to a set that we have visited a given node.
        *should_return = self.if_rhs_add_to_set(key, tree, rule_evaluation_set, parent_rule_key);
        if *should_return != LeftRecursive::False {
            return;
        }
        // If Rule is Var_Name_Ref we jump to that rule
        self.jump_if_rule_reference(
            key,
            tree,
            rules_index_map,
            rule_evaluation_set,
            should_return,
            parent_rule_key,
        );

        // Run for all children(not sure maybe only leftmost?)
        let node = tree.get_node(key);
        let children = node.get_children();
        let first_child = children.get(0);
        match first_child {
            Some(index) => {
                self.kernel(
                    *index,
                    tree,
                    rules_index_map,
                    rule_evaluation_set,
                    should_return,
                    parent_rule_key,
                );
            }
            None => {} // Terminal ignore
        }
        // for child in children{
        //     self.kernel(*child, tree, rules_index_map, rule_evaluation_set);
        // }
    }
}

struct GetInvolvedSets<'a> {
    tree: &'a BasicPublisher,
    source: &'a String,
    is_left_recursive_rule: &'a HashMap<String, LeftRecursive>,
    rules_index_map: &'a HashMap<String, Key>,
    inverse_rules_index_map: &'a HashMap<Key, String>,
    involved_sets: HashMap<String, HashSet<String>>,
}
impl<'a> GetInvolvedSets<'a> {
    fn new(
        tree: &'a BasicPublisher,
        source: &'a String,
        is_left_recursive_rule: &'a HashMap<String, LeftRecursive>,
        rules_index_map: &'a HashMap<String, Key>,
        inverse_rules_index_map: &'a HashMap<Key, String>,
    ) -> () {
        let mut slf = GetInvolvedSets {
            tree,
            source,
            is_left_recursive_rule,
            rules_index_map,
            inverse_rules_index_map,
            involved_sets: HashMap::new(),
        };
        slf.get_involved_sets();
    }

    fn get_involved_sets(&mut self) {
        for (rule_name, cycle_detected) in self.is_left_recursive_rule {
            match cycle_detected {
                LeftRecursive::False => {
                    // Do nothing since it's not left recursive.
                }
                LeftRecursive::SelfCycleDetected => {
                    // Find the involved set of rules for this self cycle.
                    let hash_set = self.get_involved_set(rule_name);
                    self.involved_sets.insert(rule_name.to_string(), hash_set);
                }
                LeftRecursive::OtherCycleDetected(_) => {
                    // todo!("Not yet implemented")
                }
            }
        }
    }

    fn get_involved_set(&self, rule_name: &String) -> HashSet<String> {
        let rule_key = self.rules_index_map.get(rule_name).expect("Should exist");
        // self.tree.print(*rule_key, Some(true));
        // println!("\n\n");
        let mut involved_set: HashSet<String> = HashSet::new();
        involved_set.insert(rule_name.clone());
        self.traverse(*rule_key, &mut involved_set);

        // Filters out rules that have no cycles.
        let involved_set = involved_set
            .into_iter()
            .filter(|r| {
                let s= self.is_left_recursive_rule.get(r);
                match s {
                    None => {panic!("Rule should exist in is_left_recursive_rule")}
                    Some(s) => {
                        match s {
                            LeftRecursive::False => return false,
                            _ => return true,
                        }
                    }
                }
                
            })
            .collect();
        
        println!("Involved Set for: {rule_name}\n{:?}\n\n", involved_set);
        involved_set
    }

    fn traverse(&self, node_key: Key, involved_set: &mut HashSet<String>) {
        let node = self.tree.get_node(node_key);
        let node_children = node.get_children();
        for child in node_children {
            let child_node = self.tree.get_node(*child);
            match child_node.rule {
                Rules::Ordered_Choice => {
                    let subchildren = child_node.get_children();
                    // We want to take every path through a top level ordered choice
                    // Since all could be the leftmost available path.
                    for child in subchildren {
                        self.traverse(*child, involved_set);
                    }
                }
                Rules::Sequence => {
                    let subchild = child_node
                        .get_children()
                        .get(0)
                        .expect("Must have at least two children.");
                    self.traverse(*subchild, involved_set);
                }
                Rules::Var_Name_Ref => {
                    self.jump_to_rule_reference(*child, involved_set);
                }
                Rules::Zero_Or_More
                | Rules::One_Or_More
                | Rules::Subexpression
                | Rules::And_Predicate
                | Rules::Optional
                | Rules::Not_Predicate
                | Rules::Atom
                | Rules::RHS
                | Rules::Nucleus => {
                    self.traverse(*child, involved_set);
                }
                Rules::Terminal
                | Rules::OrderedChoiceMatchRange
                | Rules::StringTerminal
                | Rules::ASCII => {
                    return;
                }

                _ => {
                    panic!("Unaccounted for rule: {:?}", child_node.rule)
                }
            }
        }
    }

    fn jump_to_rule_reference(&self, key: Key, involved_set: &mut HashSet<String>) {
        let node = self.tree.get_node(key);
        let s = node.get_string(&self.source);
        let rule_name = s[1..s.len() - 1].to_ascii_lowercase().to_string();
        println!("Jumping to rule:    {:?}", rule_name);
        let rhs_key = self.rules_index_map.get(&rule_name).expect(&format!(
            "Cannot use function: {:?} because it does not exist.",
            rule_name
        ));
        if involved_set.insert(rule_name) {
            // True if rule did not already exist in the set.
        } else {
            // False if the rule already existed in the set
            return; // So we don't loop endlessly.
        }
        self.traverse(*rhs_key, involved_set);
    }
}

/// For each rule we must detect whether there is a cycle to
/// determine if there is left recursion.
/// We then store this information for generatoion time
pub struct LeftRecursionDetector<'a> {
    source: &'a String,
    left_recursion_rules: HashMap<String, HashSet<String>>,
    is_left_recursive_rule: HashMap<String, LeftRecursive>,
}

impl<'a> LeftRecursionDetector<'a> {
    pub fn get_left_recursion_rules(&self) -> &HashMap<String, HashSet<String>> {
        &self.left_recursion_rules
    }
    pub fn new(tree: &BasicPublisher, source: &'a String) -> Self {
        // We assume tree is a true tree  with no cycles
        // The references between rules do not induce cycles in themselves
        // Since they are implicit and not part of the tree
        // Only by jumping to the next rule when we hit a Var_Name_Ref
        // does a stack overflow occur in left recursion(if you do not handle it correctly)
        // This does not hold for basic publishers that have not done
        // clear_false()

        let mut lr_detector = LeftRecursionDetector {
            source,
            left_recursion_rules: HashMap::new(),
            is_left_recursive_rule: HashMap::new(),
        };

        // Must build a graph of all rules from Grammar
        // Then order rules from lowest to top. So we can abort loops if
        // we know they already loop.
        // Then we can go to progessively higher tier rules.
        // And see if they loop back to themselves
        // This may work
        // Not sure if I need to work through every rule path or not
        // We care if keys are duplicated because they're pointers
        // Not the rule name per se
        // Ya silly goose
        // Still apply the above.
        let rules_index_map = lr_detector.get_rules_index_map(tree);
        let inverse_rules_index_map: HashMap<Key, String> = rules_index_map
            .iter()
            .map(|(k, v)| (v.clone(), k.clone()))
            .collect();
        let get_left_recursive =
            GetLeftRecursiveRules::new(tree, source, &rules_index_map, &inverse_rules_index_map);
        lr_detector.is_left_recursive_rule = get_left_recursive.is_left_recursive_rule.clone();

        println!(
            "Whether Rules are left recursive or not:\n{:#?}",
            lr_detector.is_left_recursive_rule
        );

        let rules: &HashMap<String, LeftRecursive> = &lr_detector.is_left_recursive_rule;
        let get_involved_sets = GetInvolvedSets::new(
            tree,
            source,
            &get_left_recursive.is_left_recursive_rule,
            &get_left_recursive.rules_index_map,
            &get_left_recursive.inverse_rules_index_map,
        );
        // Everytime a rule goes to a key in rules_index_map it's traversing a new rule
        // If it does it multiple times it could be a loop.
        // It could just be called multiple times if e.g it's a terminal.

        lr_detector
    }

    fn get_rule_rhs_index(&self, node: &Node) -> Key {
        // We get the index of rhs for the rule not the name because this is the actual graph
        // the LHS and the rule node are just syntax for setting up the grammar.
        debug_assert_eq!(node.rule, Rules::Rule);
        let rhs = node.get_children()[1];
        rhs
    }

    fn get_rule_name(&self, tree: &BasicPublisher, node: &Node) -> String {
        debug_assert_eq!(node.rule, Rules::Rule);

        let lhs = node.get_children()[0];
        let lhs = tree.get_node(lhs);
        debug_assert_eq!(lhs.rule, Rules::LHS);

        let var_name_decl = lhs.get_children()[0];
        let var_name_decl = tree.get_node(var_name_decl);
        debug_assert_eq!(var_name_decl.rule, Rules::Var_Name_Decl);
        let s = var_name_decl.get_string(&self.source);
        s[1..s.len() - 1].to_ascii_lowercase().to_string()
    }

    fn get_rules_index_map(&self, tree: &BasicPublisher) -> HashMap<String, Key> {
        let node = tree.get_node(Key(0));
        let mut map: HashMap<String, Key> = HashMap::new();
        debug_assert_eq!(node.rule, Rules::Grammar);
        for key in node.get_children() {
            let child = tree.get_node(*key);
            match child.rule {
                Rules::Rule => {
                    let rule_name = self.get_rule_name(tree, child);
                    let rule_rhs_index = self.get_rule_rhs_index(child);
                    map.insert(rule_name, rule_rhs_index);
                    // println!("{:?} {:?}", rule_name, rule_rhs_index);
                    // tree.print(rule_rhs_index, Some(true));
                }
                _ => {}
            }
        }
        map
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
    fn test_var_name_lr2() {
        let string = r##"<test_indirect_three_level_A> = (<test_indirect_three_level_B>, '-', <test_LR_num>) / <test_LR_num>;
<test_indirect_three_level_B> = <test_indirect_three_level_C>;
<test_indirect_three_level_C> = <test_indirect_three_level_A>;
<test_LR_num> = <Num>;
<Num> = [0x30..0x39];
<test_LR_expr> = (<test_LR_expr>, '-', <test_LR_num>) / <test_LR_num>; # Should match 0-0-0-0-0-0-0-0 etc #
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
