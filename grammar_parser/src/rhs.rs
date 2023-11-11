use cache::Cache;
use parser_core::{Context, Rules};
use parser_core::{Source, _ordered_choice, _var_name};

use crate::{atom, ordered_choice, sequence};

pub fn rhs<T: Cache>(context: &Context<T>, source: &Source, position: u32) -> (bool, u32) {
    let v1 = _var_name(Rules::Sequence, context, sequence);
    let v2 = _var_name(Rules::OrderedChoice, context, ordered_choice);
    let v3 = _var_name(Rules::Atom, context, atom);
    let s1 = _ordered_choice(&v1, &v2);
    let s2 = _ordered_choice(&s1, &v3);
    s2(source, position)
}
#[cfg(test)]
mod tests {
    use super::*;
    use cache::MyCache4;
    use parser_core::Source;

    #[test]
    fn test_rhs_true() {
        let string = "\"A\"/\"B\"/\"C\"/\"D\"/\"E\"/\"F\"/\"G\"/\"H\"/\"I\"/\"J\"/\"K\"/\"L\"/\"M\"/\"N\"/\"O\"/\"P\"/\"Q\"/\"R\"/\"S\"/\"T\"/\"U\"/\"V\"/\"W\"/\"X\"/\"Y\"/\"Z\"".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4>::new(src_len, 42);

        let result = rhs(&context, &source, position);
        assert_eq!(result, (true, src_len));
    }
}
