use core::panic;

use crate::Cache;
use rules::{Key, Rules};
use std::collections::HashSet;

// We can probably replace the generic HashSet with some form of simple array since there will
// rarely be more than 100 rules or so it may be faster to once again just index into an array instead not sure.
// Potentially even one like IndirectLeftRecursionCache itself where we just index into Rules offset + multiple of position * Rules quantity.
#[derive(Clone, Debug)]
pub struct Head {
    rule: Rules,
    involved_set: HashSet<Rules>,
    eval_set: HashSet<Rules>,
}

impl Head {
    fn new(rule: Rules) -> Self {
        Head {
            rule: rule,
            involved_set: HashSet::new(),
            eval_set: HashSet::new(),
        }
    }
    pub fn get_head_rule(&self) -> Rules {
        self.rule
    }

    pub fn remove_involved_set(&mut self, rule: Rules) -> bool {
        self.involved_set.remove(&rule)
    }

    pub fn push_involved_set(&mut self, rule: Rules) -> bool {
        self.involved_set.insert(rule)
    }

    pub fn push_eval_set(&mut self, rule: Rules) -> bool {
        self.eval_set.insert(rule)
    }

    pub fn set_eval_set(&mut self) {
        // Eval Set must always be set to all rules involved in the left recursion.
        self.eval_set.extend(&self.involved_set);
    }
    pub fn remove_rule_from_eval_set(&mut self, rule: Rules) {
        self.eval_set.remove(&rule);
    }

    pub fn is_not_head_or_involved_set(&self, rule: Rules) -> bool {
        if rule == self.rule {
            return false;
        } else {
            self.involved_set.contains(&rule)
        }
    }
    pub fn is_in_eval_set_remove_if_true(&mut self, rule: Rules) -> bool {
        let val = self.eval_set.contains(&rule);
        if val {
            self.eval_set.remove(&rule);
        }
        val
    }
}

pub struct IndirectLeftRecursionCache {
    end_position: Vec<u32>,
    indexes: Vec<Key>,
    is_true: Vec<Option<bool>>, // Position encoded as start_position*src_length + struct_position // May be slower due to arithmetic who knows
    number_of_structs: u32,
    last_node: Option<Key>,
    lr_detected: Option<Rules>,
    heads: Vec<Option<Head>>, // None if LR is not underway
}
impl Cache for IndirectLeftRecursionCache {
    // Try as flat packed data structure. Since using zero to fill didn't seem to make much difference.
    fn new(size_of_source: u32, number_of_structs: u32) -> IndirectLeftRecursionCache {
        let capacity = (size_of_source + 1) * (number_of_structs + 1);
        let capacity = capacity as usize;
        let mut c = IndirectLeftRecursionCache {
            is_true: Vec::with_capacity(capacity),
            end_position: Vec::with_capacity(capacity),
            indexes: Vec::with_capacity(capacity),
            number_of_structs,
            last_node: None,
            lr_detected: None,
            heads: Vec::with_capacity(capacity),
        };
        c.is_true.resize(capacity, Some(false));
        c.end_position.resize(capacity, 0);
        c.indexes.resize(capacity, Key(u32::MAX));
        c.heads.resize(capacity, None);
        c
        // for every arg cache in c set size to <number_of_structs>
    }
    fn last_node(&self) -> Option<Key> {
        self.last_node
    }
    fn set_last_node(&mut self, key: Option<Key>) {
        self.last_node = key
    }

    fn push_deny_LR(
        &mut self,
        rule: Rules,
        is_true: Option<bool>,
        start_position: u32,
        end_position: u32,
        reference: Key,
    ) {
        let index = (start_position * self.number_of_structs + (rule as u32)) as usize;
        self.is_true[index] = is_true;
        self.end_position[index] = end_position;
        self.indexes[index] = reference;
    }

    fn push(
        &mut self,
        rule: Rules,
        is_true: bool,
        start_position: u32,
        end_position: u32,
        stack_index: Key,
    ) {
        // println!(
        //     "DirectLRCache: Rule: {:?} End Position: {:?}",
        //     rule, end_position
        // );
        let index = (start_position * self.number_of_structs + (rule as u32)) as usize;
        self.is_true[index] = Some(is_true);
        self.end_position[index] = end_position;
        self.indexes[index] = stack_index;
    }

    fn check(&self, rule: Rules, start_position: u32) -> Option<(bool, u32, Key)> {
        panic!("This cache requires the use of the _var_name_LR function.")
    }
    fn check_LR(&self, rule: Rules, start_position: u32) -> Option<(Option<bool>, u32, Key)> {
        let index = (start_position * self.number_of_structs + (rule as u32)) as usize;
        //println!("Index: {:?}, Start_Position: {:?}, Rule: {:?}", index, start_position, rule);
        let is_true: Option<bool> = self.is_true[index];
        let end_position: u32 = self.end_position[index];
        let indexed: Key = self.indexes[index];
        if is_true.is_none() {
            return Some((is_true, end_position, indexed));
        }
        if end_position != 0 {
            // Result is returned to callee to unwrap
            Some((is_true, end_position, indexed))
        } else {
            // Tells callee to simply run the actual code instead of using cached value since one does not exist.
            None
        }
    }
    fn set_lr_detected(&mut self, detected: Option<Rules>) {
        self.lr_detected = detected;
    }
    fn get_lr_detected(&self, rule: Rules) -> bool {
        self.lr_detected.is_some()
    }

    fn set_indirect_lr_detected(&mut self, detected: Rules, start_position: u32) {
        self.heads[start_position as usize] = Some(Head::new(detected));
    }
    fn get_indirect_lr_detected(&mut self, start_position: u32) -> Option<&mut Head> {
        let f = self.heads[start_position as usize]
            .as_mut()
            .expect("Shouldn't be calling this function where it can fail.");
        return Some(f);
    }

    fn clear(&mut self) {}
    fn reinitialize(&mut self) {
        self.end_position.fill(0); // Because 0 is used as a marker of whether a cache position has been used. Maybe a bug if a rule evaluates to end_position 0 like !<ASCII> but for now oh well.
    }
}
