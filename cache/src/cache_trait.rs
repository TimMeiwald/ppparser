use rules::{Key, Rules};

use crate::{
    vbasic_direct_left_recursion_cache::{ASTOrLR, MemoEntry, AST},
    LR,
};

// use crate::indirect_left_recursion_cache::Head;

pub trait Cache {
    fn new(size_of_source: u32, number_of_structs: u32) -> Self;
    fn push(
        &mut self,
        rule: Rules,
        is_true: bool,
        start_position: u32,
        end_position: u32,
        reference: ASTOrLR,
    );
    fn set_astOrLrAndPosition(
        &mut self,
        rule: Rules,
        start_position: u32,
        reference: ASTOrLR,
        end_position: u32,
    );
    fn check_lr(&mut self, rule: Rules, start_position: u32) -> Option<&MemoEntry>;
    fn set_lr_detected(&mut self, rule: Rules, start_position: u32, detected: LR);
    fn get_lr_detected(&self, rule: Rules, start_position: u32) -> bool;
    // // fn set_indirect_lr_detected(&mut self, detected: Rules, start_position: u32);
    // // fn get_indirect_lr_detected(&mut self, start_position: u32) -> Option<&mut Head>;
    // fn check(&self, rule: Rules, start_position: u32) -> Option<(bool, u32, Key)>;
    fn clear(&mut self);
    fn reinitialize(&mut self); //Reset state without deallocating memory for reuse.
                                // fn last_node(&self) -> Option<Key>;
                                // fn set_last_node(&mut self, key: Option<Key>);
                                // fn set_is_fail(&mut self, rule: Rules, start_position: u32, is_fail: bool);
                                // fn get_is_fail(&self, rule: Rules, start_position: u32) -> bool;
}
