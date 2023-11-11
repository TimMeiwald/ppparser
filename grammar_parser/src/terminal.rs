use super::*;
use cache::Cache;
use parser_core::{Context, Rules};
use parser_core::{Source, _ordered_choice, _sequence, _subexpression, _terminal, _var_name};

pub fn terminal<T: Cache>(context: &Context<T>, source: &Source, position: u32) -> (bool, u32) {
    let apostrophe = _var_name(Rules::Apostrophe, context, apostrophe);
    let ascii = _var_name(Rules::Ascii, context, ascii);
    let s1 = _sequence(&apostrophe, &ascii);
    let s2 = _sequence(&s1, &apostrophe);
    let sub1 = _subexpression(&s2);

    let t1 = _terminal(b'\\');
    let t2 = _terminal(b'n');
    let t3 = _terminal(b'r');
    let t4 = _terminal(b't');
    let oc3 = _ordered_choice(&t2, &t3);
    let oc4 = _ordered_choice(&oc3, &t4);
    let sub2 = _subexpression(&oc4);

    let s3 = _sequence(&apostrophe, &t1);
    let s4 = _sequence(&s3, &sub2);
    let s5 = _sequence(&s4, &apostrophe);
    let sub3 = _subexpression(&s5);

    let s6 = _ordered_choice(&sub1, &sub3);
    let epsilon = _var_name(Rules::Epsilon, context, epsilon);
    let s7 = _ordered_choice(&s6, &epsilon);

    s7(source, position)
}
#[cfg(test)]
mod tests {
    use super::*;
    use cache::MyCache4;
    use parser_core::Source;

    #[test]
    fn test_terminal_false() {
        let string = "\"a".to_string();
        let src_len = string.len() as u32;

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4>::new(src_len, 42);

        let result = terminal(&context, &source, position);
        assert_eq!(result, (false, 0));
    }
    #[test]
    fn test_terminal_true() {
        let string = "\"a\"".to_string();
        let src_len = string.len() as u32;

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4>::new(src_len, 42);

        let result = terminal(&context, &source, position);
        assert_eq!(result, (true, 3));
    }
    #[test]
    fn test_terminal_true1() {
        let string = "\"\n\"".to_string();
        let src_len = string.len() as u32;

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4>::new(src_len, 42);

        let result = terminal(&context, &source, position);
        assert_eq!(result, (true, 3));
    }
    #[test]
    fn test_terminal_true2() {
        let string = "\"\t\"".to_string();
        let src_len = string.len() as u32;

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4>::new(src_len, 42);

        let result = terminal(&context, &source, position);
        assert_eq!(result, (true, 3));
    }
    #[test]
    fn test_terminal_true3() {
        let string = "\"\r\"".to_string();
        let src_len = string.len() as u32;

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4>::new(src_len, 42);

        let result = terminal(&context, &source, position);
        assert_eq!(result, (true, 3));
    }

    #[test]
    fn test_terminal_true4() {
        let string = "\" \"".to_string();
        let src_len = string.len() as u32;

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4>::new(src_len, 42);

        let result = terminal(&context, &source, position);
        assert_eq!(result, (true, 3));
    }
}
