use crate::count_lines;
use crate::symbol_table::SymbolTable;
use publisher::Node;
use publisher::Publisher;
use publisher::Tree;
use rules::Key;
use rules::Rules;
struct GeneratedCode {
    // String per rule so we can seperate into files per rule.
    output: Vec<String>,
}

impl GeneratedCode {
    pub fn new(symbol_table: &SymbolTable, tree: &Tree, source: &String) -> Self {
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
            let rule = self.match_rule(symbol_table, tree, source, child_index);
            println!("{}", rule);
            counter += 1;
        }
    }
    fn generate_kernel(
        &self,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) {
        let node = tree.get_node(index);
        let mut counter = 0;
        if node.rule == Rules::VarNameDecl && node.result {
            let name =
                &source[((node.start_position + 1) as usize)..((node.end_position - 1) as usize)];
            println!("VarNameDecl: {:?}", name);
        }
        loop {
            if counter >= node.get_children().len() {
                break;
            }
            let child_index = node.get_children()[counter];
            // Recurse.
            self.generate_kernel(symbol_table, tree, source, child_index);
            counter += 1;
        }
    }

    fn match_rule(
        &self,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) -> String {
        let node = tree.get_node(index);
        println!("node_rule: {:?}", node.rule);
        match node.rule {
            Rules::Rule => self.rule(symbol_table, tree, source, index),
            _ => {
                todo!("Not yet implemented")
            }
        }
    }

    fn rule(&self, symbol_table: &SymbolTable, tree: &Tree, source: &String, index: Key) -> String {
        let rule_node = tree.get_node(index);
        let rule_children = rule_node.get_children();
        let mut name: Option<String> = None;
        let mut rhs: Option<String> = None;
        let mut comment: Option<String> = None;
        tree.print(index, Some(true));
        for i in rule_children {
            match tree.get_node(*i).rule {
                Rules::Lhs => {
                    name = Some(self.lhs(symbol_table, tree, source, *i));
                }
                Rules::Rhs => {
                    rhs = Some(self.rhs(symbol_table, tree, source, *i));
                }
                Rules::Comment => {
                    comment = Some(self.comment(symbol_table, tree, source, *i));
                }
                _ => {}
            }
        }
        let rule_header = format!(
            "pub fn {}<T: Cache, S: Publisher>(
            _context: &Context<T, S>,
            source: &Source,
            position: u32,
        ) -> (bool, u32) {{",
            name.expect("Must have name")
        );
        let builder = format!(
            "{}\n{}\n{} }}",
            rule_header,
            comment.unwrap_or("".to_string()),
            rhs.expect("Must have rhs")
        );

        return builder;
    }

    fn lhs(&self, symbol_table: &SymbolTable, tree: &Tree, source: &String, index: Key) -> String {
        let lhs_node = tree.get_node(index);
        let lhs_children = lhs_node.get_children();
        let var_name_decl_key = lhs_children[0];
        let var_name_decl = tree.get_node(var_name_decl_key);
        let var_name_key = var_name_decl.get_children()[0];
        let var_name = tree.get_node(var_name_key);
        let name = var_name.get_string(source);
        // name still has angle brackets but cba to create string from subnodes
        // so simply remove first and last char
        let mut s = name;
        s.pop(); // remove last
        if s.len() > 0 {
            s.remove(0); // remove first
        }
        s
    }

    fn comment(
        &self,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) -> String {
        let comment_node = tree.get_node(index);
        let max_comment_size = (comment_node.end_position - comment_node.start_position) as usize;
        let acc = String::with_capacity(max_comment_size + 3);
        let mut start: Option<u32> = None;
        let mut end: Option<u32> = None;
        for key in comment_node.get_children() {
            match tree.get_node(*key).rule {
                Rules::Ascii => {
                    if start.is_none() {
                        start = Some(tree.get_node(*key).start_position);
                    }

                    end = Some(tree.get_node(*key).end_position);
                }
                _ => {}
            };
        }
        let acc = "// ".to_string()
            + &source[(start.expect("Must have start") as usize)
                ..(end.expect("Must have end") as usize)]
                .to_string();
        acc
    }

    fn rhs(&self, symbol_table: &SymbolTable, tree: &Tree, source: &String, index: Key) -> String {
        let rhs_node = tree.get_node(index);
        for i in rhs_node.get_children() {
            match tree.get_node(*i).rule {
                Rules::OrderedChoice => {
                    Some(self.ordered_choice(symbol_table, tree, source, *i));
                }
                Rules::Sequence => {},
                Rules::Atom => {},
                _ => {}
            }
        }

        "TODO: rhs".to_string()
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
    fn test_2() {
        let string = "<Rule>=\"A\"/\"B\";   #   Ein Kommentar   #  ".to_string();
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
        let tree = &tree.clear_false();

        //tree.print(Key(0), None);
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        //sym_table.print();
        let gen_code = GeneratedCode::new(&sym_table, &tree, src);
    }

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
        let tree = &tree.clear_false();
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        //sym_table.print();
        let gen_code = GeneratedCode::new(&sym_table, &tree, src);
    }
}
