pub use crate::and_predicate::_AndPredicate;
use crate::cache::cache_fn_wrapper;
use crate::cache::cache_struct_wrapper;
use crate::cache::Cache;
pub use crate::not_predicate::_NotPredicate;
pub use crate::one_or_more::_OneOrMore;
pub use crate::optional::_Optional;
pub use crate::ordered_choice::_OrderedChoice;
pub use crate::sequence::_Sequence;
pub use crate::subexpression::_SubExpression;
pub use crate::terminal::Resolvable;
pub use crate::terminal::_Terminal;
use crate::terminal::token;
pub use crate::var_name::_VarName;
pub use crate::zero_or_more::_ZeroOrMore;
pub use crate:: output_stack::{Stack, StackEntry};




#[derive(Debug)]
pub enum Rules{AlphabetUpper,
AlphabetLower,
Num,
Spaces,
Specials,
Ascii,
Apostrophe,
LeftAngleBracket,
RightAngleBracket,
LeftBracket,
RightBracket,
Assignment,
EndRule,
Ampersand,
ExclamationMark,
Plus,
Star,
QuestionMark,
Comma,
Backslash,
VarName,
Subexpression,
Epsilon,
Lterminal,
Nucleus,
Atom,
AndPredicate,
NotPredicate,
Sequence,
OrderedChoice,
OneOrMore,
ZeroOrMore,
Optional,
Whitespace,
Rhs,
Lhs,
Rule,
Grammar,
Comment,
SemanticInstructions,
Delete,
Passthrough,
Collect,
}


    #[derive(Copy, Clone)]
    pub struct AlphabetUpper;
    impl Resolvable for AlphabetUpper {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"A".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"B".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"C".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"D".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"E".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"F".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"G".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"H".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"I".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"J".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"K".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"L".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"M".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"N".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"O".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"P".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"Q".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"R".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"S".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"T".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"U".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"V".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"W".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"X".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"Y".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"Z".to_string().as_bytes()[0]}};
        let e: StackEntry = StackEntry{rule: Rules::AlphabetUpper, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::AlphabetUpper as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct AlphabetLower;
    impl Resolvable for AlphabetLower {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"a".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"b".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"c".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"d".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"e".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"f".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"g".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"h".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"i".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"j".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"k".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"l".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"m".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"n".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"o".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"p".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"q".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"r".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"s".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"t".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"u".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"v".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"w".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"x".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"y".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"z".to_string().as_bytes()[0]}};
        let e: StackEntry = StackEntry{rule: Rules::AlphabetLower, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::AlphabetLower as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Num;
    impl Resolvable for Num {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"0".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"1".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"2".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"3".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"4".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"5".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"6".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"7".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"8".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"9".to_string().as_bytes()[0]}};
        let e: StackEntry = StackEntry{rule: Rules::Num, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Num as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Spaces;
    impl Resolvable for Spaces {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"\n".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"\t".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"\r".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:" ".to_string().as_bytes()[0]}};
        let e: StackEntry = StackEntry{rule: Rules::Spaces, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Spaces as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Specials;
    impl Resolvable for Specials {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"+".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"*".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"-".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"&".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"!".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"?".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"<".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:">".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:'"'.to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"(".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:")".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"_".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:",".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"/".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:";".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"=".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:'\\'.to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"#".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:":".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"|".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:".".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"{".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"}".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"[".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"]".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"%".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"'".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"^".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"~".to_string().as_bytes()[0]}};
        let e: StackEntry = StackEntry{rule: Rules::Specials, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Specials as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Ascii;
    impl Resolvable for Ascii {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:AlphabetLower}, arg_rhs:_VarName{arg:AlphabetUpper}}, arg_rhs:_VarName{arg:Num}}, arg_rhs:_VarName{arg:Spaces}}, arg_rhs:_VarName{arg:Specials}};
        let e: StackEntry = StackEntry{rule: Rules::Ascii, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Ascii as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Apostrophe;
    impl Resolvable for Apostrophe {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Terminal{arg:'"'.to_string().as_bytes()[0]};
        let e: StackEntry = StackEntry{rule: Rules::Apostrophe, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Apostrophe as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct LeftAngleBracket;
    impl Resolvable for LeftAngleBracket {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Terminal{arg:"<".to_string().as_bytes()[0]};
        let e: StackEntry = StackEntry{rule: Rules::LeftAngleBracket, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::LeftAngleBracket as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct RightAngleBracket;
    impl Resolvable for RightAngleBracket {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Terminal{arg:">".to_string().as_bytes()[0]};
        let e: StackEntry = StackEntry{rule: Rules::RightAngleBracket, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::RightAngleBracket as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct LeftBracket;
    impl Resolvable for LeftBracket {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Terminal{arg:"(".to_string().as_bytes()[0]};
        let e: StackEntry = StackEntry{rule: Rules::LeftBracket, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::LeftBracket as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct RightBracket;
    impl Resolvable for RightBracket {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Terminal{arg:")".to_string().as_bytes()[0]};
        let e: StackEntry = StackEntry{rule: Rules::RightBracket, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::RightBracket as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Assignment;
    impl Resolvable for Assignment {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Terminal{arg:"=".to_string().as_bytes()[0]};
        let e: StackEntry = StackEntry{rule: Rules::Assignment, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Assignment as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct EndRule;
    impl Resolvable for EndRule {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Terminal{arg:";".to_string().as_bytes()[0]};
        let e: StackEntry = StackEntry{rule: Rules::EndRule, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::EndRule as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Ampersand;
    impl Resolvable for Ampersand {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Terminal{arg:"&".to_string().as_bytes()[0]};
        let e: StackEntry = StackEntry{rule: Rules::Ampersand, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Ampersand as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct ExclamationMark;
    impl Resolvable for ExclamationMark {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Terminal{arg:"!".to_string().as_bytes()[0]};
        let e: StackEntry = StackEntry{rule: Rules::ExclamationMark, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::ExclamationMark as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Plus;
    impl Resolvable for Plus {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Terminal{arg:"+".to_string().as_bytes()[0]};
        let e: StackEntry = StackEntry{rule: Rules::Plus, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Plus as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Star;
    impl Resolvable for Star {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Terminal{arg:"*".to_string().as_bytes()[0]};
        let e: StackEntry = StackEntry{rule: Rules::Star, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Star as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct QuestionMark;
    impl Resolvable for QuestionMark {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Terminal{arg:"?".to_string().as_bytes()[0]};
        let e: StackEntry = StackEntry{rule: Rules::QuestionMark, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::QuestionMark as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Comma;
    impl Resolvable for Comma {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Terminal{arg:",".to_string().as_bytes()[0]};
        let e: StackEntry = StackEntry{rule: Rules::Comma, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Comma as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Backslash;
    impl Resolvable for Backslash {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Terminal{arg:"/".to_string().as_bytes()[0]};
        let e: StackEntry = StackEntry{rule: Rules::Backslash, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Backslash as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct VarName;
    impl Resolvable for VarName {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:LeftAngleBracket}, arg_rhs:_SubExpression{arg:_OrderedChoice{arg_lhs:_VarName{arg:AlphabetLower}, arg_rhs:_VarName{arg:AlphabetUpper}}}}, arg_rhs:_ZeroOrMore{arg: _SubExpression{arg:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:AlphabetLower}, arg_rhs:_VarName{arg:AlphabetUpper}}, arg_rhs:_Terminal{arg:"_".to_string().as_bytes()[0]}}}}}, arg_rhs:_VarName{arg:RightAngleBracket}};
        let e: StackEntry = StackEntry{rule: Rules::VarName, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::VarName as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Subexpression;
    impl Resolvable for Subexpression {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:LeftBracket}, arg_rhs:_VarName{arg:Rhs}}, arg_rhs:_VarName{arg:RightBracket}};
        let e: StackEntry = StackEntry{rule: Rules::Subexpression, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Subexpression as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Epsilon;
    impl Resolvable for Epsilon {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_VarName{arg:Apostrophe}, arg_rhs:_VarName{arg:Apostrophe}};
        let e: StackEntry = StackEntry{rule: Rules::Epsilon, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Epsilon as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Lterminal;
    impl Resolvable for Lterminal {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_SubExpression{arg:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Apostrophe}, arg_rhs:_VarName{arg:Ascii}}, arg_rhs:_VarName{arg:Apostrophe}}}, arg_rhs:_SubExpression{arg:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Apostrophe}, arg_rhs:_Terminal{arg:'\\'.to_string().as_bytes()[0]}}, arg_rhs:_SubExpression{arg:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"n".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"r".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"t".to_string().as_bytes()[0]}}}}, arg_rhs:_VarName{arg:Apostrophe}}}}, arg_rhs:_VarName{arg:Epsilon}};
        let e: StackEntry = StackEntry{rule: Rules::Lterminal, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Lterminal as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Nucleus;
    impl Resolvable for Nucleus {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_SubExpression{arg:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:Subexpression}, arg_rhs:_VarName{arg:Lterminal}}, arg_rhs:_VarName{arg:VarName}}}, arg_rhs:_VarName{arg:Whitespace}};
        let e: StackEntry = StackEntry{rule: Rules::Nucleus, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Nucleus as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Atom;
    impl Resolvable for Atom {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_SubExpression{arg:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:AndPredicate}, arg_rhs:_VarName{arg:NotPredicate}}, arg_rhs:_VarName{arg:OneOrMore}}, arg_rhs:_VarName{arg:ZeroOrMore}}, arg_rhs:_VarName{arg:Optional}}, arg_rhs:_VarName{arg:Nucleus}}}, arg_rhs:_VarName{arg:Whitespace}};
        let e: StackEntry = StackEntry{rule: Rules::Atom, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Atom as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct AndPredicate;
    impl Resolvable for AndPredicate {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_VarName{arg:Ampersand}, arg_rhs:_VarName{arg:Nucleus}};
        let e: StackEntry = StackEntry{rule: Rules::AndPredicate, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::AndPredicate as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct NotPredicate;
    impl Resolvable for NotPredicate {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_VarName{arg:ExclamationMark}, arg_rhs:_VarName{arg:Nucleus}};
        let e: StackEntry = StackEntry{rule: Rules::NotPredicate, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::NotPredicate as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Sequence;
    impl Resolvable for Sequence {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Atom}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Comma}}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Atom}}, arg_rhs:_ZeroOrMore{arg: _SubExpression{arg:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Comma}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Atom}}}}};
        let e: StackEntry = StackEntry{rule: Rules::Sequence, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Sequence as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct OrderedChoice;
    impl Resolvable for OrderedChoice {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Atom}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Backslash}}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Atom}}, arg_rhs:_ZeroOrMore{arg: _SubExpression{arg:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Backslash}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Atom}}}}};
        let e: StackEntry = StackEntry{rule: Rules::OrderedChoice, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::OrderedChoice as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct OneOrMore;
    impl Resolvable for OneOrMore {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Nucleus}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Plus}};
        let e: StackEntry = StackEntry{rule: Rules::OneOrMore, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::OneOrMore as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct ZeroOrMore;
    impl Resolvable for ZeroOrMore {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Nucleus}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Star}};
        let e: StackEntry = StackEntry{rule: Rules::ZeroOrMore, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::ZeroOrMore as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Optional;
    impl Resolvable for Optional {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Nucleus}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:QuestionMark}};
        let e: StackEntry = StackEntry{rule: Rules::Optional, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Optional as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Whitespace;
    impl Resolvable for Whitespace {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _ZeroOrMore{arg: _SubExpression{arg:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:" ".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"\n".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"\r".to_string().as_bytes()[0]}}}};
        let e: StackEntry = StackEntry{rule: Rules::Whitespace, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Whitespace as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Rhs;
    impl Resolvable for Rhs {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:Sequence}, arg_rhs:_VarName{arg:OrderedChoice}}, arg_rhs:_VarName{arg:Atom}};
        let e: StackEntry = StackEntry{rule: Rules::Rhs, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Rhs as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Lhs;
    impl Resolvable for Lhs {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_VarName{arg:VarName}, arg_rhs:_Optional{arg: _SubExpression{arg:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Whitespace}, arg_rhs:_VarName{arg:SemanticInstructions}}, arg_rhs:_VarName{arg:Whitespace}}}}};
        let e: StackEntry = StackEntry{rule: Rules::Lhs, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Lhs as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Rule;
    impl Resolvable for Rule {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Lhs}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Assignment}}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Rhs}}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:EndRule}}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_ZeroOrMore{arg: _VarName{arg:Comment}}};
        let e: StackEntry = StackEntry{rule: Rules::Rule, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Rule as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Grammar;
    impl Resolvable for Grammar {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_OneOrMore{arg: _VarName{arg:Rule}}, arg_rhs:_VarName{arg:Whitespace}};
        let e: StackEntry = StackEntry{rule: Rules::Grammar, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Grammar as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Comment;
    impl Resolvable for Comment {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Whitespace}, arg_rhs:_Terminal{arg:"#".to_string().as_bytes()[0]}}, arg_rhs:_ZeroOrMore{arg: _SubExpression{arg:_Sequence{arg_lhs:_NotPredicate{arg: _Terminal{arg:"#".to_string().as_bytes()[0]}}, arg_rhs:_VarName{arg:Ascii}}}}}, arg_rhs:_Terminal{arg:"#".to_string().as_bytes()[0]}}, arg_rhs:_VarName{arg:Whitespace}};
        let e: StackEntry = StackEntry{rule: Rules::Comment, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Comment as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct SemanticInstructions;
    impl Resolvable for SemanticInstructions {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:Delete}, arg_rhs:_VarName{arg:Passthrough}}, arg_rhs:_VarName{arg:Collect}};
        let e: StackEntry = StackEntry{rule: Rules::SemanticInstructions, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::SemanticInstructions as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Delete;
    impl Resolvable for Delete {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Terminal{arg:"D".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"E".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"L".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"E".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"T".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"E".to_string().as_bytes()[0]}};
        let e: StackEntry = StackEntry{rule: Rules::Delete, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Delete as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Passthrough;
    impl Resolvable for Passthrough {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Terminal{arg:"P".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"A".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"S".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"S".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"T".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"H".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"R".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"O".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"U".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"G".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"H".to_string().as_bytes()[0]}};
        let e: StackEntry = StackEntry{rule: Rules::Passthrough, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Passthrough as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    

    #[derive(Copy, Clone)]
    pub struct Collect;
    impl Resolvable for Collect {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
        let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Terminal{arg:"C".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"O".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"L".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"L".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"E".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"C".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"T".to_string().as_bytes()[0]}};
        let e: StackEntry = StackEntry{rule: Rules::Collect, start_position: position, end_position: 0};
        let e_position = stack.push(e);
        let hook = cache_struct_wrapper(stack, cache, rule, Rules::Collect as u32, position, source);
        if hook.0 == false || position == hook.1 { 
            //println!("Popping");
            //println!("Should be popping {}", e_position);
            loop {
                let stack_length = stack.len();
                if stack_length > e_position{
                    stack.pop();
                }
                else{
                    break;
                }
            }
        }
        else{
            stack.update(e_position, hook.1);
        }
        return hook;
        }
    }
    
