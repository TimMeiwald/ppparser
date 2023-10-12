use parser_core::{Context, Rules};
use parser_core::{Source, _sequence, _var_name, _zero_or_more};

use crate::{
    comment, lhs, rhs,
    symbols::{assignment, end_rule},
    whitespace,
};

pub fn rule(context: &Context, source: &Source, position: u32) -> (bool, u32) {
    let v1 = _var_name(Rules::Lhs, context, lhs);
    let v2 = _var_name(Rules::Whitespace, context, whitespace);
    let v3 = _var_name(Rules::Assignment, context, assignment);
    let v4 = _var_name(Rules::Rhs, context, rhs);
    let v5 = _var_name(Rules::EndRule, context, end_rule);
    let v6 = _var_name(Rules::Comment, context, comment);
    let z1 = _zero_or_more(&v6);

    let s1 = _sequence(&v1, &v2);
    let s2 = _sequence(&s1, &v3);
    let s3 = _sequence(&s2, &v2);
    let s4 = _sequence(&s3, &v4);
    let s5 = _sequence(&s4, &v2);
    let s6 = _sequence(&s5, &v5);
    let s7 = _sequence(&s6, &v2);
    let s8 = _sequence(&s7, &z1);

    s8(source, position)
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser_core::Source;

    #[test]
    fn test_rule_true() {
        let string = "<Alphabet_Upper> PASSTHROUGH = \"A\"/\"B\"/\"C\"/\"D\"/\"E\"/\"F\"/\"G\"/\"H\"/\"I\"/\"J\"/\"K\"/\"L\"/\"M\"/\"N\"/\"O\"/\"P\"/\"Q\"/\"R\"/\"S\"/\"T\"/\"U\"/\"V\"/\"W\"/\"X\"/\"Y\"/\"Z\"; #We all love commments#".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::new(src_len, 42);

        let result = rule(&context, &source, position);
        assert_eq!(result, (true, src_len));
    }

    #[test]
    fn test_rule_true2() {
        let string = "<Spaces> PASSTHROUGH = \"\n\"/\"\t\"/\"\r\"/\" \";".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::new(src_len, 42);

        let result = rule(&context, &source, position);
        assert_eq!(result, (true, src_len));
    }

    #[test]
    fn test_rule_true3() {
        let string = "<Specials> PASSTHROUGH = \"+\"/\"*\"/\"-\"/\"&\"/\"!\"/\"?\"/\"<\"/\">\"/\"\"\"/\"(\"/\")\"/\"_\"/\",\"/\"/\"/\";\"/\"=\"/\"\\\"/\"#\"/\":\"/\"|\"/\".\"/\"{\"/\"}\"/\"[\"/\"]\"/\"%\"/\"'\"/\"^\"/\"~\";".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::new(src_len, 42);

        let result = rule(&context, &source, position);
        assert_eq!(result, (true, src_len));
    }

    #[test]
    fn test_rule_true4() {
        let string =
            "<ASCII> PASSTHROUGH = <Alphabet_Lower>/<Alphabet_Upper>/<Num>/<Spaces>/<Specials>;"
                .to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::new(src_len, 42);

        let result = rule(&context, &source, position);
        assert_eq!(result, (true, src_len));
    }

    #[test]
    fn test_rule_true5() {
        let string = "<RHS> PASSTHROUGH = <Sequence>/<Ordered_Choice>/<Atom>;
    "
        .to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::new(src_len, 42);

        let result = rule(&context, &source, position);
        assert_eq!(result, (true, src_len));
    }
    #[test]
    fn test_rule_true6() {
        let string = "<LHS> = <Var_Name>, (<Whitespace>, <Semantic_Instructions>, <Whitespace>)?;
    "
        .to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::new(src_len, 42);

        let result = rule(&context, &source, position);
        assert_eq!(result, (true, src_len));
    }
    #[test]
    fn test_rule_true7() {
        let string = "<Rule> = <LHS>, <Whitespace>, <Assignment>, <Whitespace>, <RHS>, <Whitespace>, <End_Rule>, <Whitespace>, <Comment>*;".to_string();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::new(src_len, 42);

        let result = rule(&context, &source, position);
        assert_eq!(result, (true, src_len));
    }
}
