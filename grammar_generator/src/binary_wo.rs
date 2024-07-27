use core::panic;
use std::fmt::format;

use crate::count_lines;
use crate::symbol_table::SymbolTable;
use publisher::Node;
use publisher::Publisher;
use publisher::Tree;
use rules::Key;
use rules::Rules;

#[derive(Debug, Clone)]
pub enum Reference {
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

    OrderedChoiceMatchRange(u32, u32), // Custom Code for optimization purposes
    StringTerminal(Vec<char>),
    StringTerminalAsciiOpt(Vec<char>),
}

pub struct BinaryTree_WO {
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
            "    ".repeat(indent),
            index,
            node.reference,
            node.lhs,
            node.rhs
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

    fn match_ref(&self, stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        match node.reference {
            Reference::AndPredicate => self.and_predicate(stack, index),
            Reference::NotPredicate => self.not_predicate(stack, index),
            Reference::Optional => self.optional(stack, index),
            Reference::OneOrMore => self.one_or_more(stack, index),
            Reference::ZeroOrMore => self.zero_or_more(stack, index),
            Reference::Subexpression => self.subexpression(stack, index),

            Reference::OrderedChoice => self.ordered_choice(stack, index),
            Reference::Sequence => self.sequence(stack, index),

            Reference::Terminal(_) => self.terminal(stack, index),
            Reference::VarName(_) => self.var_name(stack, index),
            Reference::OrderedChoiceMatchRange(_, _) => {
                self.ordered_choice_match_range(stack, index)
            }
            Reference::StringTerminal(_) => self.string_terminal(stack, index),
            Reference::StringTerminalAsciiOpt(_) => self.string_terminal_ascii_opt(stack, index),
            Reference::Exec | Reference::NULL => {
                panic!("Exec should only exist once and NULL should never exist")
            }
        }
    }

    fn to_string_kernel(&self, stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        let key = match node.reference {
            Reference::Exec => {
                let child_index = node.lhs.expect("Should always have child");
                let key = self.match_ref(stack, child_index);
                stack.push(format!("closure_{}(source, position)", key.0));
                key
            }

            _ => panic!("Invalid Key Index. Must be Exec"),
        };
        key
    }

    fn string_terminal(&self, stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        match &node.reference {
            Reference::StringTerminal(chars) => {
                stack.push(format!(
                    "let closure_{:?} = _string_terminal(&{:?});",
                    index.0, chars
                ));
                index
            }
            _ => {
                panic!("Shouldn't happen string terminal")
            }
        }
    }

    fn string_terminal_ascii_opt(&self, stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        match &node.reference {
            Reference::StringTerminalAsciiOpt(chars) => {
                let mut r: String =
                    format!("let closure_{:?} = _string_terminal_opt_ascii(&[", index.0);

                for i in chars {
                    r.push_str(&format!("b{:?},", i))
                }
                r.pop();
                r.push_str("]);");
                stack.push(r);
                index
            }
            _ => {
                panic!("Shouldn't happen ascii opt string terminal")
            }
        }
    }

    fn ordered_choice_match_range(&self, stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        match &node.reference {
            Reference::OrderedChoiceMatchRange(start, end) => {
                stack.push(format!(
                    "let closure_{:?} = _ordered_choice_match_range(b'{}', b'{}');",
                    index.0, start, end
                ));
                index
            }
            _ => {
                panic!("Shouldn't happen")
            }
        }
    }

    fn and_predicate(&self, stack: &mut Vec<String>, index: Key) -> Key {
        {
            let node = &self.nodes[usize::from(index)];
            let key1 = self.match_ref(stack, node.lhs.expect("Should always have a left node"));
            match node.reference {
                Reference::AndPredicate => {
                    stack.push(format!(
                        "let closure_{:?} = _and_predicate(&closure_{:?});",
                        index.0, key1.0
                    ));
                    index
                }
                _ => {
                    panic!("Shouldn't happen")
                }
            }
        }
    }
    fn not_predicate(&self, stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        let key1 = self.match_ref(stack, node.lhs.expect("Should always have a left node"));
        match node.reference {
            Reference::NotPredicate => {
                stack.push(format!(
                    "let closure_{:?} = _not_predicate(&closure_{:?});",
                    index.0, key1.0
                ));
                index
            }
            _ => {
                panic!("Shouldn't happen")
            }
        }
    }

    fn one_or_more(&self, stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        let key1 = self.match_ref(stack, node.lhs.expect("Should always have a left node"));
        match node.reference {
            Reference::OneOrMore => {
                stack.push(format!(
                    "let closure_{:?} = _one_or_more(&closure_{:?});",
                    index.0, key1.0
                ));
                index
            }
            _ => {
                panic!("Shouldn't happen")
            }
        }
    }
    fn zero_or_more(&self, stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        let key1 = self.match_ref(stack, node.lhs.expect("Should always have a left node"));
        match node.reference {
            Reference::ZeroOrMore => {
                stack.push(format!(
                    "let closure_{:?} = _zero_or_more(&closure_{:?});",
                    index.0, key1.0
                ));
                index
            }
            _ => {
                panic!("Shouldn't happen")
            }
        }
    }
    fn subexpression(&self, stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        let key1 = self.match_ref(stack, node.lhs.expect("Should always have a left node"));
        match node.reference {
            Reference::Subexpression => {
                stack.push(format!(
                    "let closure_{:?} = _subexpression(&closure_{:?});",
                    index.0, key1.0
                ));
                index
            }
            _ => {
                panic!("Shouldn't happen")
            }
        }
    }
    fn optional(&self, stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        let key1 = self.match_ref(stack, node.lhs.expect("Should always have a left node"));
        match node.reference {
            Reference::Optional => {
                stack.push(format!(
                    "let closure_{:?} = _optional(&closure_{:?});",
                    index.0, key1.0
                ));
                index
            }
            _ => {
                panic!("Shouldn't happen")
            }
        }
    }

    fn ordered_choice(&self, stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        let key1 = self.match_ref(stack, node.lhs.expect("Should always have a left node"));
        let key2 = self.match_ref(stack, node.rhs.expect("Should always have a right node"));
        match node.reference {
            Reference::OrderedChoice => {
                stack.push(format!(
                    "let closure_{:?} = _ordered_choice(&closure_{:?}, &closure_{:?});",
                    index.0, key1.0, key2.0
                ));
                index
            }
            _ => {
                panic!("Shouldn't happen")
            }
        }
    }

    fn sequence(&self, stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        let key1 = self.match_ref(stack, node.lhs.expect("Should always have a left node"));
        let key2 = self.match_ref(stack, node.rhs.expect("Should always have a right node"));
        match node.reference {
            Reference::Sequence => {
                stack.push(format!(
                    "let closure_{:?} = _sequence(&closure_{:?}, &closure_{:?});",
                    index.0, key1.0, key2.0
                ));
                index
            }
            _ => {
                panic!("Shouldn't happen")
            }
        }
    }
    fn terminal(&self, stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        match &node.reference {
            Reference::Terminal(content) => {
                stack.push(format!(
                    "let closure_{:?} = _terminal(b'{}');",
                    index.0, content
                ));
                index
            }
            _ => {
                panic!("Shouldn't happen")
            }
        }
    }
    fn var_name(&self, stack: &mut Vec<String>, index: Key) -> Key {
        let node = &self.nodes[usize::from(index)];
        match &node.reference {
            Reference::VarName(content) => {
                let contents = format!(
                    "let closure_{:?} = _var_name(Rules::{}, context, {});",
                    index.0,
                    content,
                    content.to_lowercase()
                );
                stack.push(contents);
                index
            }
            _ => {
                panic!("Shouldn't happen")
            }
        }
    }
}

pub struct BinaryNode {
    reference: Reference,
    lhs: Option<Key>,
    rhs: Option<Key>,
}
