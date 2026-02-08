use crate::{BasicPublisher, Key, Node, Rules};
use std::collections::{HashMap, HashSet};
pub struct LeftRecursionDetector {
    rule_call_tree: RuleCallTree,
}
impl LeftRecursionDetector {
    pub fn new(tree: &BasicPublisher, source: &String) -> Self {
        LeftRecursionDetector {
            rule_call_tree: RuleCallTree::new(tree, source),
        }
    }
    pub fn get_left_recursion_rules(&self) -> &HashMap<String, HashSet<String>> {
        &self.rule_call_tree.involved_sets
    }
}

#[derive(Debug, PartialEq, Clone)]
enum LeftRecursive {
    False, // Not left recursive
}

struct RuleCallTree {
    rules_keys: HashMap<String, Key>,
    rules_rhs_keys: HashMap<String, Key>,
    rules_referenced_by_rule: HashMap<String, HashSet<String>>,
    is_rule_left_recursive: HashMap<String, LeftRecursive>,
    rules_left_most_rule_refs: HashMap<String, HashSet<String>>,
    involved_sets: HashMap<String, HashSet<String>>,
}
impl RuleCallTree {
    fn new(tree: &BasicPublisher, source: &String) -> Self {
        let node = tree.get_node(Key(0)); // 0 is root node
        let mut rc_tree = RuleCallTree {
            rules_keys: HashMap::new(),
            rules_rhs_keys: HashMap::new(),
            rules_referenced_by_rule: HashMap::new(),
            is_rule_left_recursive: HashMap::new(),
            rules_left_most_rule_refs: HashMap::new(),
            involved_sets: HashMap::new(),
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
            "Rules -> IsLeftRecursive First Time: {:#?}",
            rc_tree.is_rule_left_recursive
        );
        rc_tree.get_left_most_called_rules_of_all_rules(tree, source);
        println!(
            "Rules -> Left Most Rules: {:#?}",
            rc_tree.rules_left_most_rule_refs
        );
        // If all left most rules are also not left recursive then we know that rule isn't left recursive either.
        rc_tree.repeated_loop_over_cleanup_left_most_called_rules();
        println!(
            "Rules -> IsLeftRecursive: {:#?}",
            rc_tree.is_rule_left_recursive
        );
        rc_tree.create_involved_sets();
        println!("Rules -> InvolvedSets: {:#?}", rc_tree.involved_sets);
        rc_tree.merge_dependent_sets_loop_until_no_change();

        println!(
            "Rules -> InvolvedSets After Merge: {:#?}",
            rc_tree.involved_sets
        );

        // Copy over the left most rules but ignore any that aren't left recursive
        // These appear to be almost the correct involved_set.
        // Need to analyse their behaviour.
        // By making a tree to see how behaviour loops.
        // E.g sub_expr only calls expr_addsub which can call sub_expr so that's clearly a loop.
        // Each involved set can include the calling rule(I think)

        rc_tree
    }
    fn involved_rule_count(&mut self) -> usize {
        let mut count = 0;
        for set in self.involved_sets.values() {
            count += set.len();
        }
        count
    }

    fn merge_dependent_sets_loop_until_no_change(&mut self) {
        let mut last_value = usize::MAX;
        let mut current_value = self.involved_rule_count();
        while current_value != last_value {
            last_value = current_value;
            self.merge_dependent_sets();
            current_value = self.involved_rule_count();
        }
    }
    fn merge_dependent_sets(&mut self) {
        // If a rule depends on a rule that is itself left recursive then the involved set
        // Includes it's involved sets.
        let copied_involved_sets = self.involved_sets.clone();
        for (rule_name, involved_set) in &copied_involved_sets {
            let mut copied_set = involved_set.clone();

            for subrule_name in involved_set {
                let subrule_involved_set = copied_involved_sets
                    .get(subrule_name)
                    .unwrap_or_else(|| panic!("Rule: {subrule_name:?} should exist"));
                copied_set = copied_set.union(subrule_involved_set).cloned().collect();
            }
            self.involved_sets.insert(rule_name.clone(), copied_set);
        }
    }

    fn create_involved_sets(&mut self) {
        for (rule_name, left_most_rules) in &self.rules_left_most_rule_refs {
            let mut involved_set: HashSet<String> = HashSet::new();
            for rule in left_most_rules {
                let is_lr = self.is_rule_left_recursive.get(rule);
                match is_lr {
                    None => {
                        // Is LR, so insert rule
                        involved_set.insert(rule.to_string());
                    }
                    Some(lr) => {
                        if *lr != LeftRecursive::False {
                            involved_set.insert(rule.to_string());
                        }
                    }
                }
            }
            if !involved_set.is_empty() {
                involved_set.insert(rule_name.to_string()); // Insert self rule
                self.involved_sets
                    .insert(rule_name.to_string(), involved_set);
            }
        }
    }
    fn repeated_loop_over_cleanup_left_most_called_rules(&mut self) {
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
            self.cleanup_left_most_called_rules();
        }
    }

    fn cleanup_left_most_called_rules(&mut self) {
        for (rule_name, left_most_rule_refs) in &self.rules_left_most_rule_refs {
            // println!("Rule Name: {:#?}", rule_name);
            let mut is_not_left_recursive = true;
            for left_most_ref in left_most_rule_refs {
                // println!("Left most Ref: {:#?}", left_most_ref);
                match self.is_rule_left_recursive.get(left_most_ref) {
                    None => {
                        // Must assume it could be LR
                        is_not_left_recursive = false;
                        // println!("Left most ref is None");
                    }
                    Some(lr) => {
                        match lr {
                            LeftRecursive::False => {
                                // Do nothing
                                // println!("Left most ref is LeftRecursive::False");
                            }
                        }
                    }
                }
            }
            if is_not_left_recursive {
                self.is_rule_left_recursive
                    .insert(rule_name.to_string(), LeftRecursive::False);
            }
        }
    }
    fn get_left_most_called_rules_of_all_rules(&mut self, tree: &BasicPublisher, source: &String) {
        let mut rule_strings = Vec::new();
        for rule in self.rules_keys.keys() {
            match self.is_rule_left_recursive.get(rule) {
                None => {}
                Some(lr) => {
                    // Ignore rules we already know are NOT left recursive.
                    if *lr == LeftRecursive::False {
                        continue;
                    }
                }
            }
            rule_strings.push(rule.clone());
        }
        for rule_name in rule_strings {
            let key = self.rules_rhs_keys[&rule_name];
            let node = tree.get_node(key);
            debug_assert!(node.rule == Rules::RHS);
            let mut left_most_rules: HashSet<String> = HashSet::new();
            println!("Rule Name: {:?}", rule_name.clone());
            Self::get_left_most_called_rules_of_rule(
                rule_name.clone(),
                node,
                tree,
                source,
                &mut left_most_rules,
            );
            self.rules_left_most_rule_refs
                .insert(rule_name.to_string(), left_most_rules);
        }
    }

    fn get_left_most_called_rules_of_rule(
        rule_name: String,
        node: &Node,
        tree: &BasicPublisher,
        source: &String,
        left_most_rules: &mut HashSet<String>,
    ) {
        // For a given rule we need to find the left most rule or rules
        // to investigate if they can then lead back to the original rule
        // Since that's what determines if a rule is left recursive.
        // A minor complication is that ordered choice can both lead to left recursion.
        // Subexpressions are another complication since they can be considered like an anonymous subrule.
        // This is why there can be multiple rules that are the leftmost referenced rule at runtime.
        let node_children = node.get_children();
        // println!("Inspecting {:?}", node.rule);
        for child in node_children {
            let child_node = tree.get_node(*child);
            match child_node.rule {
                Rules::Ordered_Choice => {
                    // We do not care about the rightmost rule in an ordered choice
                    // As whilst this could still lead to a loop it's the rule that
                    // is called's responsibility to handle the left recursion at that point.
                    let children = child_node.get_children();
                    let children = &children[..children.len() - 1];

                    for child_key in children {
                        let child = tree.get_node(*child_key);
                        // println!("Child: {:?} {:?}", child.rule, child_key);
                        Self::get_left_most_called_rules_of_rule(
                            rule_name.clone(),
                            child,
                            tree,
                            source,
                            left_most_rules,
                        );
                    }
                    return;
                }
                Rules::Sequence => {
                    let children = child_node.get_children();
                    // Sequence must have at least two children
                    let child = children[0];
                    let child = tree.get_node(child);
                    Self::get_left_most_called_rules_of_rule(
                        rule_name.clone(),
                        child,
                        tree,
                        source,
                        left_most_rules,
                    );
                    return;
                }
                Rules::Atom
                | Rules::RHS
                | Rules::Nucleus
                | Rules::Subexpression
                | Rules::Optional
                | Rules::Zero_Or_More
                | Rules::One_Or_More
                | Rules::And_Predicate
                | Rules::Not_Predicate => {
                    Self::get_left_most_called_rules_of_rule(
                        rule_name.clone(),
                        child_node,
                        tree,
                        source,
                        left_most_rules,
                    );
                    return;
                }
                Rules::Var_Name_Ref => {
                    let rule_ref = Self::get_rule_ref_name(source, child_node);
                    // println!(
                    //     "One Left Most Rule of {:?} is {:?}",
                    //     rule_ref, child_node.get_string(source)
                    // );
                    left_most_rules.insert(rule_ref);
                    return;
                }
                _ => {
                    // Do Nothing
                }
            }
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

    fn get_rules_name(tree: &BasicPublisher, source: &str, node: &Node) -> String {
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

    fn get_rule_ref_name(source: &str, node: &Node) -> String {
        debug_assert_eq!(node.rule, Rules::Var_Name_Ref);
        let s = node.get_string(source);
        s[1..s.len() - 1].to_string()
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

    fn walk_node_children(&mut self, tree: &BasicPublisher, source: &String, node: &Node) {
        let node_children = node.get_children();
        for child in node_children {
            let child_node = tree.get_node(*child);
            if child_node.rule == Rules::Rule {
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
        let context = BasicContext::new(src_len as usize, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let user_state = RefCell::new(UserState::new());
        let result = grammar(&user_state, Key(0), &context, &source, position);

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
        stdout().flush().expect("Why did it not flush");
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
        let context = BasicContext::new(src_len as usize, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let user_state = RefCell::new(UserState::new());
        let result = grammar(&user_state, Key(0), &context, &source, position);

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
        stdout().flush().expect("Why did it not flush");
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
        let string = r##"<Num> = [0x30..0x39];
        <test_LR_num> = <Num>;
        <test_indirect_three_level_A> = (<test_indirect_three_level_B>, '-', <test_LR_num>) / <test_LR_num>;
<test_indirect_three_level_B> = <test_indirect_three_level_C>;
<test_indirect_three_level_C> = <test_indirect_three_level_A>;"##
        .to_string();
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len as usize, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let user_state = RefCell::new(UserState::new());
        let result = grammar(&user_state, Key(0), &context, &source, position);

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
        stdout().flush().expect("Why did it not flush");
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
        let path = "../parser/Grammar.dsl";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len as usize, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let user_state = RefCell::new(UserState::new());
        let result = grammar(&user_state, Key(0), &context, &source, position);

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
        stdout().flush().expect("Why did it not flush");
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
        let path = "../examples/example_4_full_maths/example_4_full_maths.dsl";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len as usize, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let user_state = RefCell::new(UserState::new());
        let result = grammar(&user_state, Key(0), &context, &source, position);

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
        stdout().flush().expect("Why did it not flush");
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
    fn test_example_2() {
        let path = "../examples/example_2/example_2.dsl";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len as usize, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let user_state = RefCell::new(UserState::new());
        let result = grammar(&user_state, Key(0), &context, &source, position);

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
        stdout().flush().expect("Why did it not flush");
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
    fn test_example_3() {
        let path = "../examples/example_8_indirect_lr_3_level/LR_num_indirect_3_level.dsl";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len as usize, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let user_state = RefCell::new(UserState::new());
        let result = grammar(&user_state, Key(0), &context, &source, position);

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
        stdout().flush().expect("Why did it not flush");
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
