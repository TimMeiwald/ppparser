use core::panic;

use crate::Cache;
use rules::{Key, Rules};
#[derive(Debug, Eq, PartialEq)]

pub enum AST {
    FAIL,
    SUCCESS(Key),
    IGNORE,
}

#[derive(Clone, Copy)]
pub struct LR {
    detected: bool,
}
impl LR {
    pub fn new(detected: bool) -> Self {
        LR { detected }
    }
}
pub enum ASTOrLR {
    LR(LR),
    AST(AST),
}

#[derive(Debug)]
pub struct MemoEntry {
    pub position: u32,
    pub ast: AST,
    pub is_true: bool,
}
impl MemoEntry {
    pub fn new(ast: AST, position: u32, is_true: bool) -> Self {
        MemoEntry {
            position,
            ast,
            is_true,
        }
    }
}
// This cache will completely flatten the cache to see if that improves performance.
pub struct DirectLeftRecursionCache {
    end_position: Vec<u32>,
    indexes: Vec<Key>,
    is_true: Vec<bool>, // Position encoded as start_position*src_length + struct_position // May be slower due to arithmetic who knows
    is_fail: Vec<bool>,
    number_of_structs: u32,
    last_node: Option<Key>,
    lr_detected: Vec<LR>,
}
// TODO: Last Node should probably be in the publisher not in Cache. Irrelevant to caching per se.
impl Cache for DirectLeftRecursionCache {
    // Try as flat packed data structure. Since using zero to fill didn't seem to make much difference.
    fn new(size_of_source: u32, number_of_structs: u32) -> DirectLeftRecursionCache {
        let capacity = (size_of_source + 1) * (number_of_structs + 1);
        let capacity = capacity as usize;
        let mut c = DirectLeftRecursionCache {
            is_true: Vec::with_capacity(capacity),
            is_fail: Vec::with_capacity(capacity),
            end_position: Vec::with_capacity(capacity),
            indexes: Vec::with_capacity(capacity),
            number_of_structs,
            last_node: None,
            lr_detected: Vec::with_capacity(capacity),
        };
        c.is_true.resize(capacity, false);
        c.end_position.resize(capacity, 0);
        c.indexes.resize(capacity, Key(u32::MAX));
        c.is_fail.resize(capacity, false);
        c.lr_detected.resize(capacity, LR::new(false));
        c
        // for every arg cache in c set size to <number_of_structs>
    }
    fn last_node(&self) -> Option<Key> {
        self.last_node
    }
    fn set_last_node(&mut self, key: Option<Key>) {
        self.last_node = key
    }
    fn set_lr_detected(&mut self, rule: Rules, start_position: u32, detected: LR) {
        let index = (start_position * self.number_of_structs + (rule as u32)) as usize;
        self.lr_detected[index] = detected;
    }
    fn get_lr_detected(&self, rule: Rules, start_position: u32) -> LR {
        let index = (start_position * self.number_of_structs + (rule as u32)) as usize;
        self.lr_detected[index]
    }

    fn get_is_fail(&self, rule: Rules, start_position: u32) -> bool {
        let index = (start_position * self.number_of_structs + (rule as u32)) as usize;
        self.is_fail[index]
    }

    fn set_is_fail(&mut self, rule: Rules, start_position: u32, is_fail: bool) {
        let index = (start_position * self.number_of_structs + (rule as u32)) as usize;
        self.is_fail[index] = is_fail;
    }

    fn push(
        &mut self,
        rule: Rules,
        is_true: bool,
        start_position: u32,
        end_position: u32,
        reference: ASTOrLR,
    ) {
        let index = (start_position * self.number_of_structs + (rule as u32)) as usize;
        self.is_true[index] = is_true;
        self.end_position[index] = end_position;

        match reference {
            ASTOrLR::AST(ast) => match ast {
                AST::FAIL => self.is_fail[index] = true,
                AST::SUCCESS(key) => {
                    self.is_fail[index] = false;
                    self.indexes[index] = key;
                }
                AST::IGNORE => panic!("Should never occur I think albeit not sure!"),
            },
            ASTOrLR::LR(lr) => {
                self.lr_detected[index] = lr;
            }
        }
    }

    fn check(&self, rule: Rules, start_position: u32) -> Option<(bool, u32, Key)> {
        panic!("This cache requires the use of the _var_name_LR function.")
    }
    fn check_lr(&self, rule: Rules, start_position: u32) -> Option<MemoEntry> {
        let index = (start_position * self.number_of_structs + (rule as u32)) as usize;
        //println!("Index: {:?}, Start_Position: {:?}, Rule: {:?}", index, start_position, rule);
        let is_true: bool = self.is_true[index];
        let end_position: u32 = self.end_position[index];
        let is_fail: bool = self.is_fail[index];
        let indexed: Key = self.indexes[index];

        if is_fail {
            // AST has been set to FAIL as per paper.
            return Some(MemoEntry::new(AST::FAIL, end_position, is_true));
        } else if end_position != 0 {
            // Result is returned to callee to unwrap
            Some(MemoEntry::new(AST::SUCCESS(indexed), end_position, is_true))
        } else {
            // Nil, I.E there is no cached entry
            None
        }
    }

    fn clear(&mut self) {}
    fn reinitialize(&mut self) {
        self.end_position.fill(0);
    }
    // fn set_indirect_lr_detected(&mut self, detected: Rules, start_position: u32) {}
    // fn get_indirect_lr_detected(&mut self, start_position: u32) -> Option<&mut Head> {
    //     panic!("Cannot use this cache for Indirect Left Recursion")
    // }
}
