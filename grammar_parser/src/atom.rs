use cache::Cache;
use parser_core::Context;
use parser_core::_sequence;
use parser_core::{Source, _ordered_choice, _subexpression, _var_name};
use publisher::Publisher;
use rules::rules::Rules;

use crate::{
    and_predicate, not_predicate, nucleus, one_or_more, optional, sequence::sequence, whitespace,
    zero_or_more,
};

pub fn atom<T: Cache, S: Publisher>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let v1 = _var_name(Rules::AndPredicate, context, and_predicate);
    let v2 = _var_name(Rules::NotPredicate, context, not_predicate);
    let oc1 = _ordered_choice(&v1, &v2);

    let v3 = _var_name(Rules::OneOrMore, context, one_or_more);
    let oc2 = _ordered_choice(&oc1, &v3);

    let v4 = _var_name(Rules::ZeroOrMore, context, zero_or_more);
    let oc3 = _ordered_choice(&oc2, &v4);

    let v5 = _var_name(Rules::Optional, context, optional);
    let oc4 = _ordered_choice(&oc3, &v5);

    let v6 = _var_name(Rules::Nucleus, context, nucleus);
    let oc5 = _ordered_choice(&oc4, &v6);

    let sub1 = _subexpression(&oc5);

    let v7 = _var_name(Rules::Whitespace, context, whitespace);
    let oc6 = _sequence(&sub1, &v7);

    oc6(source, position)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cache::MyCache4;
    use parser_core::Source;
    use publisher::Tree;
    use rules::Key;
    #[test]
    fn test_atom_true() {
        let string = "\"A\"/\"B\"/\"C\"/\"D\"/\"E\"/\"F\"/\"G\"/\"H\"/\"I\"/\"J\"/\"K\"/\"L\"/\"M\"/\"N\"/\"O\"/\"P\"/\"Q\"/\"R\"/\"S\"/\"T\"/\"U\"/\"V\"/\"W\"/\"X\"/\"Y\"/\"Z\"".to_string();
        let src_len = string.len() as u32;

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, Tree>::new(src_len, 44);

        let result = atom(&context, &source, position);
        context.stack.borrow().print(Key(0), None);
        assert_eq!(result, (true, 3));
    }

    #[test]
    fn test_atom_char() {
        let string = "\"A\"/\"B\"".to_string();
        let src_len = string.len() as u32;

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, Tree>::new(src_len, 44);

        let result = atom(&context, &source, position);
        context.stack.borrow().print(Key(0), Some(true));
        assert_eq!(result, (true, 3));
    }
}
