use std::fmt::format;

use crate::count_lines;
use crate::symbol_table::SymbolTable;
use publisher::Node;
use publisher::Publisher;
use publisher::Tree;
use rules::Key;
use rules::Rules;

#[derive(Debug, Clone)]
enum Reference {
    Terminal(String), // For Terminals
    VarName(String),

    OneOrMore,
    ZeroOrMore,
    Optional,
    AndPredicate,
    NotPredicate,
    Subexpression,

    OrderedChoice,
    Sequence,
    Exec,
    NULL,
}

struct BinaryTree_WO {
    nodes: Vec<BinaryNode>,
}

impl BinaryTree_WO {
    pub fn new() -> Self {
        let mut tree = BinaryTree_WO {
            nodes: Vec::<BinaryNode>::new(),
        };
        tree.push(Reference::NULL, None, None);
        tree
    }

    pub fn push(&mut self, reference: Reference, lhs: Option<Key>, rhs: Option<Key>) -> Key {
        let node = BinaryNode {
            reference,
            lhs,
            rhs,
        };
        let key = Key(self.nodes.len());
        println!("{:?} {:?} {:?} {:?}", &node.reference, lhs, rhs, key);

        self.nodes.push(node);
        key
    }

    pub fn print(&self, index: Key) {
        self.print_kernel(index, 0)
    }

    fn print_kernel(&self, index: Key, indent: usize) {
        let node = &self.nodes[usize::from(index)];
        println!(
            "{} Key: {:?}, Reference: {:?}, LHS: {:?}, RHS: {:?}",
            "    ".repeat(indent),index, node.reference, node.lhs, node.rhs
        );
        let child_index = node.lhs;
        if child_index.is_some() {
            self.print_kernel(child_index.unwrap(), indent + 1);
        }
        let child_index = node.rhs;
        if child_index.is_some() {
            self.print_kernel(child_index.unwrap(), indent + 1);
        }
    }

    pub fn to_string(&self, index: Key) -> Vec<String> {
        let mut stack = Vec::<String>::new();
        self.to_string_kernel(&mut stack, index);
        stack
    }

    fn match_ref(&self, mut stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        match node.reference {
            Reference::AndPredicate => {
                self.and_predicate(stack, index)
            },
            Reference::NotPredicate => {
                self.not_predicate(stack, index)
            } ,
            Reference::Optional  => {
                self.optional(stack, index)
            },
            Reference::OneOrMore  => {
                self.one_or_more(stack, index)
            },
            Reference::ZeroOrMore  => {
                self.zero_or_more(stack, index)
            },
            Reference::Subexpression => {
                self.subexpression(stack, index)
            }

            Reference::OrderedChoice  => {
                self.ordered_choice(stack, index)
            },
            Reference::Sequence  => {
                self.sequence(stack, index)
            },

            Reference::Terminal(_)  => {
                self.terminal(stack, index)
            },
            Reference::VarName(_)  => {
                self.var_name(stack, index)
            },

            Reference::Exec | Reference::NULL => panic!("Exec should only exist once and NULL should never exist")
        }
    }
    

    fn to_string_kernel(&self, mut stack: &mut Vec<String>, index: Key) -> Key { 
        let node = &self.nodes[usize::from(index)];

        let key = match node.reference {
            Reference::Exec  => {
                let child_index = node.lhs.expect("Should always have child");
                let child = &self.nodes[usize::from(child_index)];
                self.match_ref(stack, child_index)
                },
            
            _ => panic!("Invalid Key Index. Must be Exec")
        };
        // Push Exec here
        key 
    }

    fn and_predicate(&self, mut stack: &mut Vec<String>, index: Key) -> Key {
        {
            let node = &self.nodes[usize::from(index)];
            let key1 = self.match_ref(stack, node.lhs.expect("Should always have a left node"));
            match node.reference {
                Reference::AndPredicate => {
                    stack.push(format!("let closure_{:?} = _and_predicate(&closure_{:?})", index, key1));
                    index
                }
                _ => {panic!("Shouldn't happen")}
            }
        }
    }
    fn not_predicate(&self, mut stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        let key1 = self.match_ref(stack, node.lhs.expect("Should always have a left node"));
        match node.reference {
            Reference::NotPredicate => {
                stack.push(format!("let closure_{:?} = _not_predicate(&closure_{:?})", index, key1));
                index
            }
            _ => {panic!("Shouldn't happen")}
        }
    }
    
    fn one_or_more(&self, mut stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        let key1 = self.match_ref(stack, node.lhs.expect("Should always have a left node"));
        match node.reference {
            Reference::OneOrMore => {
                stack.push(format!("let closure_{:?} = _one_or_more(&closure_{:?})", index, key1));
                index
            }
            _ => {panic!("Shouldn't happen")}
        }
    }
    fn zero_or_more(&self, mut stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        let key1 = self.match_ref(stack, node.lhs.expect("Should always have a left node"));
        match node.reference {
            Reference::ZeroOrMore => {
                stack.push(format!("let closure_{:?} = _zero_or_more(&closure_{:?})", index, key1));
                index
            }
            _ => {panic!("Shouldn't happen")}
        }
    }
    fn subexpression(&self, mut stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        let key1 = self.match_ref(stack, node.lhs.expect("Should always have a left node"));
        match node.reference {
            Reference::Subexpression => {
                stack.push(format!("let closure_{:?} = _subexpression(&closure_{:?})", index, key1));
                index
            }
            _ => {panic!("Shouldn't happen")}
        }
    }
    fn optional(&self, mut stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        let key1 = self.match_ref(stack, node.lhs.expect("Should always have a left node"));
        match node.reference {
            Reference::Optional => {
                stack.push(format!("let closure_{:?} = _optional(&closure_{:?})", index, key1));
                index
            }
            _ => {panic!("Shouldn't happen")}
        }
    }

    fn ordered_choice(&self, mut stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        let key1 = self.match_ref(stack, node.lhs.expect("Should always have a left node"));
        let key2 = self.match_ref(stack, node.lhs.expect("Should always have a right node"));
        match node.reference {
            Reference::OrderedChoice => {
                stack.push(format!("let closure_{:?} = _ordered_choice(&closure_{:?}, &closure_{:?})", index, key1, key2));
                index
            }
            _ => {panic!("Shouldn't happen")}
        }
    }
    
    fn sequence(&self, mut stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        let key1 = self.match_ref(stack, node.lhs.expect("Should always have a left node"));
        let key2 = self.match_ref(stack, node.lhs.expect("Should always have a right node"));
        match node.reference {
            Reference::Sequence => {
                stack.push(format!("let closure_{:?} = _sequence(&closure_{:?}, &closure_{:?})", index, key1, key2));
                index
            }
            _ => {panic!("Shouldn't happen")}
        }
    }
    fn terminal(&self, mut stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        match &node.reference {
            Reference::Terminal(content) => {
                stack.push(format!("let closure_{:?} = _terminal(b'{}')", index, content));
                index
            }
            _ => {panic!("Shouldn't happen")}
        }


    }
    fn var_name(&self, mut stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        match &node.reference {
            Reference::VarName(content) => {
                let contents = format!(
                    "let closure_{:?} = _var_name(Rule::{}, context, {});",
                    index,
                    content,
                    content.to_lowercase()
                );
                stack.push(contents);
                index
            }
            _ => {panic!("Shouldn't happen")}
        }
    }
    

}


struct BinaryNode {
    reference: Reference,
    lhs: Option<Key>,
    rhs: Option<Key>,
}

struct GeneratedCode {
    // String per rule so we can seperate into files per rule.
    output: Vec<String>,
}

impl GeneratedCode {
    pub fn new(symbol_table: &SymbolTable, tree: &Tree, source: &String) -> Self {
        let s = GeneratedCode {
            output: Vec::<String>::new(),
        };
        println!("Generating Code");
        s.generate(symbol_table, tree, source);
        s
    }

    fn generate(&self, symbol_table: &SymbolTable, tree: &Tree, source: &String) {
        let node = tree.get_node(Key(0));
        if node.rule != Rules::Grammar {
            panic!("Invalid Root. Must be of type Rules::Grammar");
        }
        let mut counter = 0;
        loop {
            if counter >= node.get_children().len() {
                break;
            }
            let child_index = node.get_children()[counter];
            // Recurse.
            let rule = self.match_rule(symbol_table, tree, source, child_index);
            println!("{}", rule);
            counter += 1;
        }
    }
    fn generate_kernel(
        &self,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) {
        let node = tree.get_node(index);
        let mut counter = 0;
        if node.rule == Rules::VarNameDecl && node.result {
            let name =
                &source[((node.start_position + 1) as usize)..((node.end_position - 1) as usize)];
            println!("VarNameDecl: {:?}", name);
        }
        loop {
            if counter >= node.get_children().len() {
                break;
            }
            let child_index = node.get_children()[counter];
            // Recurse.
            self.generate_kernel(symbol_table, tree, source, child_index);
            counter += 1;
        }
    }

    fn match_rule(
        &self,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) -> String {
        let node = tree.get_node(index);
        match node.rule {
            Rules::Rule => self.rule(symbol_table, tree, source, index),
            _ => {
                todo!("Not yet implemented")
            }
        }
    }

    fn rule(&self, symbol_table: &SymbolTable, tree: &Tree, source: &String, index: Key) -> String {
        let rule_node = tree.get_node(index);
        let rule_children = rule_node.get_children();
        let mut name: Option<String> = None;
        let mut rhs: String = "".to_string();
        let mut comment: Option<String> = None;
        tree.print(index, Some(true));
        for i in rule_children {
            match tree.get_node(*i).rule {
                Rules::Lhs => {
                    name = Some(self.lhs(symbol_table, tree, source, *i));
                }
                Rules::Rhs => {
                    let mut out_tree = BinaryTree_WO::new();
                    let rhs_key = self.rhs(&mut out_tree, symbol_table, tree, source, *i);
                    println!("{:?}", rhs_key);
                    let last_key = out_tree.push(Reference::Exec, Some(rhs_key), None);
                    out_tree.print(last_key);

                    let mut result = "".to_string();
                    for i in out_tree.to_string(rhs_key){
                        result.push_str("\t");
                        result.push_str(&i);
                        result.push_str("\n");
                    }
                }
                Rules::Comment => {
                    comment = Some(self.comment(symbol_table, tree, source, *i));
                }
                _ => {}
            }
        }

        let rule_header = format!(
            "pub fn {}<T: Cache, S: Publisher>(
            _context: &Context<T, S>,
            source: &Source,
            position: u32,
        ) -> (bool, u32) {{",
            name.expect("Must have name")
        );
        let builder = format!(
            "{}\n{}\n{} ",
            rule_header,
            comment.unwrap_or("".to_string()),
            rhs
        );

        return builder;
    }

    fn lhs(&self, symbol_table: &SymbolTable, tree: &Tree, source: &String, index: Key) -> String {
        let lhs_node = tree.get_node(index);
        let lhs_children = lhs_node.get_children();
        let var_name_decl_key = lhs_children[0];
        let var_name_decl = tree.get_node(var_name_decl_key);
        let var_name_key = var_name_decl.get_children()[0];
        let var_name = tree.get_node(var_name_key);
        let name = var_name.get_string(source);
        // name still has angle brackets but cba to create string from subnodes
        // so simply remove first and last char
        let mut s = name;
        s.pop(); // remove last
        if s.len() > 0 {
            s.remove(0); // remove first
        }
        s
    }

    fn comment(
        &self,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) -> String {
        let comment_node = tree.get_node(index);
        let max_comment_size = (comment_node.end_position - comment_node.start_position) as usize;
        let acc = String::with_capacity(max_comment_size + 3);
        let mut start: Option<u32> = None;
        let mut end: Option<u32> = None;
        for key in comment_node.get_children() {
            match tree.get_node(*key).rule {
                Rules::Ascii => {
                    if start.is_none() {
                        start = Some(tree.get_node(*key).start_position);
                    }

                    end = Some(tree.get_node(*key).end_position);
                }
                _ => {}
            };
        }
        let acc = "// ".to_string()
            + &source[(start.expect("Must have start") as usize)
                ..(end.expect("Must have end") as usize)]
                .to_string();
        acc
    }

    fn rhs(
        &self,
        mut out_tree: &mut BinaryTree_WO,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) -> Key {
        let rhs_node = tree.get_node(index);
        let mut ret_key = Key(0);
        for i in rhs_node.get_children() {
            match tree.get_node(*i).rule {
                Rules::OrderedChoice => {
                    ret_key = self.ordered_choice(&mut out_tree, symbol_table, tree, source, *i);
                }
                Rules::Sequence => {
                    ret_key = self.sequence(&mut out_tree, symbol_table, tree, source, *i);
                }
                Rules::Atom => ret_key = self.atom(&mut out_tree, symbol_table, tree, source, *i),
                _ => return panic!("rhs"),
            }
        }
        ret_key
    }

    fn ordered_choice(
        &self,
        mut out_tree: &mut BinaryTree_WO,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) -> Key {
        let oc_node = tree.get_node(index);
        let mut count = 0;

        let mut last_key = Key(0);
        for i in oc_node.get_children() {
            match tree.get_node(*i).rule {
                Rules::Atom => {
                    let mut key = self.atom(out_tree, symbol_table, tree, source, *i);
                    if count != 0 {
                        key = out_tree.push(
                            Reference::OrderedChoice,
                            Some(last_key),
                            Some(key),
                        )
                    }
                    count += 1;
                    last_key = key;
                }
                Rules::Backslash | Rules::Whitespace => {}
                _ => panic!("ordered_choice"),
            }
        }
        last_key
    }

    fn sequence(
        &self,
        mut out_tree: &mut BinaryTree_WO,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) -> Key {
        let oc_node = tree.get_node(index);
        let mut count = 0;

        let mut last_key = Key(0);
        for i in oc_node.get_children() {
            match tree.get_node(*i).rule {
                Rules::Atom => {
                    let mut key = self.atom(out_tree, symbol_table, tree, source, *i);
                    if count != 0 {
                        key = out_tree.push(
                            Reference::Sequence,
                            Some(last_key),
                            Some(key),
                        );
                    }
                    count += 1;
                    last_key = key;
                }
                Rules::Comma | Rules::Whitespace => {}
                _ => panic!("sequence"),
            }
        }
        last_key
    }

    fn atom(
        &self,
        mut out_tree: &mut BinaryTree_WO,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut ret_key = Key(0);

        for i in node.get_children() {
            match tree.get_node(*i).rule {
                Rules::AndPredicate => {
                    ret_key = self.and_predicate(out_tree, symbol_table, tree, source, *i);
                }
                Rules::NotPredicate => {
                    ret_key = self.not_predicate(out_tree, symbol_table, tree, source, *i);
                }
                Rules::OneOrMore => {
                    ret_key = self.one_or_more(out_tree, symbol_table, tree, source, *i);
                }
                Rules::ZeroOrMore => {
                    ret_key = self.zero_or_more(out_tree, symbol_table, tree, source, *i);
                }
                Rules::Optional => {
                    ret_key = self.optional(out_tree, symbol_table, tree, source, *i);
                }
                Rules::Nucleus => {
                    ret_key = self.nucleus(out_tree, symbol_table, tree, source, *i);
                }

                _ => panic!("atom"),
            }
        }
        ret_key
    }

    fn optional(
        &self,
        mut out_tree: &mut BinaryTree_WO,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut ret_key = Key(0);

        for i in node.get_children() {
            match tree.get_node(*i).rule {
                Rules::Whitespace | Rules::QuestionMark => {}
                Rules::Nucleus => {
                    let key = self.nucleus(out_tree, symbol_table, tree, source, *i);
                    ret_key =
                        out_tree.push(Reference::Optional, Some(key), None)
                }

                _ => panic!("optional"),
            }
        }
        ret_key
    }
    fn one_or_more(
        &self,
        mut out_tree: &mut BinaryTree_WO,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut ret_key = Key(0);

        for i in node.get_children() {
            match tree.get_node(*i).rule {
                Rules::Whitespace | Rules::Plus => {}
                Rules::Nucleus => {
                    let key = self.nucleus(out_tree, symbol_table, tree, source, *i);
                    ret_key = out_tree.push(
                        Reference::OneOrMore,
                        Some(key),
                        None,
                    )
                }

                _ => panic!("one_or_more"),
            }
        }
        ret_key
    }
    fn zero_or_more(
        &self,
        mut out_tree: &mut BinaryTree_WO,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut ret_key = Key(0);

        for i in node.get_children() {
            match tree.get_node(*i).rule {
                Rules::Whitespace | Rules::Star => {}
                Rules::Nucleus => {
                    let key = self.nucleus(out_tree, symbol_table, tree, source, *i);
                    ret_key = out_tree.push(
                        Reference::ZeroOrMore,
                        Some(key),
                        None,
                    )
                }

                _ => panic!("zero_or_more"),
            }
        }
        ret_key
    }

    fn and_predicate(
        &self,
        mut out_tree: &mut BinaryTree_WO,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut ret_key = Key(0);

        for i in node.get_children() {
            match tree.get_node(*i).rule {
                Rules::Whitespace | Rules::Ampersand => {}
                Rules::Nucleus => {
                    let key = self.nucleus(out_tree, symbol_table, tree, source, *i);
                    ret_key = out_tree.push(
                        Reference::AndPredicate,
                        Some(key),
                        None,
                    );
                }

                _ => panic!("and_predicate"),
            }
        }
        ret_key
    }

    fn not_predicate(
        &self,
        mut out_tree: &mut BinaryTree_WO,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut ret_key = Key(0);
        for i in node.get_children() {
            match tree.get_node(*i).rule {
                Rules::Whitespace | Rules::ExclamationMark => {}
                Rules::Nucleus => {
                    let key = self.nucleus(out_tree, symbol_table, tree, source, *i);
                    ret_key = out_tree.push(
                        Reference::NotPredicate,
                        Some(key),
                        None,
                    )
                }

                _ => panic!("not_predicate"),
            }
        }
        ret_key
    }

    fn nucleus(
        &self,
        mut out_tree: &mut BinaryTree_WO,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut ret_key = Key(0);
        for i in node.get_children() {
            match tree.get_node(*i).rule {
                Rules::Subexpression => {
                    ret_key = self.subexpression(out_tree, symbol_table, tree, source, *i);
                }
                Rules::Terminal => {
                    ret_key = self.terminal(out_tree, symbol_table, tree, source, *i);
                }
                Rules::VarName => {
                    ret_key = self.var_name(out_tree, symbol_table, tree, source, *i);
                }

                _ => panic!("nucleus"),
            }
        }
        ret_key
    }

    fn subexpression(
        &self,
        mut out_tree: &mut BinaryTree_WO,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let mut ret_key = Key(0);
        for i in node.get_children() {
            match tree.get_node(*i).rule {
                Rules::Rhs => {
                    let key = self.rhs(out_tree, symbol_table, tree, source, *i);
                    ret_key = out_tree.push(
                        Reference::Subexpression,
                        Some(key),
                        None,
                    );
                }
                Rules::LeftBracket | Rules::RightBracket => {}
                _ => panic!("subexpression"),
            }
        }
        ret_key
    }

    fn var_name(
        &self,
        mut out_tree: &mut BinaryTree_WO,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        let contents: String = source
            [((node.start_position + 1) as usize)..((node.end_position - 1) as usize)]
            .to_string();

        let contents = format!(
            "{}",
            contents,
        );
        out_tree.push(Reference::VarName(contents), None, None)
    }

    fn terminal(
        &self,
        mut out_tree: &mut BinaryTree_WO,
        symbol_table: &SymbolTable,
        tree: &Tree,
        source: &String,
        index: Key,
    ) -> Key {
        let node = tree.get_node(index);
        if node.end_position - node.start_position == 2 {
            let key1 = out_tree.push(
                Reference::Terminal(r#"""#.to_string()),
                None,
                None,
            );
            let key2 = out_tree.push(
                Reference::Terminal(r#"""#.to_string()),
                None,
                None,
            );
            out_tree.push(
                Reference::Sequence,
                Some(key1),
                Some(key2),
            )
        } else {
            let contents: String = source
                [((node.start_position + 1) as usize)..((node.end_position - 1) as usize)]
                .to_string();
            let contents = format!("{}", contents);
            out_tree.push(Reference::Terminal(contents), None, None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cache::MyCache4;
    use grammar_parser::grammar;
    use parser_core::Context;
    use parser_core::Source;
    use publisher::Tree;
    use std::env;
    use std::fs::{canonicalize, read_to_string};


    #[test]
    fn test_5() {
        let string =  "<Rule>=\"A\"/\"B\"/\"C\"/\"D\";   #   Ein Kommentar   #  ".to_string();
        let string2 = string.clone();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, Tree>::new(src_len, 45);
        let result = grammar(&context, &source, position);

        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = &context.stack.borrow();
        let tree = &tree.clear_false();

        //tree.print(Key(0), None);
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        //sym_table.print();
        let gen_code = GeneratedCode::new(&sym_table, &tree, src);
    }


    #[test]
    fn test_4() {
        let string =  "<Rule>=\"A\",\"B\",\"\";   #   Ein Kommentar   #  ".to_string();
        let string2 = string.clone();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, Tree>::new(src_len, 45);
        let result = grammar(&context, &source, position);

        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = &context.stack.borrow();
        let tree = &tree.clear_false();

        //tree.print(Key(0), None);
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        //sym_table.print();
        let gen_code = GeneratedCode::new(&sym_table, &tree, src);
    }

    #[test]
    fn test_2() {
        let string = "<Rule>=\"A\"/\"B\"/\"\";   #   Ein Kommentar   #  ".to_string();
        let string2 = string.clone();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, Tree>::new(src_len, 45);
        let result = grammar(&context, &source, position);

        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = &context.stack.borrow();
        let tree = &tree.clear_false();

        //tree.print(Key(0), None);
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        //sym_table.print();
        let gen_code = GeneratedCode::new(&sym_table, &tree, src);
    }

    #[test]
    fn test_3() {
        let string = r#"<Atom> PASSTHROUGH = (<And_Predicate>/<Not_Predicate>/<One_Or_More>/<Zero_Or_More>/<Optional>/<Nucleus>), <Whitespace>;
        "#.to_string();
        let string2 = string.clone();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, Tree>::new(src_len, 45);
        let result = grammar(&context, &source, position);

        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = &context.stack.borrow();
        let tree = &tree.clear_false();

        //tree.print(Key(0), None);
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        //sym_table.print();
        let gen_code = GeneratedCode::new(&sym_table, &tree, src);
    }

    #[test]
    fn test() {
        println!("{:?}", env::current_dir().unwrap());
        let path = "../parser_core/tests/Grammar.txt";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");
        let string2 = string.clone();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, Tree>::new(src_len, 45);
        let result = grammar(&context, &source, position);

        // Checks full file was parsed.
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let tree = &context.stack.borrow();
        //tree.print(Key(0), None);
        let tree = &tree.clear_false();
        let src = &String::from(source);
        let sym_table = SymbolTable::new(tree, src);
        //sym_table.print();
        let gen_code = GeneratedCode::new(&sym_table, &tree, src);
    }
}
