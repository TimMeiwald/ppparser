use crate::Cache;
use core::panic;
use rules::{Key, Rules};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

// Trait set below works with direct left recursion for reference.
// // use crate::indirect_left_recursion_cache::Head;

// pub trait Cache {
//     fn new(size_of_source: u32, number_of_structs: u32) -> Self;
//     fn push(
//         &mut self,
//         rule: Rules,
//         is_true: bool,
//         start_position: u32,
//         end_position: u32,
//         reference: ASTOrLR,
//     );
//     fn set_astOrLrAndPosition(
//         &mut self,
//         rule: Rules,
//         start_position: u32,
//         reference: ASTOrLR,
//         end_position: u32,
//     );

//     fn check_lr(&mut self, rule: Rules, start_position: u32) -> Option<&MemoEntry>;
//     fn set_lr_detected(&mut self, rule: Rules, start_position: u32, detected: LR);
//     fn get_lr_detected(&self, rule: Rules, start_position: u32) -> bool;
//     // // fn set_indirect_lr_detected(&mut self, detected: Rules, start_position: u32);
//     // // fn get_indirect_lr_detected(&mut self, start_position: u32) -> Option<&mut Head>;
//     // fn check(&self, rule: Rules, start_position: u32) -> Option<(bool, u32, Key)>;
//     fn clear(&mut self);
//     fn reinitialize(&mut self); //Reset state without deallocating memory for reuse.
//                                 // fn last_node(&self) -> Option<Key>;
//                                 // fn set_last_node(&mut self, key: Option<Key>);
//                                 // fn set_is_fail(&mut self, rule: Rules, start_position: u32, is_fail: bool);
//                                 // fn get_is_fail(&self, rule: Rules, start_position: u32) -> bool;
// }

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LR {
    pub detected: bool,
}
impl LR {
    pub fn new(detected: bool) -> Self {
        LR { detected }
    }
}
#[derive(Debug, Eq, PartialEq, Copy, Clone)]

pub enum ASTOrLR {
    LR(LR),
    AST(AST),
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

struct Head {
    pub recursion_setup_flag: bool,
    pub recursion_execution_flag: bool,
    pub involved_set: BTreeSet<Rules>,
    pub eval_set: BTreeSet<Rules>,
    pub active_left_recursion_rule: Rules,
    pub involved_stack: VecDeque<Rules>,
}
impl Head {
    fn new() -> Self {
        Head {
            recursion_setup_flag: false,
            recursion_execution_flag: false,
            involved_set: BTreeSet::new(),
            eval_set: BTreeSet::new(),
            active_left_recursion_rule: Rules::Grammar,
            involved_stack: VecDeque::new(),
        }
    }

    fn set_active_rule(&mut self, rule: Rules) {
        self.active_left_recursion_rule = rule;
    }
    fn get_active_rule(&self) -> Rules {
        self.active_left_recursion_rule
    }
    fn eval_set_is_empty(&self) -> bool {
        self.eval_set.is_empty()
    }
    fn remove_from_eval_set(&mut self, rule: Rules) {
        self.eval_set.remove(&rule);
    }
    fn print_eval_set(&self) {
        println!("Eval Set: {:?}", self.eval_set);
    }
    fn is_in_eval_set(&self, rule: Rules) -> bool {
        self.eval_set.contains(&rule)
    }
    fn print_involved_set(&self) {
        println!("Involved Stack: {:?}", self.involved_stack)
    }
    fn set_recursion_execution_flag(&mut self) {
        self.recursion_execution_flag = true;
    }
    fn reset_recursion_execution_flag(&mut self) {
        self.recursion_execution_flag = false;
    }
    fn get_recursion_execution_flag(&self) -> bool {
        self.recursion_execution_flag
    }
    fn insert_into_involved_set(&mut self, rule: Rules) -> bool {
        println!(
            "In Insert Into Involved Set: Active Rule : {:?}; Rule: {:?}",
            self.active_left_recursion_rule, rule
        );
        if !self.involved_stack.contains(&rule) {
            self.involved_stack.push_front(rule);
            self.involved_set.insert(rule);
            println!("Involved Set: {:?}", self.involved_set);
            println!("Involved Stack: {:?}", self.involved_stack);
            return true;
        } else {
            return false;
        }
    }
    fn get_recursion_setup_flag(&self) -> bool {
        return self.recursion_setup_flag;
    }
    fn copy_involved_set_into_eval_set(&mut self) {
        self.eval_set.clone_from(&self.involved_set);
        println!("Copied involved set into eval set");
        self.print_eval_set();
        self.print_involved_set();
    }

    fn reset_recursion_setup_flag(&mut self) {
        self.involved_set.remove(&self.active_left_recursion_rule);
        self.print_involved_set();
        self.recursion_setup_flag = false;
    }
    fn set_recursion_setup_flag(&mut self) {
        self.recursion_setup_flag = true;
    }
}

pub struct DirectLeftRecursionCache {
    memo_entries: HashMap<(Rules, u32), MemoEntry>,
    heads: HashMap<u32, Option<Head>>,
}

impl Cache for DirectLeftRecursionCache {
    fn new(size_of_source: u32, number_of_structs: u32) -> DirectLeftRecursionCache {
        DirectLeftRecursionCache {
            memo_entries: HashMap::new(),
            heads: HashMap::new(),
        }
    }
    fn set_active_rule(&mut self, rule: Rules, position: u32) {
        // Called before anything else usign head so we create one here for the relevant position.
        self.heads.insert(position, Some(Head::new()));
        println!("Active Left Recursion Rule: {:?} at {:?}", rule, position);
        self.heads
            .get_mut(&position)
            .expect("Should not get called until after head already exists.")
            .as_mut()
            .unwrap()
            .set_active_rule(rule);
    }
    fn get_active_rule(&self, position: u32) -> Rules {
        let head = self.heads.get(&position).unwrap().as_ref().unwrap();
        head.get_active_rule()
    }
    fn eval_set_is_empty(&self, position: u32) -> bool {
        let head = self.heads.get(&position).unwrap().as_ref().unwrap();
        head.eval_set_is_empty()
    }
    fn remove_from_eval_set(&mut self, rule: Rules, position: u32) {
        self.heads
            .get_mut(&position)
            .unwrap()
            .as_mut()
            .unwrap()
            .remove_from_eval_set(rule);
    }
    fn print_eval_set(&self, position: u32) {
        let head = self.heads.get(&position).unwrap().as_ref().unwrap();
        head.print_eval_set();
    }
    fn is_in_eval_set(&self, rule: Rules, position: u32) -> bool {
        let head = self.heads.get(&position).unwrap().as_ref().unwrap();
        head.is_in_eval_set(rule)
    }
    fn print_involved_set(&self, position: u32) {
        let head = self.heads.get(&position).unwrap().as_ref().unwrap();
        head.print_involved_set();
    }
    fn set_recursion_execution_flag(&mut self, position: u32) {
        self.heads
            .get_mut(&position)
            .unwrap()
            .as_mut()
            .unwrap()
            .set_recursion_execution_flag();
    }
    fn reset_recursion_execution_flag(&mut self, position: u32) {
        self.heads
            .get_mut(&position)
            .unwrap()
            .as_mut()
            .unwrap()
            .reset_recursion_execution_flag();
    }
    fn get_recursion_execution_flag(&self, position: u32) -> bool {
        let head = self.heads.get(&position).unwrap().as_ref().unwrap();
        head.get_recursion_execution_flag()
    }
    fn insert_into_involved_set(&mut self, rule: Rules, position: u32) -> bool {
        self.heads
            .get_mut(&position)
            .unwrap()
            .as_mut()
            .unwrap()
            .insert_into_involved_set(rule)
    }
    fn get_recursion_setup_flag(&self, position: u32) -> bool {
        let head = self.heads.get(&position).unwrap().as_ref().unwrap();
        head.get_recursion_setup_flag()
    }
    fn copy_involved_set_into_eval_set(&mut self, position: u32) {
        self.heads
            .get_mut(&position)
            .unwrap()
            .as_mut()
            .unwrap()
            .copy_involved_set_into_eval_set();
    }

    fn reset_recursion_setup_flag(&mut self, position: u32) {
        self.heads
            .get_mut(&position)
            .unwrap()
            .as_mut()
            .unwrap()
            .reset_recursion_setup_flag();
    }
    fn set_recursion_setup_flag(&mut self, position: u32) {
        self.heads
            .get_mut(&position)
            .unwrap()
            .as_mut()
            .unwrap()
            .set_recursion_setup_flag();
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
        let memo_entry = self
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

    use super::{DirectLeftRecursionCache, AST};

    #[test]
    fn test_cache() {
        let mut cache = DirectLeftRecursionCache::new(0, 0);
        cache.push(
            Rules::Num,
            true,
            0,
            0,
            crate::ASTOrLR::LR(crate::LR { detected: true }),
        );
        let f = cache.get_lr_detected(Rules::Num, 0);
        println!("{:?}", f);
        assert!(false);
    }
}
