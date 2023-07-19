use parser::parser::{Stack, AlphabetUpper};
use std::{path::Path};
use parser::parser::Rules;

pub fn generate(source: String, ast: Stack){
    match_rule(source, ast);
}

fn match_rule(source: String, ast: Stack){
    let mut count = 0;
    for entry in &ast{
        //println!("{:?}, {}, {}", entry.rule, entry.start_position, entry.end_position);
        match entry.rule {
            Rules::Grammar => {println!("{:?}, {}, {}", entry.rule, entry.start_position, entry.end_position)}
            Rules::Rule => {println!("{:?}, {}, {}, {}", entry.rule, entry.start_position, entry.end_position, &source[entry.start_position as usize..entry.end_position as usize])}
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
            _ => {}
        }
        //println!("{}", count);

        count += 1;   
    }

}


#[cfg(test)]
mod tests {
    use parser::parse;

    use super::*;
    #[test]
    fn test_generate() {
        let (_,_,_,source,stack) = parse(Path::new("../parser/src/Grammar.txt"));
        generate(source, stack);
    }

}
