use publisher::{Publisher, Tree};
use rules::{rules::Rules, Key};

use std::panic;
use crate::count_lines;
pub struct SymbolTable<'a> {
    names: Vec<String>,
    source: &'a String,
}

impl<'a> SymbolTable<'a> {
    pub fn new(tree: &Tree, source: &'a String) -> Self {
        let mut sym_table = SymbolTable {
            names: Vec::<String>::new(),
            source,
        };
        println!("Symbol Table created successfully");
        sym_table.create_symbol_table_from_tree(tree);
        sym_table.run_duplication_check();
        return sym_table;
    }

    pub fn print(&self) {
        for i in &self.names {
            println!("{}", i)
        }
    }

    pub fn push(&mut self, index: u32, name: String) {
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

    fn run_duplication_check(&mut self){
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

    fn create_symbol_table_from_tree_kernel(&mut self, tree: &Tree, index: Key) {
        let node = tree.get_node(index);
        let mut counter = 0;
        if node.rule == Rules::VarNameDecl {
            if !node.result{
                panic!("No false results should exist.")
            }
            let name = &self.source[((node.start_position+1) as usize)..((node.end_position-1) as usize)];
            self.names.push(name.to_string());
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
    use cache::MyCache4;
    use grammar_parser::grammar;
    use parser_core::Context;
    use parser_core::Source;
    use publisher::Tree;
    use std::env;
    use std::fs::{canonicalize, read_to_string};

    #[test]
    fn test() {
        println!("{:?}", env::current_dir().unwrap());
        let path = "../parser_core/tests/Grammar.txt";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");
        let string2 = string.clone();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, Tree>::new(src_len, 45);
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
        let src =&String::from(source);
        let clean_tree = tree.clear_false();
        clean_tree.print(Key(0), Some(true));
        let sym_table = SymbolTable::new(&clean_tree, src);
        sym_table.print();
    }

    #[test]
    fn test_2() {
        let string = "<Rule>=\"A\"/\"B\";".to_string();
        let string2 = string.clone();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, Tree>::new(src_len, 45);
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
        let src =&String::from(source);
        tree.print(Key(0), Some(true));
        println!("\nCLEAN TREE\n");
        let clean_tree = tree.clear_false();
        clean_tree.print(Key(0), Some(true));
        let sym_table = SymbolTable::new(&clean_tree, src);
        sym_table.print();
    }
}


