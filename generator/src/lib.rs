use parser::parser::{Stack, AlphabetUpper};
use std::{path::Path};
use parser::parser::Rules;
use std::collections::HashSet;

pub fn generate(source: String, ast: Stack){
    symbol_table(source, ast);
}

fn print_ast(source: String, ast: Stack){
    for entry in &ast{
        match entry.rule {
            Rules::Rule => {println!("\n\n{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            Rules::Lhs => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            Rules::OrderedChoice => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            Rules::Sequence => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            Rules::OneOrMore => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            Rules::ZeroOrMore => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            Rules::Optional => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            Rules::AndPredicate => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            Rules::NotPredicate => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            Rules::Subexpression => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            Rules::VarName => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            Rules::Lterminal => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            Rules::Atom => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            _ => {}
        } 
    }
}

fn symbol_table(source: String, ast: Stack){
    let response: String;
    let mut set_of_var_names: HashSet<String> = std::collections::HashSet::new();
    // Get the rule names 
    for entry in &ast{
        match entry.rule {
            //Rules::Rule => {println!("\n\n{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            // Rules::Lhs => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            // Rules::OrderedChoice => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            // Rules::Sequence => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            // Rules::OneOrMore => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            // Rules::ZeroOrMore => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            // Rules::Optional => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            // Rules::AndPredicate => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            // Rules::NotPredicate => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            // Rules::Subexpression => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            Rules::VarName => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}

            // Rules::VarName => {
            //     let s = &source[entry.start_position as usize..entry.end_position as usize];
            //     let s = &s[1..s.len()-1];
            //     set_of_var_names.insert(s.to_string());
            // }
            //Rules::Lterminal => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            Rules::Atom => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
            _ => {}
        } 
    }
    for name in set_of_var_names{
        println!("{}", name);
    }

}




#[cfg(test)]
mod tests {
    use parser::parse;

    use super::*;
    #[test]
    fn test_generate() {
        let (_,_,_,source,stack) = parse(Path::new("../parser/src/Grammar.txt"));
        print_ast(source, stack);
    }

}
