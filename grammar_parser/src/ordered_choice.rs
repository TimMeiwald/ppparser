use cache::Cache;
use parser_core::{Context, Rules};
use parser_core::{Source, _sequence, _subexpression, _var_name, _zero_or_more};
use stack::Stack;

use crate::{atom, symbols::backslash, whitespace};

pub fn ordered_choice<T: Cache, S: Stack>(
    context: &Context<T, S>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let v1 = _var_name(Rules::Atom, context, atom);
    let v2 = _var_name(Rules::Whitespace, context, whitespace);
    let v3 = _var_name(Rules::Backslash, context, backslash);
    let s1 = _sequence(&v1, &v2);
    let s2 = _sequence(&s1, &v3);
    let v4 = _var_name(Rules::Whitespace, context, whitespace);
    let v5 = _var_name(Rules::Atom, context, atom);

    let s3 = _sequence(&s2, &v4);
    let s4 = _sequence(&s3, &v5);

    let v7 = _var_name(Rules::Atom, context, atom);
    let v6 = _var_name(Rules::Whitespace, context, whitespace);
    let v8 = _var_name(Rules::Backslash, context, backslash);

    let s5 = _sequence(&v8, &v6);
    let s6 = _sequence(&s5, &v7);
    let sub1 = _subexpression(&s6);
    let z1 = _zero_or_more(&sub1);

    let s7 = _sequence(&s4, &z1);
    s7(source, position)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cache::MyCache4;
    use parser_core::Source;
    use stack::NoopStack;

    #[test]
    fn test_ordered_choice_false() {
        let string = "<this_is_a_valid_var_name>".to_string();
        let str_len = string.len() as u32;

        let source = Source::new(string);

        let position: u32 = 0;
        let context = Context::<MyCache4, NoopStack>::new(str_len, 44);

        let result = ordered_choice(&context, &source, position);
        assert_eq!(result, (false, 0));
    }
    #[test]
    fn test_ordered_choice_true() {
        let string = "\"A\"/\"B\"".to_string();
        let str_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, NoopStack>::new(str_len, 44);

        let result = ordered_choice(&context, &source, position);
        assert_eq!(result, (true, 7));
    }

    #[test]
    fn test_ordered_choice_true2() {
        let string = "\"A\"/\"B\"/\"C\"".to_string();
        let str_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, NoopStack>::new(str_len, 44);
        let result = ordered_choice(&context, &source, position);
        assert_eq!(result, (true, 11));
    }

    #[test]
    fn test_ordered_choice_true3() {
        let string = "\"A\"/\"B\"/\"C\"/\"D\"/\"E\"/\"F\"/\"G\"/\"H\"/\"I\"/\"J\"/\"K\"/\"L\"/\"M\"/\"N\"/\"O\"/\"P\"/\"Q\"/\"R\"/\"S\"/\"T\"/\"U\"/\"V\"/\"W\"/\"X\"/\"Y\"/\"Z\"".to_string();
        let str_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, NoopStack>::new(str_len, 44);

        let result = ordered_choice(&context, &source, position);
        assert_eq!(result, (true, str_len));
    }
    #[test]
    fn test_ordered_choice_true4() {
        let string = "\"+\"/\"*\"/\"-\"/\"&\"/\"!\"/\"?\"/\"<\"/\">\"/\"\"\"/\"(\"/\")\"/\"_\"/\",\"/\"/\"/\";\"/\"=\"/\"\\\"/\"#\"/\":\"/\"|\"/\".\"/\"{\"/\"}\"/\"[\"/\"]\"/\"%\"/\"'\"/\"^\"/\"~\"".to_string();
        let str_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, NoopStack>::new(str_len, 44);

        let result = ordered_choice(&context, &source, position);
        assert_eq!(result, (true, str_len));
    }

    #[test]
    fn test_ordered_choice_true5() {
        let string = "\"\n\"/\"\t\"/\"\r\"/\" \"".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, NoopStack>::new(src_len, 44);

        let result = ordered_choice(&context, &source, position);
        assert_eq!(result, (true, src_len));
    }
}
