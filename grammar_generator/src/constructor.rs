use crate::count_lines;
use crate::symbol_table::SymbolTable;
use publisher::Tree;
use publisher::Publisher;
use rules::Key;
use rules::Rules;
struct GeneratedCode {
    // String per rule so we can seperate into files per rule.
    output: Vec<String>,
}

impl GeneratedCode {
    pub fn new(symbol_table: &SymbolTable, tree: &Tree, source: &String) -> Self {
        let mut sym_table = SymbolTable::new(tree, source);
        let s = GeneratedCode {
            output: Vec::<String>::new(),
        };
        println!("Generating Code");
        s.generate(symbol_table, tree, source);
        s
    }

    fn generate(&self, symbol_table: &SymbolTable, tree: &Tree, source: &String) {
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
            self.generate_kernel(symbol_table, tree,source, child_index);
            counter += 1;
        }
    }
    fn generate_kernel(&self, symbol_table: &SymbolTable, tree: &Tree, source: &String, index: Key) {
        let node = tree.get_node(index);
        let mut counter = 0;
        if node.rule == Rules::VarNameDecl && node.result {
            let name = &source
                [((node.start_position + 1) as usize)..((node.end_position - 1) as usize)];
            println!("{:?}", name);
        }
        loop {
            if counter >= node.get_children().len() {
                break;
            }
            let child_index = node.get_children()[counter];
            // Recurse.
            self.generate_kernel(symbol_table, tree,source, child_index);
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
        tree.print(Key(0), None);
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        //sym_table.print();
        let gen_code = GeneratedCode::new(&sym_table, &tree, src);
    }
}
