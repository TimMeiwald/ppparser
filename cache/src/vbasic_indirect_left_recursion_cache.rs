use crate::Cache;
use core::panic;
use rules::{Key, Rules};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ASTOrLR {
    LR(LR),
    AST(AST),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum AST {
    FAIL,
    SUCCESS(Key),
    IGNORE,
}
impl Into<Key> for AST {
    fn into(self) -> Key {
        return match self {
            AST::FAIL => panic!("Should never happen"),
            AST::IGNORE => panic!("Should never happen 2"),
            AST::SUCCESS(key) => key,
        };
    }
}

pub struct Head {
    rule: Rules,
    involved_set: HashSet<Rules>,
    eval_set: HashSet<Rules>,
}
pub struct Heads {
    heads: HashMap<u32, Head>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LR {
    pub detected: bool,
}
impl LR {
    pub fn new(detected: bool) -> Self {
        LR { detected }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct MemoEntry {
    pub position: u32,
    pub ast_or_lr: ASTOrLR,
    pub is_true: bool,
}
impl MemoEntry {
    pub fn new(ast_or_lr: ASTOrLR, position: u32, is_true: bool) -> Self {
        MemoEntry {
            position,
            ast_or_lr,
            is_true,
        }
    }
}

impl Into<AST> for ASTOrLR {
    fn into(self) -> AST {
        match self {
            ASTOrLR::LR(_) => panic!("Not an AST"),
            ASTOrLR::AST(ast) => return ast,
        }
    }
}

pub struct IndirectLeftRecursionCache {
    memo_entries: HashMap<(Rules, u32), MemoEntry>,
}
impl Cache for IndirectLeftRecursionCache {
    fn new(size_of_source: u32, number_of_structs: u32) -> IndirectLeftRecursionCache {
        IndirectLeftRecursionCache {
            memo_entries: HashMap::new(),
        }
    }

    fn check_lr(&mut self, rule: Rules, start_position: u32) -> Option<&MemoEntry> {
        self.memo_entries.get(&(rule, start_position))
    }
    fn set_astOrLrAndPosition(
        &mut self,
        rule: Rules,
        start_position: u32,
        reference: ASTOrLR,
        end_position: u32,
    ) {
        let mut memo_entry = self
            .memo_entries
            .get_mut(&(rule, start_position))
            .expect("If using this should exist");
        memo_entry.ast_or_lr = reference;
        memo_entry.position = end_position;
    }
    fn push(
        &mut self,
        rule: Rules,
        is_true: bool,
        start_position: u32,
        end_position: u32,
        reference: ASTOrLR,
    ) {
        self.memo_entries.insert(
            (rule, start_position),
            MemoEntry {
                position: end_position,
                ast_or_lr: reference,
                is_true: is_true,
            },
        );
    }
    fn clear(&mut self) {
        self.memo_entries.clear();
    }
    fn reinitialize(&mut self) {}
    fn get_lr_detected(&self, rule: Rules, start_position: u32) -> bool {
        let memo_entry = self
            .memo_entries
            .get(&(rule, start_position))
            .expect("Should exist");
        return match &memo_entry.ast_or_lr {
            ASTOrLR::AST(_) => false,
            ASTOrLR::LR(lr) => lr.detected,
        };
    }
    fn set_lr_detected(&mut self, rule: Rules, start_position: u32, detected: LR) {
        let memo_entry: &mut MemoEntry = self
            .memo_entries
            .get_mut(&(rule, start_position))
            .expect("Should exist by the time this is in use. ");
        memo_entry.ast_or_lr = ASTOrLR::LR(detected);
    }
}

#[cfg(test)]
mod tests {
    use ::rules::{Key, Rules};
    use rules::rules;

    use crate::Cache;

    use super::{ASTOrLR, IndirectLeftRecursionCache, AST};

    #[test]
    fn test_cache() {
        let mut cache = IndirectLeftRecursionCache::new(0, 0);
        cache.push(
            Rules::Num,
            true,
            0,
            0,
            super::ASTOrLR::LR(crate::LR { detected: true }),
        );
        let f = cache.get_lr_detected(Rules::Num, 0);
        println!("{:?}", f);
        assert!(false);
    }
}
