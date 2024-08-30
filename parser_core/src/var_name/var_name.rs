#![allow(clippy::type_complexity)]

use std::any::Any;
use std::borrow::BorrowMut;
use std::mem::ManuallyDrop;
use std::process::exit;
use std::u32;

// While complex still under development and the core of the entire program is here so complexity is
// Acceptable
use super::_var_name_indirect::_var_name_kernel_indirect_lr;
use crate::publisher_utils::*;
use crate::source::Source;
use crate::Context;
use cache::ASTOrLR;
use cache::Cache;
use cache::MemoEntry;
use cache::AST;
use cache::LR;
use publisher::Publisher;
use rules::{Key, Rules};
pub fn _var_name<T: Cache, S: Publisher>(
    rule: Rules,
    context: &Context<T, S>,
    func: fn(&Context<T, S>, &Source, u32) -> (bool, u32, AST),
) -> impl Fn(&Source, u32) -> (bool, u32, AST) + '_ {
    move |source: &Source, position: u32| {
        _var_name_kernel_indirect_lr(rule, context, source, position, func)
    }
}
