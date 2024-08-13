mod parse;

#[cfg(test)]
mod tests {
    use crate::parse::parse;
    use std::env;
    #[test]
    fn test_basic_publisher() {
        println!("{:?}", env::current_dir().unwrap());
        // This file won't be valid for a different type of parser obviously
        let path = "../grammar_parser/tests/newGrammar_test_only_dont_modify.dsl"; // Switch to some other file location for tests if you wish
        let success = parse(path);
        assert!(success.unwrap())
    }
}
