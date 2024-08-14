#![allow(unused)]
use anyhow::Result;
use cache::*;
use grammar_parser::{grammar, test_lr_expr};
use parser_core::Context;
use parser_core::Source;
use publisher::*;
use rules::Key;
use rules::RULES_SIZE;
use std::fs::canonicalize;
use std::fs::read_to_string;
use std::path::Path;
use std::time::Instant;

pub fn parse(path: impl AsRef<Path>) -> Result<bool> {
    let pathbuf = canonicalize(path)?;
    let string = read_to_string(pathbuf)?;
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, RULES_SIZE);
    let now = Instant::now();
    let result = test_lr_expr(&context, &source, position);
    let elapsed = now.elapsed();
    context.stack.borrow().print(Key(0), None);
    let only_true_tree = context.stack.borrow().clear_false();
    let elapsed2 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Elapsed with Tree cleaning: {:.2?}", elapsed2);

    //only_true_tree.print(Key(0), None);
    //println!("Capacity: {:?}, Len: {:?}", context.stack.borrow().capacity(), context.stack.borrow().len());
    //println!("Capacity: {:?}, Len: {:?}", only_true_tree.capacity(), only_true_tree.len());
    // for i in &*context.publisher.borrow() {
    //     // if i[0] == 20 || i[0] == 36 || i[0] == 29 || (i[0] >= 26 && i[0] <= 32) {
    //     //     println!("{:?}: {}", i, &string2[(i[1] as usize)..(i[2] as usize)]);
    //     // }
    //     //println!("{}",i[0]);
    //     println!("{:?}: {}", i, &string2[(i[1] as usize)..(i[2] as usize)]);
    // }
    println!("{:?} {:?}", result.0, result.1);
    assert_eq!(result, (true, src_len));
    Ok(result.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;
    use std::path::Path;

    #[test]
    fn test_overflows_by_default() {
        let path = Path::new("../grammar_parser/newGrammar_test_only_dont_modify.dsl");
        parse(path);
    }
}
