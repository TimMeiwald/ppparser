use std::collections::{HashMap, HashSet};

use parser::{Node, Rules};

use crate::{BasicPublisher, Key};


#[derive(Debug, PartialEq, Clone, Copy)]
enum LeftRecursive {
    True,
    False,
}

/// For each rule we must detect whether there is a cycle to
/// determine if there is left recursion.
/// We then store this information for generatoion time
pub struct LeftRecursionDetector<'a> {
    source: &'a String,
    rules_name_map: HashMap<String, Key>, // Lookup a key for a rule by the name of the rule.
    left_recursion_rules: HashMap<String, HashSet<String>>,
    rules_that_terminate_map: HashMap<String, LeftRecursive>
}

impl<'a> LeftRecursionDetector<'a> {
    pub fn new(tree: &BasicPublisher, source: &'a String) -> Self {
        let mut lr_detector = LeftRecursionDetector {
            source,
            rules_name_map: HashMap::new(),
            left_recursion_rules: HashMap::new(),
            rules_that_terminate_map: HashMap::new(),
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
        

        lr_detector.get_rule_keys(tree); // We get the declared rules and their keys
                                         // So we can then lookup the rule in the tree when it's referenced in a different rule
        lr_detector.left_walk_init(tree);
        lr_detector.remove_terminals(tree);
        lr_detector
    }
    fn remove_terminals(&mut self, tree: &BasicPublisher){
        for (terminal_rule, value) in &self.rules_that_terminate_map{
            debug_assert_eq!(*value, LeftRecursive::False);
            for (lr_rule, set) in &mut self.left_recursion_rules{
                set.remove(terminal_rule); // Remove from sets if the rule is terminal.
            }

        }
        // Remove from self.left_recursion_rules if rule is terminal.
        for (terminal_rule, value) in &self.rules_that_terminate_map{
            self.left_recursion_rules.remove(terminal_rule);
        }

    }
    fn print_rules_name_map(&self) {
        println!("{:#?}", self.rules_name_map);
    }
    fn print_rules_that_terminate_map(&self) {
        println!("{:#?}", self.rules_that_terminate_map);
    }
    fn print_left_recursive_rules(&self) {
        println!("{:#?}", self.left_recursion_rules);
    }
    pub fn get_left_recursion_rules(&self) -> &HashMap<String, HashSet<String>> {
        return &self.left_recursion_rules;
    }
    fn get_rule_name(&mut self, tree: &BasicPublisher, node: &Node) -> String {
        debug_assert_eq!(node.rule, Rules::LHS);
        let var_name_decl = node.get_children()[0];
        let var_name_decl = tree.get_node(var_name_decl);
        debug_assert_eq!(var_name_decl.rule, Rules::Var_Name_Decl);
        let s = var_name_decl.get_string(&self.source);
        s[1..s.len() - 1].to_ascii_lowercase().to_string()
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
                    // self.rules_name_map.insert(rule_name, *key);
                    // We actually want the RHS not the full rule index so let's get that
                    // So it can act as a full graph of any given rule(since a dependency on another rule
                    // is part of that same graph).
                    let rhs_key = child.get_children()[1];
                    let rhs = tree.get_node(rhs_key);
                    debug_assert_eq!(rhs.rule, Rules::RHS);
                    self.rules_name_map.insert(rule_name, rhs_key);
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
            let child_name = child.get_string(&self.source);
            let mut rules_set: HashSet<String> = HashSet::new();
            match child.rule {
                Rules::Rule => {
                    debug_assert_eq!(child.rule, Rules::Rule);
                    let lhs = child.get_children()[0];
                    let lhs = tree.get_node(lhs);
                    let parent_rule_name = self.get_rule_name(tree, lhs);
                    println!("RULE START: {}", parent_rule_name);
                    // println!("\nRule: {:?}", parent_rule_name);
                    let rhs_key = child.get_children()[1];
                    let rhs = tree.get_node(rhs_key);
                    debug_assert_eq!(rhs.rule, Rules::RHS);
                    let lr = self.left_walk_rule_init(
                        tree,
                        rhs_key,
                        parent_rule_name.clone(),
                        &mut rules_set,
                    );
                    if lr == LeftRecursive::False{
                        self.rules_that_terminate_map.insert(parent_rule_name.clone(), lr);
                    }
                    println!("RULE EXIT: {}, Left Recursive = {:?}\n", parent_rule_name.clone(), lr);
                }
                _ => {}
            }
        }
    }

    fn left_walk_rule_init(
        &mut self,
        tree: &BasicPublisher,
        key: Key,
        parent_rule_name: String,
        mut rules_set: &mut HashSet<String>,
    ) -> LeftRecursive {
        // We want to take every viable left recursive path at the top rule level
        // But not thereafter(I think!)
        let node = tree.get_node(key);
        let mut lr: Option<LeftRecursive> = None;
        for child in node.get_children() {
            let child_node = tree.get_node(*child);
            println!("Left Walk Rule Init: Evaluating {:?}", child_node.rule);
            match child_node.rule {
                Rules::ASCII | Rules::OrderedChoiceMatchRange | Rules::Terminal => {
                    // We know these can not lead to rule references.
                    println!("Left Walk Rule Init: Ignoring {:?}", child_node.rule);
                }
                Rules::Var_Name_Ref => {
                    println!(
                        "Left Walk Rule Init: Entering '{}'",
                        child_node.get_string(self.source)
                    );
                    lr = Some(self.left_walk_kernel(tree, *child, parent_rule_name.clone(), rules_set));
                    break;
                }
                Rules::Ordered_Choice => {
                    let children = child_node.get_children();
                    let lhs = children[0];
                    let rhs = children[1];
                    println!("Left Walk Rule Init: Entering Ordered Choice 1");
                    lr = Some(self.left_walk_rule_init(tree, lhs, parent_rule_name.clone(), rules_set));
                    if lr == Some(LeftRecursive::True){
                        break;
                    }
                    println!("Left Walk Rule Init: Entering Ordered Choice 2");
                    lr = Some(self.left_walk_rule_init(tree, rhs, parent_rule_name.clone(), rules_set));
                    break;
                }
                Rules::Sequence => {
                    let children = child_node.get_children();
                    let lhs = children[0];
                    let rhs = children[1];
                    lr = Some(self.left_walk_rule_init(tree, lhs, parent_rule_name.clone(), rules_set));
                    // If LR is false we do the right part of the sequence
                    if lr == Some(LeftRecursive::False){
                        lr = Some(self.left_walk_rule_init(tree, rhs, parent_rule_name.clone(), rules_set));

                    }
                    break;
                }
                _ => {
                    println!("Left Walk Rule Init: Entering {:?}", child_node.rule);
                    lr = Some(self.left_walk_rule_init(tree, *child, parent_rule_name.clone(), rules_set));
                    break;
                }
            }
        }
        if lr.is_none() {
            // If nothing returns LeftRecursive::True then it must be False.
            return LeftRecursive::False;
        } else {
            return lr.unwrap();
        }
    }

    fn left_walk_kernel(
        &mut self,
        tree: &BasicPublisher,
        key: Key,
        parent_rule_name: String,
        mut rules_set: &mut HashSet<String>,
    ) -> LeftRecursive {
        let node = tree.get_node(key);
        debug_assert_eq!(node.rule, Rules::Var_Name_Ref);
        let ref_name = node.get_string(&self.source);
        let ref_name = ref_name[1..ref_name.len() - 1]
            .to_ascii_lowercase()
            .to_string();
        rules_set.insert(ref_name.clone()); // Always insert the rule
        if ref_name == parent_rule_name {
            // If ref name is the same as the parent rule then we have looped and can stop.
            self.left_recursion_rules
                .insert(parent_rule_name, rules_set.clone());
            return LeftRecursive::True;
        }

        let rule_name = node.get_string(&self.source);
        let rule_name = rule_name[1..rule_name.len() - 1]
            .to_ascii_lowercase()
            .to_string();
        let key = self
            .rules_name_map
            .get(&rule_name)
            .expect("The index should exist. If it doesn't the program is broken. Or the rule doesn't exist which cannot be handled. Todo: Bubble errors up properly at some point");
        // println!("Jumping to Rule: {:?}", node.rule);
        // If it's a reference to a rule then we jump to that rule's index and keep recursing.
        println!("Entering Rule Name: {:?}", rule_name);
        let result = self.left_walk_rule_init(tree, *key, parent_rule_name, rules_set);
        println!("Exiting Rule Name: {:?}", rule_name);
        return result;
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
        _lr_detector.print_rules_that_terminate_map();
        _lr_detector.print_left_recursive_rules();
        let r = _lr_detector.get_left_recursion_rules();
        assert!(r.len() == 0);
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
        _lr_detector.print_rules_that_terminate_map();
        _lr_detector.print_left_recursive_rules();
        let r = _lr_detector.get_left_recursion_rules();
        assert!(r.len() == 2);
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
        _lr_detector.print_rules_that_terminate_map();
        _lr_detector.print_left_recursive_rules();
        let r = _lr_detector.get_left_recursion_rules();
        assert!(r.len() == 4);
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
        _lr_detector.print_rules_that_terminate_map();
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
        _lr_detector.print_rules_that_terminate_map();
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
