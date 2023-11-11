use parser_core::Context;
use parser_core::Source;
use parser_core::_ordered_choice;
use parser_core::_terminal;
use cache::Cache;

pub fn spaces<T: Cache>(_context: &Context<T>, source: &Source, position: u32) -> (bool, u32) {
    let t1 = _terminal(b'\n');
    let t2 = _terminal(b'\t');
    let oc1 = _ordered_choice(&t1, &t2);

    let t1 = _terminal(b'\r');
    let t2 = _terminal(b' ');
    let oc2 = _ordered_choice(&t1, &t2);
    let hoc = _ordered_choice(&oc1, &oc2);

    hoc(source, position)
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser_core::Source;
    #[test]
    fn test_num_false() {
        let string = "aaa".to_string();
        let src_len = string.len() as u32;

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::new(src_len, 42);

        let result = spaces(&context, &source, position);
        assert_eq!(result, (false, 0));
    }
    #[test]
    fn test_num_true() {
        let string = "\n".to_string();
        let src_len = string.len() as u32;

        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::new(src_len, 42);

        let result = spaces(&context, &source, position);
        assert_eq!(result, (true, 1));
    }
}
