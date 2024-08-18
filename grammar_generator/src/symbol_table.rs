use publisher::{Publisher, Tree};
use rules::{rules::Rules, Key};
use std::{collections::HashMap, panic};
pub struct SymbolTable<'a> {
    names: Vec<String>,
    source: &'a String,
    inlined_rules: HashMap<String, String>,
}

impl<'a> SymbolTable<'a> {
    pub fn new(tree: &Tree, source: &'a String) -> Self {
        let mut sym_table = SymbolTable {
            names: Vec::<String>::new(),
            source,
            inlined_rules: HashMap::new(),
        };
        println!("Symbol Table created successfully");
        sym_table.create_symbol_table_from_tree(tree);
        sym_table.run_duplication_check();
        sym_table
    }

    pub fn get_names(&self) -> &Vec<String> {
        &self.names
    }

    pub fn print(&self) {
        for i in &self.names {
            println!("{}", i)
        }
    }

    pub fn push(&mut self, name: String) {
        self.names.push(name);
    }

    pub fn check_symbol(&self, symbol_name: &str) -> bool {
        for i in &self.names {
            if symbol_name == *i {
                return true;
            }
        }
        false
    }

    pub fn check_symbol_is_inline(&self, symbol_name: &str) -> bool {
        let rule = self.inlined_rules.get(symbol_name);
        rule.is_some()
    }

    fn run_duplication_check(&mut self) {
        self.names.sort();
        // Dedup only dedups consecutive elements hence the prior sort.
        self.names.dedup();
    }

    pub fn create_symbol_table_from_tree(&mut self, tree: &Tree) {
        let node = tree.get_node(Key(0));
        if node.rule != Rules::Grammar {
            panic!("Invalid Root. Must be of type Rules::Grammar");
        }
        let mut counter = 0;
        loop {
            if counter >= node.get_children().len() {
                break;
            }
            let child_index = node.get_children()[counter];
            // Recurse.
            self.create_symbol_table_from_tree_kernel(tree, child_index);
            counter += 1;
        }
    }

    pub fn print_inlined_rules(&self) {
        println!("{:#?}", self.inlined_rules);
    }

    fn check_semantic_instructions(&mut self, name: &str, tree: &Tree, index: Key) {
        let lhs = tree.get_node(index);
        let node = tree.get_node(lhs.parent.expect("Parent should exist"));
        for child in node.get_children() {
            let child_node = tree.get_node(*child);
            if child_node.rule == Rules::Semantic_Instructions {
                let sem_instr_key = child_node.get_children()[0];
                let sem_instr_node = tree.get_node(sem_instr_key).rule;
                if sem_instr_node == Rules::Inline {
                    self.inlined_rules.insert(name.to_string(), "".to_string());
                }
            }
        }
    }

    fn create_symbol_table_from_tree_kernel(&mut self, tree: &Tree, index: Key) {
        let node = tree.get_node(index);
        let mut counter = 0;
        if node.rule == Rules::Var_Name_Decl {
            if !node.result {
                panic!("No false results should exist.")
            }
            let name = &self.source
                [((node.start_position + 1) as usize)..((node.end_position - 1) as usize)];
            self.names.push(name.to_string());

            self.check_semantic_instructions(name, tree, index);
        }
        loop {
            if counter >= node.get_children().len() {
                break;
            }
            let child_index = node.get_children()[counter];
            // Recurse.
            self.create_symbol_table_from_tree_kernel(tree, child_index);
            counter += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::count_lines;
    use cache::MyCache4;
    use grammar_parser::grammar;
    use parser_core::Context;
    use parser_core::Source;
    use publisher::Tree;
    use rules::RULES_SIZE;
    use std::env;
    use std::fs::{canonicalize, read_to_string};

    #[test]
    fn test() {
        println!("{:?}", env::current_dir().unwrap());
        let path = "../grammar_parser/tests/newGrammar_test_only_dont_modify.dsl";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");
        let string2 = string.clone();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = Context::<MyCache4, Tree>::new(src_len, RULES_SIZE);
        let result = grammar(&context, &source, position);

        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = &context.stack.borrow();
        let src = &String::from(source);
        let clean_tree = tree.clear_false();
        clean_tree.print(Key(0), Some(true));
        let sym_table = SymbolTable::new(&clean_tree, src);
        sym_table.print();
        sym_table.print_inlined_rules()
    }

    #[test]
    fn test_2() {
        let string = "<Rule>='A'/'B';".to_string();
        let string2 = string.clone();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = Context::<MyCache4, Tree>::new(src_len, RULES_SIZE);
        let result = grammar(&context, &source, position);
        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = &context.stack.borrow();
        let src = &String::from(source);
        tree.print(Key(0), Some(true));
        println!("\nCLEAN TREE\n");
        let clean_tree = tree.clear_false();
        clean_tree.print(Key(0), Some(true));
        let sym_table = SymbolTable::new(&clean_tree, src);
        sym_table.print();
    }

    #[test]
    fn test_3() {
        let string = "<Rule> Inline ='A'/'B';".to_string();
        let string2 = string.clone();
        let src_len = string.len() as u32;
        let source = Source::new(&string);
        let position: u32 = 0;
        let context = Context::<MyCache4, Tree>::new(src_len, RULES_SIZE);
        let result = grammar(&context, &source, position);
        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = &context.stack.borrow();
        let src = &String::from(source);
        //tree.print(Key(0), Some(true));
        println!("\nCLEAN TREE\n");
        let clean_tree = tree.clear_false();
        clean_tree.print(Key(0), Some(true));
        let sym_table = SymbolTable::new(&clean_tree, src);
        sym_table.print();
        println!("Inlined Rules:");
        sym_table.print_inlined_rules();
    }
}
