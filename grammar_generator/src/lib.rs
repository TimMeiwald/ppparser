use cache::MyCache4;
use grammar_parser::grammar;
use grammar_parser::Context;
use grammar_parser::Source;
use parser_core::Rules;
use stack::BasicStack;
use std::env;
use std::fs::canonicalize;
use std::fs::read_to_string;

struct SymbolTable {
    index: Vec<u32>,
    name: Vec<String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            index: Vec::<u32>::new(),
            name: Vec::<String>::new(),
        }
    }

    pub fn push(&mut self, index: u32, name: String) {
        self.index.push(index);
        self.name.push(name);
    }

    pub fn check_symbol(&self, symbol_name: &str) -> bool {
        for i in &self.name{
            if symbol_name == *i {
                return true
            }
        }
        false
    }
}

fn count_lines(src: &String, start_position: u32) -> u32 {
    let mut new_line_count: u32 = 0;

    for i in &src.as_bytes()[0..start_position as usize] {
        if *i == b'\n'{
            new_line_count += 1;
        }
    }
    return new_line_count
}

fn create_symbol_table(stack: &BasicStack, src: &String) -> SymbolTable {
    let mut sym_table = SymbolTable::new();
    let mut index = 0;
    for i in stack {
        if i[0] == Rules::VarNameDecl as u32 {
            let name = &src[(i[1] as usize)..(i[2] as usize)];
            sym_table.push(index, name.to_string());
        }
        index += 1;
    }
    for i in stack{
        if i[0] == Rules::VarName as u32{
            let name = &src[(i[1] as usize)..(i[2] as usize)];
            if !sym_table.check_symbol(name){
                panic!("{:?} on Line: {:?} - Chars: {:?}:{:?} does not exist", name, count_lines(src, i[1]), i[1] as usize, i[2] as usize);
            }

        }
    }

    for i in 0..sym_table.index.len() {
        println!("{}", sym_table.name[i])
    }
    sym_table
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
    let context = Context::<MyCache4, BasicStack>::new(src_len, 45);
    let result = grammar(&context, &source, position);
    //context.stack.borrow().print(&string2);
    // for i in &*context.stack.borrow() {

    //     println!("{:?}: {}", i, &string2[(i[1] as usize)..(i[2] as usize)]);
    // }
    // println!("\n\n");
    let sym_table = create_symbol_table(&*context.stack.borrow(), &string2);
    assert_eq!(result, (true, src_len));
}
