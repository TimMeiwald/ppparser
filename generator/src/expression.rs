use crate::{BasicPublisher, Key, Node, Rules};
use parser::publisher_trait::Publisher;
use std::collections::{HashMap, HashSet};
#[derive(Debug)]
pub struct Rule {
    root_key: Key,
    rhs_key: Key,
    name: String,
    rules_referenced_by_rule: HashSet<String>,
}
impl Rule {
    fn new(key: Key, tree: &BasicPublisher, source: &String) -> Rule {
        let root_node = tree.get_node(key);
        assert_eq!(root_node.rule, Rules::Rule);
        let rule_name = Rule::get_rules_name(root_node, tree, source);
        let rhs_key = Rule::get_rule_rhs_index(root_node);
        let mut referenced_rules: HashSet<String> = HashSet::new();
        Rule::get_rules_referenced_by_rule(tree, source, root_node, &mut referenced_rules);
        Rule {
            root_key: key,
            rhs_key,
            name: rule_name,
            rules_referenced_by_rule: referenced_rules,
        }
    }
    pub fn get_rhs_key(&self) -> Key {
        self.rhs_key
    }
    pub fn get_root_key(&self) -> Key {
        self.root_key
    }

    fn get_rules_name(node: &Node, tree: &BasicPublisher, source: &str) -> String {
        debug_assert_eq!(node.rule, Rules::Rule);

        let lhs = node.get_children()[0];
        let lhs = tree.get_node(lhs);
        debug_assert_eq!(lhs.rule, Rules::LHS);

        let var_name_decl = lhs.get_children()[0];
        let var_name_decl = tree.get_node(var_name_decl);
        debug_assert_eq!(var_name_decl.rule, Rules::Var_Name_Decl);
        let s = var_name_decl.get_string(source);
        s[1..s.len() - 1].to_string()
    }
    fn get_rule_rhs_index(node: &Node) -> Key {
        // We get the index of rhs for the rule not the name because this is the actual graph
        // the LHS and the rule node are just syntax for setting up the grammar.
        debug_assert_eq!(node.rule, Rules::Rule);
        let rhs = node.get_children()[1];
        rhs
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
            if child_node.rule == Rules::Var_Name_Ref {
                // The Key for Rule references.
                referenced_rules.insert(Self::get_rule_ref_name(source, child_node));
            }
            Self::get_rules_referenced_by_rule(tree, source, child_node, referenced_rules);
        }
        referenced_rules
    }
    fn get_rule_ref_name(source: &str, node: &Node) -> String {
        debug_assert_eq!(node.rule, Rules::Var_Name_Ref);
        let s = node.get_string(source);
        s[1..s.len() - 1].to_string()
    }
}

#[derive(Debug)]
pub struct RulesMap {
    rules: HashMap<String, Rule>,
}
impl<'a> IntoIterator for &'a RulesMap {
    type IntoIter = std::collections::hash_map::Iter<'a, String, Rule>;
    type Item = (&'a String, &'a Rule);
    fn into_iter(self) -> Self::IntoIter {
        self.rules.iter()
    }
}

impl RulesMap {
    pub fn new(key: Key, tree: &BasicPublisher, source: &String) -> Self {
        let rules_vec = Expression::get_rules_in_expression(key, tree, source);
        let mut rules_map: HashMap<String, Rule> = HashMap::with_capacity(rules_vec.len());
        // Check no duplicates
        for rule in rules_vec {
            let rule_name = rule.name.clone();
            let response = rules_map.insert(rule.name.clone(), rule);
            if response.is_none() {
                continue;
            } else {
                panic!(
                    "Duplicate rule:\nFirst Rule: \n{:?}\nSecond Rule: \n{:?}",
                    response,
                    rules_map.get(&rule_name)
                )
            }
        }
        // Check every referenced rule in a rule exists in the entire map.
        for (rule_name, rule) in &rules_map {
            for referenced_rule in &rule.rules_referenced_by_rule {
                assert!(
                    !rules_map.get(referenced_rule).is_none(),
                    "Rule: {rule_name:?} references {referenced_rule:?} which does not exist!."
                )
            }
        }
        RulesMap { rules: rules_map }
    }

    pub fn get_rule<S: AsRef<str>>(&self, name: S) -> Option<&Rule> {
        self.rules.get(name.as_ref())
    }
    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn get_cycle_detected_map(
        &self,
        tree: &BasicPublisher,
        source: &String,
    ) -> HashMap<String, bool> {
        let mut cycle_detected = HashMap::new();
        for (rule_name, rule) in self {
            cycle_detected.insert(
                rule_name.clone(),
                self.cycle_detector(rule.get_rhs_key(), tree, source),
            );
        }
        return cycle_detected;
    }

    pub fn get_always_returns_true_map(
        &self,
        tree: &BasicPublisher,
        source: &String,
    ) -> HashMap<String, bool> {
        let mut always_returns_true = HashMap::new();
        for (rule_name, rule) in self {
            always_returns_true.insert(
                rule_name.clone(),
                self.does_expression_always_returns_true(rule.get_rhs_key(), tree, source),
            );
        }
        return always_returns_true;
    }

    pub fn detect_left_recursion(
        &self,
        key: Key,
        tree: &BasicPublisher,
        source: &String,
        cycle_detected_rules: &HashMap<String, bool>,
        always_returns_true: &HashMap<String, bool>,
    ) -> bool {
        let mut set_of_keys_already_checked: HashMap<(u32, Key), bool> = HashMap::new();
        self._detect_left_recursion(
            key,
            tree,
            source,
            &mut set_of_keys_already_checked,
            cycle_detected_rules,
            always_returns_true,
        )
    }

    fn _detect_left_recursion(
        &self,
        key: Key,
        tree: &BasicPublisher,
        source: &String,
        set_of_keys_already_checked: &mut HashMap<(u32, Key), bool>,
        cycle_detected_rules: &HashMap<String, bool>,
        always_returns_true: &HashMap<String, bool>,
    ) -> bool {
        let node = tree.get_node(key);
        println!("Rule: {:?}, '{}'", node.rule, node.get_string(source));
        let node_children = node.get_children();
        let new_key: Key;

        // Handle a rule reference
        if node.rule == Rules::Var_Name_Ref {
            // If it's a variable name reference we call the function again
            // but with the new key's rhs key.
            let referenced_rule_name = Rule::get_rule_ref_name(source, node);
            new_key = self
                .get_rule(&referenced_rule_name)
                .expect("Should have been checked on construction")
                .rhs_key;
            // If  key exists
            if set_of_keys_already_checked.contains_key(&(node.start_position, new_key)) {
                // If checked and is True(i.e is left recursive) then it's true and we can return true
                // If checked and is False(i.e is not left recursive then it's false and we can return false)
                let already_checked = *set_of_keys_already_checked
                    .get(&(node.start_position, new_key))
                    .expect("Should exist we just checked.");
                return already_checked;
            } else {
                set_of_keys_already_checked.insert((node.start_position, new_key), true);
                let check_lr = self._detect_left_recursion(
                    new_key,
                    tree,
                    source,
                    set_of_keys_already_checked,
                    cycle_detected_rules,
                    always_returns_true,
                );
                set_of_keys_already_checked.insert((node.start_position, new_key), check_lr);
                return check_lr;
            }
        }

        // Handle any other case
        let ret = match node_children.len() {
            0 => {
                // If there are no children and it's not a reference then it's a terminal node
                // Which by definition cannot be left recursive.
                return false;
            }
            1 => {
                // Get child node and check if it's Left Recursive.
                let new_key = node_children[0];
                self._detect_left_recursion(
                    new_key,
                    tree,
                    source,
                    set_of_keys_already_checked,
                    cycle_detected_rules,
                    always_returns_true,
                )
            }
            _ => {
                // We only check the first element in a sequence that doesn't
                // always return true
                match node.rule {
                    // NEEDS TO BE REWRITTEN
                    Rules::Sequence => {
                        for child in node_children {
                            // Ignore if always return's true.
                            if self.does_expression_always_returns_true(*child, tree, source) {
                                continue;
                            }
                            // Check first in sequence that does not always return true.
                            return self._detect_left_recursion(
                                *child,
                                tree,
                                source,
                                set_of_keys_already_checked,
                                cycle_detected_rules,
                                always_returns_true,
                            );
                        }
                        panic!("A sequence must always have N >= 2 children.")
                    }
                    // We check all ordered choice paths as any can lead to LR at runtime.
                    Rules::Ordered_Choice => {
                        let mut ret: usize = 0;
                        for child in node_children {
                            // We check each child node to see if it's left recursive.
                            let child_ret = self._detect_left_recursion(
                                *child,
                                tree,
                                source,
                                set_of_keys_already_checked,
                                cycle_detected_rules,
                                always_returns_true,
                            );
                            if child_ret {
                                ret += 1;
                            }
                        }
                        match ret {
                            0 => false,
                            _ => true,
                        }
                    }
                    _ => {
                        panic!("Did not expect rule: {:?}", node.rule)
                    }
                }
            }
        };
        println!(
            "Rule: {:?} -> {ret}, '{}' ",
            node.rule,
            node.get_string(source)
        );
        ret
    }

    // fn _detect_left_recursion(
    //     &self,
    //     key: Key,
    //     tree: &BasicPublisher,
    //     source: &String,
    //     set_of_keys_already_checked: &mut HashMap<(u32, Key), bool>,
    //     cycle_detected_rules: &HashMap<String, bool>,
    //     always_returns_true: &HashMap<String, bool>,
    // ) -> bool {
    //     let node = tree.get_node(key);
    //     println!("Rule: {:?}, '{}'", node.rule, node.get_string(source));
    //     let node_children = node.get_children();
    //     let new_key: Key;

    //     if node.rule == Rules::Var_Name_Ref {
    //         // If it's a rule name reference we jump to that rule.
    //         let referenced_rule_name = Rule::get_rule_ref_name(source, node);
    //         if !cycle_detected_rules
    //             .get(&referenced_rule_name)
    //             .expect("Should exist.")
    //         {
    //             // If there is no cycle then it's definitely not LR and we can return.
    //             let ret = false;
    //             println!(
    //                 "Rule: {:?} -> {ret}, '{}' ",
    //                 node.rule,
    //                 node.get_string(source)
    //             );
    //             return ret;
    //         }
    //         if *always_returns_true
    //             .get(&referenced_rule_name)
    //             .expect("Should exist")
    //         {
    //             // If it always returns true then we ignore it. As it cannot be handled
    //             // with left recursion(not by this rule anyway.)
    //             let ret = false;
    //             println!(
    //                 "Rule: {:?} -> {ret}, '{}' ",
    //                 node.rule,
    //                 node.get_string(source)
    //             );
    //             return ret;
    //         }
    //         new_key = self
    //             .get_rule(&referenced_rule_name)
    //             .expect("Should have been checked on construction")
    //             .rhs_key;
    //         if set_of_keys_already_checked.contains_key(&(node.start_position, new_key)) {
    //             // If key already exists then we've already hit this rule once. Making it some form of cycle.
    //             let ret = true;
    //             println!(
    //                 "Rule: {:?} -> {ret}, '{}' ",
    //                 node.rule,
    //                 node.get_string(source)
    //             );
    //             return ret;
    //         }
    //         // Key does not exist so we insert it. The _cycle detector does not use the root node of the rule
    //         // but it's RHS node so it won't immediately trigger.
    //         set_of_keys_already_checked.insert((node.start_position, new_key), true);
    //         return false;
    //     }

    //     if node_children.len() == 0 {
    //         // If there are no children then it's a terminal.
    //         let ret = false;
    //         println!(
    //             "Rule NO CHILDREN: {:?} -> {ret}, '{}' ",
    //             node.rule,
    //             node.get_string(source)
    //         );
    //         return ret;
    //     } else if node_children.len() == 1 {
    //         let new_key: Key;
    //         // Otherwise we recurse into the child.
    //         new_key = node_children[0];
    //         let ret = self._detect_left_recursion(
    //             new_key,
    //             tree,
    //             source,
    //             set_of_keys_already_checked,
    //             cycle_detected_rules,
    //             always_returns_true,
    //         );
    //         println!(
    //             "Rule: {:?} -> {ret}, '{}' ",
    //             node.rule,
    //             node.get_string(source)
    //         );
    //         return ret;
    //     } else {
    //         /*
    //         If there are multiple children it's a sequence or an ordered choice.
    //         For Sequence we need to select the first non terminal that doesn't alway's return True to check.
    //         For Ordered Choice we need to check all options.
    //         */
    //         match node.rule {
    //             Rules::Sequence => {
    //                 let mut ret: usize = 0;
    //                 for child in node_children {
    //                     // Almost certainly a perf issue. Might need to add caching of some sort.
    //                     // We check each expression since a rule can be wrapped in e.g zero or more etc.
    //                     if self._detect_left_recursion(
    //                         *child,
    //                         tree,
    //                         source,
    //                         set_of_keys_already_checked,
    //                         cycle_detected_rules,
    //                         always_returns_true,
    //                     ) {
    //                         ret += 1;
    //                     }
    //                 }
    //                 let ret = match ret {
    //                     0 => false,
    //                     _ => true,
    //                 };
    //                 println!(
    //                     "Rule: {:?} -> {ret}, '{}' ",
    //                     node.rule,
    //                     node.get_string(source)
    //                 );
    //                 return ret;
    //             }
    //             Rules::Ordered_Choice => {
    //                 // We explore every path that could lead to LR.
    //                 let mut ret: usize = 0;
    //                 for child in node_children {
    //                     if self._detect_left_recursion(
    //                         *child,
    //                         tree,
    //                         source,
    //                         set_of_keys_already_checked,
    //                         cycle_detected_rules,
    //                         always_returns_true,
    //                     ) {
    //                         ret += 1;
    //                     }
    //                 }
    //                 let ret = match ret {
    //                     0 => false,
    //                     _ => true,
    //                 };
    //                 println!(
    //                     "Rule: {:?} -> {ret}, '{}' ",
    //                     node.rule,
    //                     node.get_string(source)
    //                 );
    //                 return ret;
    //             }
    //             _ => panic!("Node Rule: {:?} not yet supported", node.rule),
    //         }
    //     }
    // }

    pub fn cycle_detector(&self, key: Key, tree: &BasicPublisher, source: &String) -> bool {
        /*
        This solely attempts to determine if a given rule has any cycles at all
        Whilst it may be more efficient to get e.g Left Recursive rules directly
        This helps to validate that logic functions correctly. This also only happens
        at generation time of the parser so it's not a big problem.

        We also do this per rule, which again is less efficient but easier to test. Definitely
        has potential for optimization at some point.
        */

        // We track start_position and also whether it's been hit so we don't
        // trigger cycle detection just because a rule get's called multiple times in the same function.
        // E.g <some_rule> = <ws>, <ws>; should not trigger cycle detection unless <ws> somehow leads back to <some_rule>
        let mut set_of_keys_already_checked: HashMap<(u32, Key), bool> = HashMap::new();
        self._cycle_detector(key, tree, source, &mut set_of_keys_already_checked)
    }

    fn _cycle_detector(
        &self,
        key: Key,
        tree: &BasicPublisher,
        source: &String,
        set_of_keys_already_checked: &mut HashMap<(u32, Key), bool>,
    ) -> bool {
        let node = tree.get_node(key);
        let node_children = node.get_children();

        if node.rule == Rules::Var_Name_Ref {
            // Rule call
            let referenced_rule_name = Rule::get_rule_ref_name(source, node);
            let rule_rhs_key = self
                .get_rule(&referenced_rule_name)
                .expect("Should have been checked on construction")
                .rhs_key;
            if set_of_keys_already_checked.contains_key(&(node.start_position, rule_rhs_key)) {
                // If key already exists then we've already hit this rule once. Making it some form of cycle.
                return true;
            }
            // Key does not exist so we insert it. The _cycle detector does not use the root node of the rule
            // but it's RHS node so it won't immediately trigger.
            set_of_keys_already_checked.insert((node.start_position, rule_rhs_key), true);
            self._cycle_detector(rule_rhs_key, tree, source, set_of_keys_already_checked)
        } else if node_children.len() == 0 {
            return false; // Terminal node
        } else {
            // Multiple child nodes, iterate over them.
            let mut result = false;
            for child in node_children {
                result = self._cycle_detector(*child, tree, source, set_of_keys_already_checked);
                if result {
                    // If a cycle is detected we break.
                    break;
                }
            }
            result
        }
    }

    pub fn does_expression_always_returns_true(
        &self,
        key: Key,
        tree: &BasicPublisher,
        source: &String,
    ) -> bool {
        let mut set_of_keys_already_checked: HashMap<Key, Option<bool>> = HashMap::new(); // Since cycles are allowed to prevent stack overflow.
        self._does_expression_always_returns_true(
            key,
            tree,
            source,
            &mut set_of_keys_already_checked,
        )
    }

    fn _does_expression_always_returns_true(
        &self,
        key: Key,
        tree: &BasicPublisher,
        source: &String,
        set_of_keys_already_checked: &mut HashMap<Key, Option<bool>>, // Since cycles are allowed to prevent stack overflow.
    ) -> bool {
        if set_of_keys_already_checked.contains_key(&key) {
            let cached_result = *set_of_keys_already_checked
                .get(&key)
                .expect("Literally just checked it exists.");
            match cached_result {
                Some(value) => return value,
                None => set_of_keys_already_checked.insert(key, Some(false)), // First None causes bypass, Second will return false.
            };
        };

        let node = tree.get_node(key);
        let node_children = node.get_children();
        if node_children.len() == 0 {
            // Zero or more and optional or any expressions containing them
            // will always have more than 1 child.
            // No children means a terminal so always false.
            if node.rule == Rules::Var_Name_Ref {
                let referenced_rule_name = Rule::get_rule_ref_name(source, node);
                let rule_rhs_key = self
                    .get_rule(&referenced_rule_name)
                    .expect("Should have been checked on construction")
                    .rhs_key;
                // Set this to false first so it's not infinitely recursing.
                if !set_of_keys_already_checked.contains_key(&key) {
                    set_of_keys_already_checked.insert(key, None);
                }
                let result = self._does_expression_always_returns_true(
                    rule_rhs_key,
                    tree,
                    source,
                    set_of_keys_already_checked,
                );
                // Now we override the result with the real value.
                set_of_keys_already_checked.insert(key, Some(result));
                return result;
            }
            set_of_keys_already_checked.insert(key, Some(false));
            return false;
        } else if node_children.len() == 1 {
            let child = node_children[0];
            let child_node = tree.get_node(child);
            if child_node.rule == Rules::Zero_Or_More || child_node.rule == Rules::Optional {
                set_of_keys_already_checked.insert(key, Some(true));
                return true;
            } else {
                let result = self._does_expression_always_returns_true(
                    child,
                    tree,
                    source,
                    set_of_keys_already_checked,
                );
                set_of_keys_already_checked.insert(key, Some(result));
                return result;
            }
        } else {
            if node.rule == Rules::StringTerminal {
                set_of_keys_already_checked.insert(key, Some(false));
                return false;
            }
            // If all results are true then we return true.
            let mut result: bool = true;
            for child in node_children {
                result = self._does_expression_always_returns_true(
                    *child,
                    tree,
                    source,
                    set_of_keys_already_checked,
                );
                if result == false {
                    // If one sequence option doesn't always return true the whole rule doesn't.
                    break;
                }
            }
            set_of_keys_already_checked.insert(key, Some(result));
            result
        }
    }
}

struct Expression {}
impl Expression {
    fn get_rules_in_expression(key: Key, tree: &BasicPublisher, source: &String) -> Vec<Rule> {
        let rule_keys = Expression::find_rule_keys_in_expression(key, tree, source);
        rule_keys
            .iter()
            .map(|key| Rule::new(*key, tree, source))
            .collect()
    }

    fn find_rule_keys_in_expression(key: Key, tree: &BasicPublisher, source: &String) -> Vec<Key> {
        let mut return_vec: Vec<Key> = Vec::new();
        Expression::_find_rule_keys_in_expression(key, tree, source, &mut return_vec);
        return_vec
    }
    fn _find_rule_keys_in_expression(
        key: Key,
        tree: &BasicPublisher,
        source: &String,
        return_vec: &mut Vec<Key>,
    ) {
        let node = tree.get_node(key);
        for child in node.get_children() {
            let child_node = tree.get_node(*child);
            match child_node.rule {
                Rules::Rule => {
                    return_vec.push(*child);
                }
                _ => {
                    Expression::_find_rule_keys_in_expression(*child, tree, source, return_vec);
                }
            }
        }
    }
}

mod test {
    use super::*;
    use crate::count_lines;

    use ::parser::*;
    use std::cell::RefCell;

    use std::fs::{canonicalize, read_to_string};
    use std::io::stdout;
    use std::io::Write;

    fn shared(source: &str) -> ((bool, u32), BasicPublisher) {
        let src_len = source.len();
        let source = Source::new(&source);
        let position = 0;
        let context = BasicContext::new(src_len as usize, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let user_state = RefCell::new(UserState::new());
        let result = grammar(&user_state, Key(0), &context, &source, position);
        let tree = context.into_inner();
        let tree = tree.get_publisher().clear_false();
        (result, tree)
    }

    #[test]
    fn test_get_rules_in_expression() {
        let string = r##"<Num> = [0x30..0x39];
        <test_LR_num> = <Num>;
        <test_indirect_three_level_A> = (<test_indirect_three_level_B>, '-', <test_LR_num>) / <test_LR_num>;
<test_indirect_three_level_B> = <test_indirect_three_level_C>;
<test_indirect_three_level_C> = <test_indirect_three_level_A>;"##;

        let (result, publisher) = shared(string);
        let result = Expression::get_rules_in_expression(Key(0), &publisher, &string.to_string());
        println!("{result:#?}")
    }

    #[test]
    fn test_get_rules_in_expression2() {
        let string = r##"<Num> = <ws>, (<thing_one>/<thing_two>), <ws>;
                                <ws> = ' '*;"##;

        let (result, publisher) = shared(string);
        println!("{result:?}");
        assert!(result.0);
        let result = Expression::get_rules_in_expression(Key(0), &publisher, &string.to_string());
        println!("{result:#?}")
    }

    #[test]
    #[should_panic] // Two ws so it should panic.
    fn test_rules_map() {
        let string = r##"
                                <ws> = ' '*;
                                <ws> = ' '*;"##;

        let (result, publisher) = shared(string);
        println!("{result:?}");
        assert!(result.0);
        let result = RulesMap::new(Key(0), &publisher, &string.to_string());
        println!("{result:#?}")
    }

    #[test]
    #[should_panic] // No thing one or thing two rule so it panics
    fn test_rules_map2() {
        let string = r##"<Num> = <ws>, (<thing_one>/<thing_two>), <ws>;
                                <ws> = ' '*;"##;

        let (result, publisher) = shared(string);
        println!("{result:?}");
        assert!(result.0);
        let result = RulesMap::new(Key(0), &publisher, &string.to_string());
        println!("{result:#?}");
    }
    #[test]
    fn test_rules_map3() {
        let string = r##"<Num> = <ws>, (<thing_one>/<thing_two>), <ws>;
                                <ws> = ' '*;
                                <thing_one> = 'a';
                                <thing_two> = 'b';
                                "##;

        let (result, publisher) = shared(string);
        println!("{result:?}");
        assert!(result.0);
        let result = RulesMap::new(Key(0), &publisher, &string.to_string());
        println!("{result:#?}");
        assert!(result.len() == 4);
    }

    #[test]
    fn test_rules_expression_always_returns_true() {
        let string = r##"<Num> = <ws>, (<thing_one>/<thing_two>), <ws>;
                                <ws> = ' '*;
                                <indirect_ws> = <ws>;
                                <thing_one> = 'a';
                                <thing_two> = 'b';
                                "##;

        let (result, publisher) = shared(string);
        println!("{result:?}");
        assert!(result.0);
        let rules_map = RulesMap::new(Key(0), &publisher, &string.to_string());
        println!("{rules_map:#?}");
        assert!(rules_map.len() == 5);

        let rhs_key = rules_map.get_rule("ws").unwrap().rhs_key;
        let result =
            rules_map.does_expression_always_returns_true(rhs_key, &publisher, &string.to_string());
        assert_eq!(
            true, result,
            "ws is zero or more whitespace so it always returns true because of zero or more."
        );

        let rhs_key = rules_map.get_rule("thing_one").unwrap().rhs_key;
        let result =
            rules_map.does_expression_always_returns_true(rhs_key, &publisher, &string.to_string());
        assert_eq!(
            false, result,
            "thing_one is terminal so should return false"
        );

        let rhs_key = rules_map.get_rule("indirect_ws").unwrap().rhs_key;
        let result =
            rules_map.does_expression_always_returns_true(rhs_key, &publisher, &string.to_string());
        assert_eq!(
            true, result,
            "Only calls <ws> so indirectly always returns true so should be true"
        );

        let rhs_key = rules_map.get_rule("Num").unwrap().rhs_key;
        let result =
            rules_map.does_expression_always_returns_true(rhs_key, &publisher, &string.to_string());
        assert_eq!(false, result, "Num is basically thing_one or thing_two but ignoring whitespace before and after so should be false.")
    }

    #[test]
    fn test_rules_expression_always_returns_true_c_parser() {
        let path = "../examples/c_parser/c_from_spec.dsl";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");

        let (result, publisher) = shared(&string);
        println!("{result:?}");
        assert!(result.0);
        let rules_map = RulesMap::new(Key(0), &publisher, &string.to_string());
        println!("{rules_map:#?}");
        rules_map.does_expression_always_returns_true(Key(0), &publisher, &string);
    }
    #[test]
    fn test_rules_expression_always_returns_true_c_parser2() {
        let string = r##"<ws_kernel> Inline = (' '/'\t'/'\r'/'\n'); # Some whitespace are never relevant # 
            <ws> Inline = <ws_kernel>*;
            <expression> = <ws>,(<assignment_expression>/(<expression>, ',', <assignment_expression>)), <ws>;
            <assignment_expression> = "ah";
"##;
        let (result, publisher) = shared(&string);
        println!("{result:?}");
        assert!(result.0);
        let rules_map = RulesMap::new(Key(0), &publisher, &string.to_string());
        println!("{rules_map:#?}");

        let result = rules_map.does_expression_always_returns_true(
            rules_map.get_rule("ws").unwrap().rhs_key,
            &publisher,
            &string.to_string(),
        );
        println!("Does expression always return true: {result:?}");
        assert_eq!(result, true);

        let result = rules_map.does_expression_always_returns_true(
            rules_map.get_rule("ws_kernel").unwrap().rhs_key,
            &publisher,
            &string.to_string(),
        );
        println!("Does expression always return true: {result:?}");
        assert_eq!(result, false);

        let result = rules_map.does_expression_always_returns_true(
            rules_map.get_rule("expression").unwrap().rhs_key,
            &publisher,
            &string.to_string(),
        );
        println!("Does expression always return true: {result:?}");
        assert_eq!(result, false)
    }

    #[test]
    fn test_no_cycles() {
        let string = r##"<Num> = <ws>;
                                <ws> = ' '*;"##;

        let (result, publisher) = shared(string);
        println!("{result:?}");
        assert!(result.0);
        let rules_map = RulesMap::new(Key(0), &publisher, &string.to_string());
        println!("{rules_map:#?}");
        let num = rules_map.get_rule("Num").unwrap().get_rhs_key();
        assert!(!rules_map.cycle_detector(num, &publisher, &string.to_string()));
        let ws = rules_map.get_rule("ws").unwrap().get_rhs_key();
        assert!(!rules_map.cycle_detector(ws, &publisher, &string.to_string()));
    }

    #[test]
    fn test_no_cycles2() {
        let string = r##"<Num> = <ws>, (<thing_one>/<thing_two>), <ws>;
                                <ws> = ' '*;
                                <indirect_ws> = <ws>;
                                <thing_one> = 'a';
                                <thing_two> = 'b';
                                "##;
        let (result, publisher) = shared(string);
        println!("{result:?}");
        assert!(result.0);
        let rules_map = RulesMap::new(Key(0), &publisher, &string.to_string());
        println!("{rules_map:#?}");

        let key = rules_map.get_rule("Num").unwrap().get_rhs_key();
        assert!(!rules_map.cycle_detector(key, &publisher, &string.to_string()));
        let key = rules_map.get_rule("ws").unwrap().get_rhs_key();
        assert!(!rules_map.cycle_detector(key, &publisher, &string.to_string()));
        let key = rules_map.get_rule("thing_one").unwrap().get_rhs_key();
        assert!(!rules_map.cycle_detector(key, &publisher, &string.to_string()));
        let key = rules_map.get_rule("thing_two").unwrap().get_rhs_key();
        assert!(!rules_map.cycle_detector(key, &publisher, &string.to_string()));
        let key = rules_map.get_rule("indirect_ws").unwrap().get_rhs_key();
        assert!(!rules_map.cycle_detector(key, &publisher, &string.to_string()));
    }

    #[test]
    fn test_recursive_cycle() {
        // rr is right recursive
        // lr is left recursive
        // Both should be detected as recursive.
        let string = r##"<rr> = ('1', <rr>)/'1'; # rr ::= "1" <rr> / "1" #
                                <lr> = (<lr>, '1')/'1'; # lr ::= <lr> "1" / "1" #
                                "##;
        let (result, publisher) = shared(string);
        println!("{result:?}");
        assert!(result.0);
        let rules_map = RulesMap::new(Key(0), &publisher, &string.to_string());
        println!("{rules_map:#?}");

        let key = rules_map.get_rule("rr").unwrap().get_rhs_key();
        assert!(rules_map.cycle_detector(key, &publisher, &string.to_string()));
        let key = rules_map.get_rule("lr").unwrap().get_rhs_key();
        assert!(rules_map.cycle_detector(key, &publisher, &string.to_string()));
    }

    #[test]
    fn test_recursive_cycle2() {
        // rr is right recursive
        // lr is left recursive
        // Both should be detected as recursive.
        let string = r##"<rr> = <lr>, <rr>;
                                <lr> = <lr>, <rr>;
                                "##;
        let (result, publisher) = shared(string);
        println!("{result:?}");
        assert!(result.0);
        let rules_map = RulesMap::new(Key(0), &publisher, &string.to_string());
        println!("{rules_map:#?}");

        let key = rules_map.get_rule("rr").unwrap().get_rhs_key();
        assert!(rules_map.cycle_detector(key, &publisher, &string.to_string()));
        let key = rules_map.get_rule("lr").unwrap().get_rhs_key();
        assert!(rules_map.cycle_detector(key, &publisher, &string.to_string()));
    }

    #[test]
    fn test_left_recursion_detection1() {
        // rr is right recursive
        // lr is left recursive
        // Both should be detected as recursive.
        let string = r##"<rr> = ('1', <rr>)/'1'; # rr ::= "1" <rr> / "1" #
                                <lr> = (<lr>, '1')/'1'; # lr ::= <lr> "1" / "1" #
                                <ws> = ' '*;
                                "##;

        let (result, publisher) = shared(string);
        let string = &string.to_string();
        println!("{result:?}");
        assert!(result.0);
        let rules_map = RulesMap::new(Key(0), &publisher, string);
        println!("{rules_map:#?}");
        let cycles_detected = rules_map.get_cycle_detected_map(&publisher, string);
        let always_true = rules_map.get_always_returns_true_map(&publisher, string);
        println!("Cycles Detected: {cycles_detected:#?}");
        println!("Always True: {always_true:#?}");

        let key = rules_map.get_rule("rr").unwrap().get_rhs_key();
        let rr = rules_map.detect_left_recursion(
            key,
            &publisher,
            string,
            &cycles_detected,
            &always_true,
        );
        println!("Rule: rr, LR Detected: {rr}");
        assert!(
            rules_map.cycle_detector(key, &publisher, &string.to_string()),
            "rr should have detectable cycles!"
        );
        assert!(!rr, "rr should not be detected as left recursive");

        println!("###############");
        let key = rules_map.get_rule("lr").unwrap().get_rhs_key();
        let lr = rules_map.detect_left_recursion(
            key,
            &publisher,
            string,
            &cycles_detected,
            &always_true,
        );
        println!("Rule: lr, LR Detected: {lr}");
        assert!(
            rules_map.cycle_detector(key, &publisher, &string.to_string()),
            "lr should have detectable cycles!"
        );
        assert!(lr, "lr should be detected as left recursive");

        let key = rules_map.get_rule("ws").unwrap().get_rhs_key();
        let ws = rules_map.detect_left_recursion(
            key,
            &publisher,
            string,
            &cycles_detected,
            &always_true,
        );
        println!("Rule: ws, LR Detected: {ws}");
        assert!(!ws)
    }

    #[test]
    fn test_left_recursion_detection2() {
        // rr is right recursive
        // lr is left recursive
        // Both should be detected as recursive.
        let string = r##"<rr> = ('1', <rr>)/'1'; # rr ::= "1" <rr> / "1" #
                                <lr> = ('1'/<lr>), <rr>;
                                <ws> = ' '*;
                                "##;

        let (result, publisher) = shared(string);
        let string = &string.to_string();
        println!("{result:?}");
        assert!(result.0);
        let rules_map = RulesMap::new(Key(0), &publisher, string);
        println!("{rules_map:#?}");
        let cycles_detected = rules_map.get_cycle_detected_map(&publisher, string);
        let always_true = rules_map.get_always_returns_true_map(&publisher, string);
        println!("Cycles Detected: {cycles_detected:#?}");
        println!("Always True: {always_true:#?}");

        let key = rules_map.get_rule("rr").unwrap().get_rhs_key();
        let rr = rules_map.detect_left_recursion(
            key,
            &publisher,
            string,
            &cycles_detected,
            &always_true,
        );
        println!("Rule: rr, LR Detected: {rr}");
        assert!(
            rules_map.cycle_detector(key, &publisher, &string.to_string()),
            "rr should have detectable cycles!"
        );
        assert!(!rr, "rr should not be detected as left recursive");

        println!("###############");
        let key = rules_map.get_rule("lr").unwrap().get_rhs_key();
        let lr = rules_map.detect_left_recursion(
            key,
            &publisher,
            string,
            &cycles_detected,
            &always_true,
        );
        println!("Rule: lr, LR Detected: {lr}");
        assert!(
            rules_map.cycle_detector(key, &publisher, &string.to_string()),
            "lr should have detectable cycles!"
        );
        assert!(lr, "lr should be detected as left recursive");

        let key = rules_map.get_rule("ws").unwrap().get_rhs_key();
        let ws = rules_map.detect_left_recursion(
            key,
            &publisher,
            string,
            &cycles_detected,
            &always_true,
        );
        println!("Rule: ws, LR Detected: {ws}");
        assert!(!ws)
    }
}
