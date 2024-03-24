use cache::Cache;
use parser_core::{Context, Rules};
use parser_core::{Source, _ordered_choice, _sequence, _subexpression, _var_name};
use stack::Stack;

use crate::{subexpression, terminal, var_name, whitespace};

pub fn nucleus<T: Cache, S: Stack>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let v1 = _var_name(Rules::Subexpression, context, subexpression);
    let v2 = _var_name(Rules::Terminal, context, terminal);
    let v3 = _var_name(Rules::VarName, context, var_name);
    let v4 = _var_name(Rules::Whitespace, context, whitespace);

    let oc1 = _ordered_choice(&v1, &v2);
    let oc2 = _ordered_choice(&oc1, &v3);
    let sub1 = _subexpression(&oc2);

    let s1 = _sequence(&sub1, &v4);
    s1(source, position)
}



#[cfg(test)]
mod tests {
    use super::*;
    use cache::MyCache4;
    use parser_core::Source;
    use stack::{BasicStack, NoopStack};

    #[test]
    fn test_nucleus_true() {
        let string = "\"A\"/\"B\"/\"C\"/\"D\"/\"E\"/\"F\"/\"G\"/\"H\"/\"I\"/\"J\"/\"K\"/\"L\"/\"M\"/\"N\"/\"O\"/\"P\"/\"Q\"/\"R\"/\"S\"/\"T\"/\"U\"/\"V\"/\"W\"/\"X\"/\"Y\"/\"Z\"".to_string();
        let src_len = string.len() as u32;

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, BasicStack>::new(src_len, 44);

        let result = nucleus(&context, &source, position);
        context.stack.borrow().print(&String::from(source));
        assert_eq!(result, (true, 3));
    }
    #[test]
    fn test_nucleus_char() {
        let string = "\"A\"".to_string();
        let src_len = string.len() as u32;

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, BasicStack>::new(src_len, 44);

        let result = nucleus(&context, &source, position);
        context.stack.borrow().print(&String::from(source));
        assert_eq!(result, (true, 3));
    }
}
