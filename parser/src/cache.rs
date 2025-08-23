use crate::{Key, Rules};
use std::collections::VecDeque;
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone)]
pub struct Head {
    pub start_position: u32,
    pub active_left_recursion_rule: Rules,
    pub involved_set: BTreeSet<Rules>,
    pub eval_set: BTreeSet<Rules>,
}
#[derive(Debug, Copy, Clone)]
pub enum LR {
    Set,
    Unset,
}

#[derive(Debug)]
pub struct BasicCache {
    cache: HashMap<(Rules, u32), (bool, u32, Key)>,
    left_recursion_cache: HashMap<(Rules, u32), (bool, u32, Key, LR)>,
    heads: HashMap<(Rules, u32), Head>,
    current_active_left_recursion: Option<(Rules, u32)>,
}

impl Default for BasicCache {
    fn default() -> Self {
        Self::new()
    }
}

impl BasicCache {
    pub fn new() -> Self {
        BasicCache {
            cache: HashMap::new(),
            left_recursion_cache: HashMap::new(),
            heads: HashMap::new(),
            current_active_left_recursion: None,
        }
    }

    pub fn get_current_active_lr_position(&self) -> Option<(Rules, u32)> {
        self.current_active_left_recursion
    }
    pub fn set_current_active_lr_position(&mut self, active_lr: Option<(Rules, u32)>) {
        self.current_active_left_recursion = active_lr;
    }

    pub fn insert(
        &mut self,
        rule: Rules,
        is_true: bool,
        start_position: u32,
        end_position: u32,
        key: Key,
    ) {
        self.cache
            .insert((rule, start_position), (is_true, end_position, key));
    }
    pub fn check(&self, rule: Rules, start_position: u32) -> Option<(bool, u32, Key)> {
        self.cache.get(&(rule, start_position)).copied()
    }
    
    pub fn check_head(&self, rule: Rules, start_position: u32) -> Option<&Head> {
        let head = self.heads.get(&(rule, start_position));
        #[allow(clippy::manual_map)]
        match head {
            None => None,             // Not yet exists therefore no Left Recursion at this position
            Some(head) => Some(head), // Return head rule since that's necessary information.
        }
    }

    pub fn set_head(
        &mut self,
        start_position: u32,
        head_rule: Rules,
        involved_set: BTreeSet<Rules>,
    ) {
        let eval_set = involved_set.clone();
        let head = Head {
            start_position,
            active_left_recursion_rule: head_rule,
            involved_set,
            eval_set,
        };
        self.heads.insert((head_rule, start_position), head);
    }

    pub fn reinitialize_eval_set(&mut self, rule: Rules, start_position: u32) {
        let head = self
            .heads
            .get_mut(&(rule, start_position))
            .expect("Should exist by now");
        head.eval_set = head.involved_set.clone();
    }

    pub fn rule_in_involved_set(&self, head_index: (Rules, u32), rule: Rules) -> bool {
        // Return true if it's in eval set

        let head = self
            .heads
            .get(&head_index)
            .expect("Should always exist when calling rule_in_involved_set");

        head.involved_set.contains(&rule)
    }

    pub fn rule_in_eval_set(&self, head_index: (Rules, u32), rule: Rules) -> bool {
        // Return true if it's in eval set
        let head = self
            .heads
            .get(&head_index)
            .expect("Should always exist when calling rule_in_eval_set");
        head.eval_set.contains(&rule)
    }
    pub fn remove_from_eval_set(&mut self, head_index: (Rules, u32), rule: Rules) {
        self.heads
            .get_mut(&head_index)
            .expect("Should exist by now")
            .eval_set
            .remove(&rule);
    }
}

#[allow(dead_code)]
#[derive(Debug)]

pub struct DirectLeftRecursionCache;
#[allow(dead_code)]
#[derive(Debug)]

pub struct IndirectLeftRecursionCache;
