use crate::{Key, Rules};
use std::collections::{BTreeSet, HashMap};

#[derive(Debug)]
pub struct Head {
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
    heads: HashMap<u32, Head>,
}

impl BasicCache {
    pub fn new() -> Self {
        BasicCache {
            cache: HashMap::new(),
            left_recursion_cache: HashMap::new(),
            heads: HashMap::new(),
        }
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
    pub fn insert_direct_lr(
        &mut self,
        rule: Rules,
        is_true: bool,
        start_position: u32,
        end_position: u32,
        key: Key,
        lr: LR,
    ) {
        self.left_recursion_cache
            .insert((rule, start_position), (is_true, end_position, key, lr));
    }
    pub fn check_direct_lr(
        &self,
        rule: Rules,
        start_position: u32,
    ) -> Option<(bool, u32, Key, LR)> {
        self.left_recursion_cache
            .get(&(rule, start_position))
            .copied()
    }

    pub fn check_head(&self, start_position: u32) -> Option<Rules> {
        let head = self.heads.get(&start_position);
        #[allow(clippy::manual_map)]
        match head {
            None => None, // Not yet exists therefore no Left Recursion at this position
            Some(head) => Some(head.active_left_recursion_rule), // Return head rule since that's necessary information.
        }
    }

    pub fn set_head(
        &mut self,
        start_position: u32,
        head_rule: Rules,
        involved_set: BTreeSet<Rules>,
    ) {
        let eval_set = involved_set.clone();
        self.heads.insert(
            start_position,
            Head {
                active_left_recursion_rule: head_rule,
                involved_set,
                eval_set,
            },
        );
    }
    pub fn reinitialize_eval_set(&mut self, start_position: u32) {
        let head = self
            .heads
            .get_mut(&start_position)
            .expect("Should exist by now");
        head.eval_set = head.involved_set.clone();
    }

    pub fn rule_in_eval_set(&self, start_position: u32, rule: Rules) -> bool {
        // Return true if it's in eval set and not currently active left recursive rule.
        let head = self
            .heads
            .get(&start_position)
            .expect("Should always exist when calling rule_in_eval_set");
        println!("Rule: {:?}\nEval Set: {:?}", rule, head.eval_set);
        head.eval_set.contains(&rule)
    }
    pub fn remove_from_eval_set(&mut self, start_position: u32, rule: Rules) {
        self.heads
            .get_mut(&start_position)
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
