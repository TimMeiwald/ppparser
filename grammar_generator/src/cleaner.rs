use crate::count_lines;
use crate::symbol_table::SymbolTable;
use publisher::Node;
use publisher::Publisher;
use publisher::Tree;
use rules::Key;
use rules::Rules;
use std::cell::RefCell;
use std::rc::Rc;



fn clean_children(index: Key, tree: &mut Tree) {
    let node: &Node = tree.get_node(index);
    let mut valid_children = Vec::<Key>::new();
    for child_index in node.get_children() {
        let result = tree.get_node(*child_index).result;
        if result {
            valid_children.push(*child_index);
        }
    }
    let node = tree.get_mut_node(index);
    node.set_children(valid_children);
}

fn cleaner(tree: &mut Tree) {
    clean_children(Key(0), tree);
    let node = tree.get_node(Key(0));

    let mut counter = 0;
    loop {
        if counter >= node.get_children().len() {
            break;
        }
        let child_index = node.get_children()[counter];
        // Recurse.
        cleaner_kernel(tree, child_index);
        counter += 1;
    }
}
fn cleaner_kernel(tree: &Tree, index: Key) {
    clean_children(Key(0), tree);

    let node = tree.get_node_mut(index);
    let mut counter = 0;
    loop {
        if counter >= node.get_children().len() {
            break;
        }
        let child_index = node.get_children()[counter];
        // Recurse.
        cleaner_kernel(tree, child_index);
        counter += 1;
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
