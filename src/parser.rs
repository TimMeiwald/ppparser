pub use crate::and_predicate::_AndPredicate;
pub use crate::not_predicate::_NotPredicate;
pub use crate::one_or_more::_OneOrMore;
pub use crate::optional::_Optional;
pub use crate::ordered_choice::_OrderedChoice;
pub use crate::sequence::_Sequence;
pub use crate::subexpression::_SubExpression;
pub use crate::terminal::Resolvable;
pub use crate::terminal::_Terminal;
pub use crate::var_name::_VarName;
pub use crate::zero_or_more::_ZeroOrMore;
use crate::cache::Cache;
use crate::cache::cache_wrapper;


#[derive(Copy, Clone)]
pub struct AlphabetUpper;
impl Resolvable for AlphabetUpper {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 0;
    let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"A".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"B".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"C".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"D".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"E".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"F".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"G".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"H".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"I".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"J".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"K".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"L".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"M".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"N".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"O".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"P".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"Q".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"R".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"S".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"T".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"U".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"V".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"W".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"X".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"Y".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"Z".to_string().as_bytes()[0]}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct AlphabetLower;
impl Resolvable for AlphabetLower {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 1;
    let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"a".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"b".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"c".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"d".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"e".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"f".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"g".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"h".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"i".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"j".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"k".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"l".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"m".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"n".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"o".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"p".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"q".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"r".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"s".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"t".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"u".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"v".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"w".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"x".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"y".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"z".to_string().as_bytes()[0]}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Num;
impl Resolvable for Num {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 2;
    let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"0".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"1".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"2".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"3".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"4".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"5".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"6".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"7".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"8".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"9".to_string().as_bytes()[0]}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Spaces;
impl Resolvable for Spaces {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 3;
    let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"\n".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"\t".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"\r".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:" ".to_string().as_bytes()[0]}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Specials;
impl Resolvable for Specials {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 4;
    let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"+".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"*".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"-".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"&".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"!".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"?".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"<".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:">".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:'"'.to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"(".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:")".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"_".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:",".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"/".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:";".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"=".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:'\\'.to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"#".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:":".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"|".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:".".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"{".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"}".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"[".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"]".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"%".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"'".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"^".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"~".to_string().as_bytes()[0]}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Ascii;
impl Resolvable for Ascii {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 5;
    let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:AlphabetLower}, arg_rhs:_VarName{arg:AlphabetUpper}}, arg_rhs:_VarName{arg:Num}}, arg_rhs:_VarName{arg:Spaces}}, arg_rhs:_VarName{arg:Specials}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Apostrophe;
impl Resolvable for Apostrophe {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 6;
    let rule = _Terminal{arg:'"'.to_string().as_bytes()[0]};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct LeftAngleBracket;
impl Resolvable for LeftAngleBracket {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 7;
    let rule = _Terminal{arg:"<".to_string().as_bytes()[0]};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct RightAngleBracket;
impl Resolvable for RightAngleBracket {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 8;
    let rule = _Terminal{arg:">".to_string().as_bytes()[0]};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct LeftBracket;
impl Resolvable for LeftBracket {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 9;
    let rule = _Terminal{arg:"(".to_string().as_bytes()[0]};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct RightBracket;
impl Resolvable for RightBracket {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 10;
    let rule = _Terminal{arg:")".to_string().as_bytes()[0]};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Assignment;
impl Resolvable for Assignment {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 11;
    let rule = _Terminal{arg:"=".to_string().as_bytes()[0]};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct EndRule;
impl Resolvable for EndRule {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 12;
    let rule = _Terminal{arg:";".to_string().as_bytes()[0]};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Ampersand;
impl Resolvable for Ampersand {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 13;
    let rule = _Terminal{arg:"&".to_string().as_bytes()[0]};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct ExclamationMark;
impl Resolvable for ExclamationMark {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 14;
    let rule = _Terminal{arg:"!".to_string().as_bytes()[0]};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Plus;
impl Resolvable for Plus {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 15;
    let rule = _Terminal{arg:"+".to_string().as_bytes()[0]};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Star;
impl Resolvable for Star {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 16;
    let rule = _Terminal{arg:"*".to_string().as_bytes()[0]};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct QuestionMark;
impl Resolvable for QuestionMark {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 17;
    let rule = _Terminal{arg:"?".to_string().as_bytes()[0]};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Comma;
impl Resolvable for Comma {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 18;
    let rule = _Terminal{arg:",".to_string().as_bytes()[0]};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Backslash;
impl Resolvable for Backslash {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 19;
    let rule = _Terminal{arg:"/".to_string().as_bytes()[0]};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct VarName;
impl Resolvable for VarName {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 20;
    let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:LeftAngleBracket}, arg_rhs:_SubExpression{arg:_OrderedChoice{arg_lhs:_VarName{arg:AlphabetLower}, arg_rhs:_VarName{arg:AlphabetUpper}}}}, arg_rhs:_ZeroOrMore{arg: _SubExpression{arg:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:AlphabetLower}, arg_rhs:_VarName{arg:AlphabetUpper}}, arg_rhs:_Terminal{arg:"_".to_string().as_bytes()[0]}}}}}, arg_rhs:_VarName{arg:RightAngleBracket}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Subexpression;
impl Resolvable for Subexpression {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 21;
    let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:LeftBracket}, arg_rhs:_VarName{arg:Rhs}}, arg_rhs:_VarName{arg:RightBracket}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Epsilon;
impl Resolvable for Epsilon {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 22;
    let rule = _Sequence{arg_lhs:_VarName{arg:Apostrophe}, arg_rhs:_VarName{arg:Apostrophe}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Lterminal;
impl Resolvable for Lterminal {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 23;
    let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_SubExpression{arg:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Apostrophe}, arg_rhs:_VarName{arg:Ascii}}, arg_rhs:_VarName{arg:Apostrophe}}}, arg_rhs:_SubExpression{arg:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Apostrophe}, arg_rhs:_Terminal{arg:'\\'.to_string().as_bytes()[0]}}, arg_rhs:_SubExpression{arg:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"n".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"r".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"t".to_string().as_bytes()[0]}}}}, arg_rhs:_VarName{arg:Apostrophe}}}}, arg_rhs:_VarName{arg:Epsilon}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Nucleus;
impl Resolvable for Nucleus {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 24;
    let rule = _Sequence{arg_lhs:_SubExpression{arg:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:Subexpression}, arg_rhs:_VarName{arg:Lterminal}}, arg_rhs:_VarName{arg:VarName}}}, arg_rhs:_VarName{arg:Whitespace}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Atom;
impl Resolvable for Atom {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 25;
    let rule = _Sequence{arg_lhs:_SubExpression{arg:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:AndPredicate}, arg_rhs:_VarName{arg:NotPredicate}}, arg_rhs:_VarName{arg:OneOrMore}}, arg_rhs:_VarName{arg:ZeroOrMore}}, arg_rhs:_VarName{arg:Optional}}, arg_rhs:_VarName{arg:Nucleus}}}, arg_rhs:_VarName{arg:Whitespace}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct AndPredicate;
impl Resolvable for AndPredicate {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 26;
    let rule = _Sequence{arg_lhs:_VarName{arg:Ampersand}, arg_rhs:_VarName{arg:Nucleus}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct NotPredicate;
impl Resolvable for NotPredicate {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 27;
    let rule = _Sequence{arg_lhs:_VarName{arg:ExclamationMark}, arg_rhs:_VarName{arg:Nucleus}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Sequence;
impl Resolvable for Sequence {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 28;
    let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Atom}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Comma}}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Atom}}, arg_rhs:_ZeroOrMore{arg: _SubExpression{arg:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Comma}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Atom}}}}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct OrderedChoice;
impl Resolvable for OrderedChoice {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 29;
    let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Atom}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Backslash}}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Atom}}, arg_rhs:_ZeroOrMore{arg: _SubExpression{arg:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Backslash}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Atom}}}}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct OneOrMore;
impl Resolvable for OneOrMore {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 30;
    let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Nucleus}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Plus}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct ZeroOrMore;
impl Resolvable for ZeroOrMore {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 31;
    let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Nucleus}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Star}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Optional;
impl Resolvable for Optional {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 32;
    let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Nucleus}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:QuestionMark}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Whitespace;
impl Resolvable for Whitespace {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 33;
    let rule = _ZeroOrMore{arg: _SubExpression{arg:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:" ".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"\n".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"\r".to_string().as_bytes()[0]}}}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Rhs;
impl Resolvable for Rhs {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 34;
    let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:Sequence}, arg_rhs:_VarName{arg:OrderedChoice}}, arg_rhs:_VarName{arg:Atom}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Lhs;
impl Resolvable for Lhs {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 35;
    let rule = _Sequence{arg_lhs:_VarName{arg:VarName}, arg_rhs:_Optional{arg: _SubExpression{arg:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Whitespace}, arg_rhs:_VarName{arg:SemanticInstructions}}, arg_rhs:_VarName{arg:Whitespace}}}}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Rule;
impl Resolvable for Rule {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 36;
    let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Lhs}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Assignment}}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Rhs}}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:EndRule}}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_ZeroOrMore{arg: _VarName{arg:Comment}}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Grammar;
impl Resolvable for Grammar {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 37;
    let rule = _Sequence{arg_lhs:_OneOrMore{arg: _VarName{arg:Rule}}, arg_rhs:_VarName{arg:Whitespace}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Comment;
impl Resolvable for Comment {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 38;
    let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Whitespace}, arg_rhs:_Terminal{arg:"#".to_string().as_bytes()[0]}}, arg_rhs:_ZeroOrMore{arg: _SubExpression{arg:_Sequence{arg_lhs:_NotPredicate{arg: _Terminal{arg:"#".to_string().as_bytes()[0]}}, arg_rhs:_VarName{arg:Ascii}}}}}, arg_rhs:_Terminal{arg:"#".to_string().as_bytes()[0]}}, arg_rhs:_VarName{arg:Whitespace}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct SemanticInstructions;
impl Resolvable for SemanticInstructions {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 39;
    let rule = _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:Delete}, arg_rhs:_VarName{arg:Passthrough}}, arg_rhs:_VarName{arg:Collect}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Delete;
impl Resolvable for Delete {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 40;
    let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Terminal{arg:"D".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"E".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"L".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"E".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"T".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"E".to_string().as_bytes()[0]}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Passthrough;
impl Resolvable for Passthrough {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 41;
    let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Terminal{arg:"P".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"A".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"S".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"S".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"T".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"H".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"R".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"O".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"U".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"G".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"H".to_string().as_bytes()[0]}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}


#[derive(Copy, Clone)]
pub struct Collect;
impl Resolvable for Collect {
fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) { 
    let arg_key = 42;
    let rule = _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Terminal{arg:"C".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"O".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"L".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"L".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"E".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"C".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"T".to_string().as_bytes()[0]}};
    let hook = cache_wrapper(cache, rule, arg_key, position, source);
    return hook;
    }
}

