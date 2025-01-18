use super::binary_wo::{BinaryTreeWO, Reference};
use crate::symbol_table::SymbolTable;
use indoc::indoc;
use ::parser::*;
use std::panic::panic_any;

pub struct GeneratedCode<'a> {
    // String per rule so we can seperate into files per rule.
    pub rules_enum_header: &'a str,
    pub rules_size_header: &'a str,
    pub parser_header: &'a str,
    pub num_rules: usize,
    pub rules: Vec<String>,
    pub rules_enum: String,
    pub rules_enum_header_2: &'a str,
}

impl GeneratedCode<'_> {
    pub fn new(symbol_table: &SymbolTable, tree: &BasicPublisher, source: &str) -> Self {
        println!("Generating Code");
        let rules = Self::generate(symbol_table, tree, source);
        let (rules_enum, num_rules) = Self::generate_rules_enum(symbol_table);
        let rules_enum_header = indoc! {
            r##"#![allow(non_camel_case_types)] // Again due to generation -> Might solve eventually
            use num_derive::FromPrimitive;
            impl From<u32> for Rules {
                fn from(i: u32) -> Rules {
                    let element = num::FromPrimitive::from_u32(i);
                    match element {
                        Some(rule) => rule,
                        None => panic!("Not a valid Rule"),
                    }
                }
            }"##
        };
        let rules_enum_header_2 = indoc! {r##"
            #[allow(clippy::upper_case_acronyms)] // Again due to generation -> Might solve eventually
            #[derive(PartialEq, Eq, Hash, FromPrimitive, Clone, Copy, Debug, Ord, PartialOrd)]
        "##};
        let rules_size_header = indoc! {
            r##"#[allow(dead_code)]
                pub static RULES_SIZE: u32 = "##
        };
        let parser_header = indoc! {
            r##"#![allow(non_camel_case_types)] // Generated Code kinda annoying to deal with so w/e
            #![allow(unused_variables)] // Generated Code also, since everything passes stuff
            #![allow(unused_imports)] // Generated Code also, since everything passes stuff
            use crate::*;
            use std::cell::RefCell;"##
        };

        let s = GeneratedCode {
            rules_enum_header,
            rules_size_header,
            parser_header,
            rules,
            rules_enum,
            num_rules,
            rules_enum_header_2,
        };

        println!("Code generation complete");
        s
    }

    pub fn sample_main_header(&self, name: &str) -> String {
        let name = name.replace("-", "_");
        format!("use {}::*;\n", name)
    }

    pub fn parser_file_content(&self) -> String {
        let mut rules: String = "".to_string();
        for i in &self.rules {
            rules.push_str(i);
        }
        format!("{}\n{}", self.parser_header, rules)
    }

    pub fn rules_enum_file_content(&self) -> String {
        let rules_size = format!("{} {};", self.rules_size_header, self.num_rules);
        format!(
            "{}\n{}\n{}\n{}",
            self.rules_enum_header, rules_size, self.rules_enum_header_2, self.rules_enum
        )
    }

    pub fn print(&self) {
        println!("Number of Rules: {}", self.num_rules);
        println!("Rules Enum: \n");
        println!("{}\n", self.rules_enum);
        println!("Rules:\n");
        print!("{}", self.parser_header);
        for i in &self.rules {
            println!("{}", i)
        }
    }

    fn generate(symbol_table: &SymbolTable, tree: &BasicPublisher, source: &str) -> Vec<String> {
        let node = tree.get_node(Key(0));
        if node.rule != Rules::Grammar {
            panic!("Invalid Root. Must be of type Rules::Grammar");
        }
        let mut counter = 0;
        let mut rules = Vec::new();
        loop {
            if counter >= node.get_children().len() {
                break;
            }
            let child_index = node.get_children()[counter];
            // Recurse.
            let rule = Self::match_rule(symbol_table, tree, source, child_index);
            rules.push(rule);
            counter += 1;
        }
        rules
    }

    fn generate_rules_enum(symbol_table: &SymbolTable) -> (String, usize) {
        let mut rules_enum = "pub enum Rules {\n".to_string();
        let mut count: usize = 0;
        for s in symbol_table.get_names() {
            if !symbol_table.check_symbol_is_inline(s) {
                rules_enum.push('\t');
                rules_enum.push_str(s);
                rules_enum.push(',');
                rules_enum.push('\n');
                count += 1;
            }
        }
        rules_enum.push_str("\n}");
        (rules_enum, count)
    }

    fn match_rule(
        symbol_table: &SymbolTable,
        tree: &BasicPublisher,
        source: &str,
        index: Key,
    ) -> String {
        let node = tree.get_node(index);
        match node.rule {
            Rules::Rule => Self::rule(symbol_table, tree, source, index),
            _ => {
                todo!("Not yet implemented")
            }
        }
    }

    fn rule(symbol_table: &SymbolTable, tree: &BasicPublisher, source: &str, index: Key) -> String {
        let rule_node = tree.get_node(index);
        let rule_children = rule_node.get_children();
        let mut name: Option<String> = None;
        let mut rhs: String = "".to_string();
        let mut comment: Option<String> = None;
        for i in rule_children {
            match tree.get_node(*i).rule {
                Rules::LHS => {
                    name = Some(Self::lhs(symbol_table, tree, source, *i));
                    name = Some(name.unwrap().to_lowercase());
                }
                Rules::RHS => {
                    let mut out_tree = BinaryTreeWO::new();
                    let rhs_key = Self::rhs(&mut out_tree, symbol_table, tree, source, *i);
                    let last_key = out_tree.push(Reference::Exec, Some(rhs_key), None);
                    //out_tree.print(last_key);

                    let mut result = "".to_string();
                    for i in out_tree.to_string(last_key) {
                        result.push('\t');
                        result.push_str(&i);
                        result.push('\n');
                    }

                    rhs = result
                }
                Rules::Comment => {
                    comment = Some(Self::comment(symbol_table, tree, source, *i));
                }
                _ => {}
            }
        }

        let rule_header = format!(

            "#[allow(dead_code)]\npub fn {}<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {{",
            name.expect("Must have name")
        );
        let builder = format!(
            "{}\n{}\n{}\n}} ",
            rule_header,
            comment.unwrap_or("".to_string()),
            rhs
        );
        //println!("{}", builder);
        builder
    }

    fn lhs(_symbol_table: &SymbolTable, tree: &BasicPublisher, source: &str, index: Key) -> String {
        let lhs_node = tree.get_node(index);
        let lhs_children = lhs_node.get_children();
        let var_name_decl_key = lhs_children[0];
        let var_name_decl = tree.get_node(var_name_decl_key);
        // let var_name_key = var_name_decl.get_children()[0];
        // let var_name = tree.get_node(var_name_key);
        let name = var_name_decl.get_string(source);
        // name still has angle brackets but cba to create string from subnodes
        // so simply remove first and last char
        let mut s = name;
        s.pop(); // remove last
        if !s.is_empty() {
            s.remove(0); // remove first
        }
        s
    }

    fn comment(
        _symbol_table: &SymbolTable,
        tree: &BasicPublisher,
        source: &str,
        index: Key,
    ) -> String {
        let comment_node = tree.get_node(index);
        let mut start: Option<u32> = None;
        let mut end: Option<u32> = None;
        for key in comment_node.get_children() {
            if let Rules::ASCII = tree.get_node(*key).rule {
                if start.is_none() {
                    start = Some(tree.get_node(*key).start_position);
                }

                end = Some(tree.get_node(*key).end_position);
            };
        }

        "\t// ".to_string()
            + &source
                [(start.expect("Must have start") as usize)..(end.expect("Must have end") as usize)]
    }

    fn rhs(
        out_tree: &mut BinaryTreeWO,
        symbol_table: &SymbolTable,
        tree: &BasicPublisher,
        source: &str,
        index: Key,
    ) -> Key {
        let rhs_node = tree.get_node(index);
        let mut ret_key = Key(0);
        for i in rhs_node.get_children() {
            match tree.get_node(*i).rule {
                Rules::Ordered_Choice => {
                    ret_key = Self::ordered_choice(out_tree, symbol_table, tree, source, *i);
                }
                Rules::Sequence => {
                    ret_key = Self::sequence(out_tree, symbol_table, tree, source, *i);
                }
                Rules::Atom => ret_key = Self::atom(out_tree, symbol_table, tree, source, *i),
                _ => panic!("rhs"),
            }
        }
        ret_key
    }

    fn ordered_choice(
        out_tree: &mut BinaryTreeWO,
        symbol_table: &SymbolTable,
        tree: &BasicPublisher,
        source: &str,
        index: Key,
    ) -> Key {
        let oc_node = tree.get_node(index);
        let mut count = 0;

        let mut last_key = Key(0);
        for i in oc_node.get_children() {
            match tree.get_node(*i).rule {
                Rules::Atom => {
                    let mut key = Self::atom(out_tree, symbol_table, tree, source, *i);
                    if count != 0 {
                        key = out_tree.push(Reference::OrderedChoice, Some(last_key), Some(key))
                    }
                    count += 1;
                    last_key = key;
                }
                //Rules::Backslash => {}
                _ => panic!("ordered_choice"),
            }
        }
        last_key
    }

    fn sequence(
        out_tree: &mut BinaryTreeWO,
        symbol_table: &SymbolTable,
        tree: &BasicPublisher,
        source: &str,
        index: Key,
    ) -> Key {
        let oc_node = tree.get_node(index);
        let mut count = 0;

        let mut last_key = Key(0);
        for i in oc_node.get_children() {
            match tree.get_node(*i).rule {
                Rules::Atom => {
                    let mut key = Self::atom(out_tree, symbol_table, tree, source, *i);
                    if count != 0 {
                        key = out_tree.push(Reference::Sequence, Some(last_key), Some(key));
                    }
                    count += 1;
                    last_key = key;
                }
                //Rules::Comma => {}
                _ => panic!("sequence"),
            }
        }
        last_key
    }

    fn atom(
        out_tree: &mut BinaryTreeWO,
        symbol_table: &SymbolTable,
        tree: &BasicPublisher,
        source: &str,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut ret_key = Key(0);

        for i in node.get_children() {
            match tree.get_node(*i).rule {
                Rules::And_Predicate => {
                    ret_key = Self::and_predicate(out_tree, symbol_table, tree, source, *i);
                }
                Rules::Not_Predicate => {
                    ret_key = Self::not_predicate(out_tree, symbol_table, tree, source, *i);
                }
                Rules::One_Or_More => {
                    ret_key = Self::one_or_more(out_tree, symbol_table, tree, source, *i);
                }
                Rules::Zero_Or_More => {
                    ret_key = Self::zero_or_more(out_tree, symbol_table, tree, source, *i);
                }
                Rules::Optional => {
                    ret_key = Self::optional(out_tree, symbol_table, tree, source, *i);
                }
                Rules::Nucleus => {
                    ret_key = Self::nucleus(out_tree, symbol_table, tree, source, *i);
                }

                _ => panic!("atom"),
            }
        }
        ret_key
    }

    fn optional(
        out_tree: &mut BinaryTreeWO,
        symbol_table: &SymbolTable,
        tree: &BasicPublisher,
        source: &str,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut ret_key = Key(0);

        for i in node.get_children() {
            match tree.get_node(*i).rule {
                //Rules::Question_Mark => {}
                Rules::Nucleus => {
                    let key = Self::nucleus(out_tree, symbol_table, tree, source, *i);
                    ret_key = out_tree.push(Reference::Optional, Some(key), None)
                }

                _ => panic!("optional"),
            }
        }
        ret_key
    }
    fn one_or_more(
        out_tree: &mut BinaryTreeWO,
        symbol_table: &SymbolTable,
        tree: &BasicPublisher,
        source: &str,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut ret_key = Key(0);

        for i in node.get_children() {
            match tree.get_node(*i).rule {
                //Rules::Plus => {}
                Rules::Nucleus => {
                    let key = Self::nucleus(out_tree, symbol_table, tree, source, *i);
                    ret_key = out_tree.push(Reference::OneOrMore, Some(key), None)
                }

                _ => panic!("one_or_more"),
            }
        }
        ret_key
    }
    fn zero_or_more(
        out_tree: &mut BinaryTreeWO,
        symbol_table: &SymbolTable,
        tree: &BasicPublisher,
        source: &str,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut ret_key = Key(0);

        for i in node.get_children() {
            match tree.get_node(*i).rule {
                //Rules::Star => {}
                Rules::Nucleus => {
                    let key = Self::nucleus(out_tree, symbol_table, tree, source, *i);
                    ret_key = out_tree.push(Reference::ZeroOrMore, Some(key), None)
                }

                _ => panic!("zero_or_more"),
            }
        }
        ret_key
    }

    fn and_predicate(
        out_tree: &mut BinaryTreeWO,
        symbol_table: &SymbolTable,
        tree: &BasicPublisher,
        source: &str,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut ret_key = Key(0);

        for i in node.get_children() {
            match tree.get_node(*i).rule {
                //Rules::Ampersand => {}
                Rules::Nucleus => {
                    let key = Self::nucleus(out_tree, symbol_table, tree, source, *i);
                    ret_key = out_tree.push(Reference::AndPredicate, Some(key), None);
                }

                _ => panic!("and_predicate"),
            }
        }
        ret_key
    }

    fn not_predicate(
        out_tree: &mut BinaryTreeWO,
        symbol_table: &SymbolTable,
        tree: &BasicPublisher,
        source: &str,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut ret_key = Key(0);
        for i in node.get_children() {
            match tree.get_node(*i).rule {
                //Rules::Exclamation_Mark => {}
                Rules::Nucleus => {
                    let key = Self::nucleus(out_tree, symbol_table, tree, source, *i);
                    ret_key = out_tree.push(Reference::NotPredicate, Some(key), None)
                }

                _ => panic!("not_predicate"),
            }
        }
        ret_key
    }

    fn nucleus(
        out_tree: &mut BinaryTreeWO,
        symbol_table: &SymbolTable,
        tree: &BasicPublisher,
        source: &str,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut ret_key = Key(0);
        for i in node.get_children() {
            let child_rule = tree.get_node(*i).rule;
            match child_rule {
                Rules::Subexpression => {
                    ret_key = Self::subexpression(out_tree, symbol_table, tree, source, *i);
                }
                Rules::Terminal => {
                    ret_key = Self::terminal(out_tree, symbol_table, tree, source, *i);
                }
                Rules::Var_Name_Ref => {
                    ret_key = Self::var_name(out_tree, symbol_table, tree, source, *i);
                }
                Rules::OrderedChoiceMatchRange => {
                    ret_key =
                        Self::ordered_choice_match_range(out_tree, symbol_table, tree, source, *i)
                }
                Rules::StringTerminal => {
                    ret_key = Self::string_terminal(out_tree, symbol_table, tree, source, *i);
                }
                _ => {
                    let err_msg = format!("nucleus, Rule: {:?}", child_rule);
                    panic_any(err_msg);
                }
            }
        }
        ret_key
    }

    fn string_terminal(
        out_tree: &mut BinaryTreeWO,
        _symbol_table: &SymbolTable,
        tree: &BasicPublisher,
        source: &str,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut data: Vec<char> = Vec::new();
        for i in node.get_children() {
            let child_node = tree.get_node(*i);
            let child_rule = child_node.rule;
            match child_rule {
                Rules::ASCII => {
                    let contents = source[((child_node.start_position) as usize)
                        ..((child_node.end_position) as usize)]
                        .as_bytes()[0];
                    data.push(contents as char);
                }
                Rules::Integer => {
                    panic!("string_terminal Not yet implemented Integer")
                }
                Rules::Hex => {
                    let contents = &source[((child_node.start_position + 2) as usize)
                        ..((child_node.end_position) as usize)];
                    let char = u32::from_str_radix(contents, 16);
                    data.push(
                        char::from_u32(char.expect("Failed to parse hex correctly"))
                            .expect("Should be valid codepoint"),
                    );
                }
                //Rules::Apostrophe => {}
                _ => {
                    let err_msg = format!("string_terminal, Rule: {:?}", child_rule);
                    panic_any(err_msg);
                }
            }
        }
        let mut all_ascii = true;
        for char in &data {
            if char.len_utf8() != 1 {
                all_ascii = false;
            }
        }
        if all_ascii {
            out_tree.push(Reference::StringTerminalAsciiOpt(data), None, None)
        } else {
            out_tree.push(Reference::StringTerminal(data), None, None)
        }
    }

    fn ordered_choice_match_range(
        out_tree: &mut BinaryTreeWO,
        _symbol_table: &SymbolTable,
        tree: &BasicPublisher,
        source: &str,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut ret_key = Key(0);
        let mut start_set: bool = false;
        let mut value_start: u32 = 0;
        let mut value_end: u32;
        for i in node.get_children() {
            let child_node = tree.get_node(*i);
            let child_rule = child_node.rule;
            match child_rule {
                Rules::Terminal => {
                    let contents = source[((child_node.start_position + 1) as usize)
                        ..((child_node.end_position - 1) as usize)]
                        .to_string();
                    if !start_set {
                        value_start = contents.as_bytes()[0] as u32; // Terminal can only be ascii
                        start_set = true;
                    } else {
                        value_end = contents.as_bytes()[0] as u32; // Terminal can only be ascii
                        ret_key = out_tree.push(
                            Reference::OrderedChoiceMatchRange(value_start, value_end),
                            None,
                            None,
                        )
                    }
                }
                Rules::Integer => {
                    panic!("OrderedChoiceMatchRange Not yet implemented Integer")
                }
                Rules::Hex => {
                    let contents = source[((child_node.start_position + 2) as usize)
                        ..((child_node.end_position) as usize)]
                        .to_string();
                    let f = match u32::from_str_radix(&contents, 16) {
                        Err(e) => {
                            panic!("Failed to parse Hex value: {:?}", e)
                        }
                        Ok(value) => value,
                    };

                    if !start_set {
                        value_start = f; // Terminal can only be ascii
                        start_set = true;
                    } else {
                        value_end = f; // Terminal can only be ascii
                        ret_key = out_tree.push(
                            Reference::OrderedChoiceMatchRange(value_start, value_end),
                            None,
                            None,
                        )
                    }
                }

                _ => {
                    let err_msg = format!("ordered choice match range, Rule: {:?}", child_rule);
                    panic_any(err_msg);
                }
            }
        }

        ret_key
    }

    fn subexpression(
        out_tree: &mut BinaryTreeWO,
        symbol_table: &SymbolTable,
        tree: &BasicPublisher,
        source: &str,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut ret_key = Key(0);
        for i in node.get_children() {
            match tree.get_node(*i).rule {
                Rules::RHS => {
                    let key = Self::rhs(out_tree, symbol_table, tree, source, *i);
                    ret_key = out_tree.push(Reference::Subexpression, Some(key), None);
                }
                //Rules::Left_Bracket | Rules::Right_Bracket => {}
                _ => panic!("subexpression"),
            }
        }
        ret_key
    }

    fn var_name(
        out_tree: &mut BinaryTreeWO,
        symbol_table: &SymbolTable,
        tree: &BasicPublisher,
        source: &str,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let var_name: String = source
            [((node.start_position + 1) as usize)..((node.end_position - 1) as usize)]
            .to_string();

        if symbol_table.check_symbol_is_inline(&var_name) {
            let contents = var_name.to_string();
            out_tree.push(Reference::InlinedRule(contents), None, None)
        } else {
            let contents = var_name.to_string();
            out_tree.push(Reference::VarName(contents), None, None)
        }
    }

    fn terminal(
        out_tree: &mut BinaryTreeWO,
        _symbol_table: &SymbolTable,
        tree: &BasicPublisher,
        source: &str,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        if node.end_position - node.start_position == 2 {
            let key1 = out_tree.push(Reference::Terminal(r#"""#.to_string()), None, None);
            let key2 = out_tree.push(Reference::Terminal(r#"""#.to_string()), None, None);
            out_tree.push(Reference::Sequence, Some(key1), Some(key2))
        } else {
            let mut contents: String = source
                [((node.start_position + 1) as usize)..((node.end_position - 1) as usize)]
                .to_string();
            if contents == "\\" {
                contents = "\\\\".to_string();
            } else if contents == "'" {
                contents = "\\'".to_string();
            }
            let contents = contents.to_string();
            out_tree.push(Reference::Terminal(contents), None, None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::count_lines;
    use ::parser::*;
    use std::cell::RefCell;
    use std::env;
    use std::fs::{canonicalize, read_to_string};

    // #[test]
    // fn test_java_grammar_parser() {
    //     println!("{:?}", env::current_dir().unwrap());
    //     let path = "../java_parser/src/java_grammar_definition_parser.txt";
    //     let pathbuf = canonicalize(path).expect("If it's moved change the string above");
    //     let string = read_to_string(pathbuf).expect("If it's moved change the string above");
    //     let string2 = string.clone();
    //     let src_len = string.len() as u32;
    //     let source = Source::new(string);
    //     let position: u32 = 0;
    //     let context = Context::<MyCache4, Tree>::new(src_len, RULES_SIZE);
    //     let result = grammar(&context, &source, position);

    //     // Checks full file was parsed.
    //     if result.1 != string2.len() as u32 {
    //         panic!(
    //             "Failed to parse grammar due to syntax error on Line: {:?}",
    //             count_lines(&string2, result.1)
    //         )
    //     } else {
    //         println!("Successfully parsed")
    //     }
    //     let tree = context.borrow();
    //     //tree.print(Key(0), None);
    //     let tree = &tree.clear_false();
    //     let src = &String::from(source);
    //     let sym_table = SymbolTable::new(tree, src);
    //     //sym_table.print();
    //     let gen_code = GeneratedCode::new(&sym_table, &tree, src);
    //     println!("{}", gen_code.rules_enum);
    //     for i in gen_code.rules {
    //         println!("{}", i)
    //     }
    // }

    #[test]
    fn test_5() {
        let string = "<Rule>='A'/'B'/'C'/'D';   #   Ein Kommentar   #  ".to_string();
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let result = grammar(Key(0), &context, &source, position);

        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = context.into_inner();
        let tree = &tree.get_publisher().clear_false();

        //tree.print(Key(0), None);
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        //sym_table.print();
        let _gen_code = GeneratedCode::new(&sym_table, &tree, src);
        _gen_code.print();
    }

    #[test]
    fn test_4() {
        let string = "<Rule>='A','B','';   #   Ein Kommentar   #  ".to_string();
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let result = grammar(Key(0), &context, &source, position);

        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = context.into_inner();
        let tree = &tree.get_publisher().clear_false();

        //tree.print(Key(0), None);
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        //sym_table.print();
        let _gen_code = GeneratedCode::new(&sym_table, &tree, src);
        _gen_code.print();
    }

    #[test]
    fn test_2() {
        let string = "<Rule>='A'/'B'/'C';   #   Ein Kommentar   #  ".to_string();
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let result = grammar(Key(0), &context, &source, position);
        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = context.into_inner();
        let tree = &tree.get_publisher().clear_false();
        //tree.print(Key(0), Some(true));
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        //sym_table.print();
        let _gen_code = GeneratedCode::new(&sym_table, &tree, src);
    }

    #[test]
    fn test_3() {
        let string = r#"<Atom> PASSTHROUGH = (<And_Predicate>/<Not_Predicate>/<One_Or_More>/<Zero_Or_More>/<Optional>/<Nucleus>), <Whitespace>;
        "#.to_string();
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let result = grammar(Key(0), &context, &source, position);

        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = context.into_inner();
        let tree = &tree.get_publisher().clear_false();

        //tree.print(Key(0), None);
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        //sym_table.print();
        let _gen_code = GeneratedCode::new(&sym_table, &tree, src);
        _gen_code.print();
    }
    #[test]
    fn test_9() {
        let string = r#"<Whitespace> Inline = (' '/'\n'/'\r'/'\t')*;
            <LHS> = <Var_Name_Decl>, (<Whitespace>, <Semantic_Instructions>, <Whitespace>)?;
        "#
        .to_string();
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let result = grammar(Key(0), &context, &source, position);

        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = context.into_inner();
        let tree = &tree.get_publisher().clear_false();

        //tree.print(Key(0), None);
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        sym_table.print();
        let _gen_code = GeneratedCode::new(&sym_table, &tree, src);
        _gen_code.print();
    }

    #[test]
    fn test() {
        println!("{:?}", env::current_dir().unwrap());
        let path = "../parser/tests/test_data/Grammar.txt";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let result = grammar(Key(0), &context, &source, position);
        //tree.print(Key(0), None);
        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }

        let tree = context.into_inner();
        let tree = &tree.get_publisher().clear_false();
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        //sym_table.print();
        let gen_code = GeneratedCode::new(&sym_table, &tree, src);
        // for i in &gen_code.rules {
        //     println!("{}", i)
        // }
        // println!("{}", gen_code.rules_enum);
        println!("Inlined Rules:");
        sym_table.print_inlined_rules();
        println!("End Inlined Rules: \n");
        gen_code.print();
    }

    // #[test]
    // fn test25() {
    //     println!("{:?}", env::current_dir().unwrap());
    //     let path = "../json_parser/json.dsl";
    //     let pathbuf = canonicalize(path).expect("If it's moved change the string above");
    //     let string = read_to_string(pathbuf).expect("If it's moved change the string above");
    //     let string2 = string.clone();
    //     let src_len = string.len();
    //     let source = Source::new(&string);
    //     let position = 0;
    //     let context = BasicContext::new(src_len, RULES_SIZE as usize);
    //     let context: RefCell<BasicContext> = context.into();
    //     let result = grammar(Key(0), &context, &source, position);
    //     //tree.print(Key(0), None);
    //     // Checks full file was parsed.
    //     if result.1 != string2.len() as u32 {
    //         panic!(
    //             "Failed to parse grammar due to syntax error on Line: {:?}",
    //             count_lines(&string2, result.1)
    //         )
    //     } else {
    //         println!("Successfully parsed")
    //     }
    //     let tree = context.borrow();
    //     let tree = &tree.clear_false();
    //     let src = &String::from(source);
    //     let sym_table = SymbolTable::new(tree, src);
    //     //sym_table.print();
    //     let gen_code = GeneratedCode::new(&sym_table, &tree, src);
    //     for i in gen_code.rules {
    //         println!("{}", i)
    //     }
    //     println!("{}", gen_code.rules_enum)
    // }

    #[test]
    fn test_ordered_choice_match_range() {
        let string = r#"<Atom> PASSTHROUGH = ['A'..'Z'];
        "#
        .to_string();
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let result = grammar(Key(0), &context, &source, position);

        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = context.into_inner();
        let tree = &tree.get_publisher().clear_false();

        tree.print(Key(0), None);
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        sym_table.print();
        let gen_code = GeneratedCode::new(&sym_table, &tree, src);
        for i in gen_code.rules {
            println!("{}", i)
        }
        println!("{}", gen_code.rules_enum)
    }

    #[test]
    fn test_ordered_choice_match_range2() {
        let string = r#"<Atom> = [0x20..0xFF];
        "#
        .to_string();
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let result = grammar(Key(0), &context, &source, position);

        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = context.into_inner();
        let tree = &tree.get_publisher().clear_false();

        tree.print(Key(0), None);
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        sym_table.print();
        let gen_code = GeneratedCode::new(&sym_table, &tree, src);
        for i in gen_code.rules {
            println!("{}", i)
        }
        println!("{}", gen_code.rules_enum)
    }

    #[test]
    fn test_string_terminal() {
        let string = r#"<Atom> PASSTHROUGH = "COLLECT";
        "#
        .to_string();
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let result = grammar(Key(0), &context, &source, position);

        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = context.into_inner();
        let tree = &tree.get_publisher().clear_false();

        tree.print(Key(0), None);
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        sym_table.print();
        let gen_code = GeneratedCode::new(&sym_table, &tree, src);
        for i in gen_code.rules {
            println!("{}", i)
        }
        println!("{}", gen_code.rules_enum)
    }

    #[test]
    fn test_string_terminal_emoji_codepoint() {
        let string = r#"<Atom> PASSTHROUGH = 0x0001F600;
        "#
        .to_string();
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let result = grammar(Key(0), &context, &source, position);
        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = context.into_inner();
        let tree = &tree.get_publisher().clear_false();

        tree.print(Key(0), None);
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        sym_table.print();
        let gen_code = GeneratedCode::new(&sym_table, &tree, src);
        for i in gen_code.rules {
            println!("{}", i)
        }
        println!("{}", gen_code.rules_enum)
    }

    #[test]
    fn test_inline() {
        let string = r#"<Atom> Inline = ['A'..'Z'];
                                <Uses_Atom> = <Atom>;
        "#
        .to_string();
        let string2 = string.clone();
        let src_len = string.len();
        let source = Source::new(&string);
        let position = 0;
        let context = BasicContext::new(src_len, RULES_SIZE as usize);
        let context: RefCell<BasicContext> = context.into();
        let result = grammar(Key(0), &context, &source, position);

        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = context.into_inner();
        let tree = &tree.get_publisher().clear_false();

        tree.print(Key(0), None);
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        sym_table.print();
        sym_table.print_inlined_rules();
        let gen_code = GeneratedCode::new(&sym_table, &tree, src);
        gen_code.print();
    }
}
