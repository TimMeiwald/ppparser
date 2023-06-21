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


#[derive(Copy, Clone)]
pub struct AlphabetUpper;
impl Resolvable for AlphabetUpper {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: AlphabetUpper"};
    let hook = alphabet_upper(position, source); 
    println!{"Struct: AlphabetUpper, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn alphabet_upper(position: u32, source: &str) -> (bool, u32) {
return _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"A".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"B".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"C".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"D".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"E".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"F".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"G".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"H".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"I".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"J".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"K".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"L".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"M".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"N".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"O".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"P".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"Q".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"R".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"S".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"T".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"U".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"V".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"W".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"X".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"Y".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"Z".to_string().as_bytes()[0]}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct AlphabetLower;
impl Resolvable for AlphabetLower {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: AlphabetLower"};
    let hook = alphabet_lower(position, source); 
    println!{"Struct: AlphabetLower, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn alphabet_lower(position: u32, source: &str) -> (bool, u32) {
return _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"a".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"b".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"c".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"d".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"e".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"f".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"g".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"h".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"i".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"j".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"k".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"l".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"m".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"n".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"o".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"p".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"q".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"r".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"s".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"t".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"u".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"v".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"w".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"x".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"y".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"z".to_string().as_bytes()[0]}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Num;
impl Resolvable for Num {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Num"};
    let hook = num(position, source); 
    println!{"Struct: Num, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn num(position: u32, source: &str) -> (bool, u32) {
return _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"0".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"1".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"2".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"3".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"4".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"5".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"6".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"7".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"8".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"9".to_string().as_bytes()[0]}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Spaces;
impl Resolvable for Spaces {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Spaces"};
    let hook = spaces(position, source); 
    println!{"Struct: Spaces, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn spaces(position: u32, source: &str) -> (bool, u32) {
return _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"\n".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"\t".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"\r".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:" ".to_string().as_bytes()[0]}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Specials;
impl Resolvable for Specials {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Specials"};
    let hook = specials(position, source); 
    println!{"Struct: Specials, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn specials(position: u32, source: &str) -> (bool, u32) {
return _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"+".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"*".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"-".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"&".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"!".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"?".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"<".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:">".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:'"'.to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"(".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:")".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"_".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:",".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"/".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:";".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"=".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:'\\'.to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"#".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:":".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"|".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:".".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"{".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"}".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"[".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"]".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"%".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"'".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"^".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"~".to_string().as_bytes()[0]}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Ascii;
impl Resolvable for Ascii {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Ascii"};
    let hook = ascii(position, source); 
    println!{"Struct: Ascii, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn ascii(position: u32, source: &str) -> (bool, u32) {
return _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:AlphabetLower}, arg_rhs:_VarName{arg:AlphabetUpper}}, arg_rhs:_VarName{arg:Num}}, arg_rhs:_VarName{arg:Spaces}}, arg_rhs:_VarName{arg:Specials}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Apostrophe;
impl Resolvable for Apostrophe {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Apostrophe"};
    let hook = apostrophe(position, source); 
    println!{"Struct: Apostrophe, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn apostrophe(position: u32, source: &str) -> (bool, u32) {
return _Terminal{arg:'"'.to_string().as_bytes()[0]}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct LeftAngleBracket;
impl Resolvable for LeftAngleBracket {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: LeftAngleBracket"};
    let hook = left_angle_bracket(position, source); 
    println!{"Struct: LeftAngleBracket, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn left_angle_bracket(position: u32, source: &str) -> (bool, u32) {
return _Terminal{arg:"<".to_string().as_bytes()[0]}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct RightAngleBracket;
impl Resolvable for RightAngleBracket {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: RightAngleBracket"};
    let hook = right_angle_bracket(position, source); 
    println!{"Struct: RightAngleBracket, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn right_angle_bracket(position: u32, source: &str) -> (bool, u32) {
return _Terminal{arg:">".to_string().as_bytes()[0]}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct LeftBracket;
impl Resolvable for LeftBracket {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: LeftBracket"};
    let hook = left_bracket(position, source); 
    println!{"Struct: LeftBracket, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn left_bracket(position: u32, source: &str) -> (bool, u32) {
return _Terminal{arg:"(".to_string().as_bytes()[0]}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct RightBracket;
impl Resolvable for RightBracket {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: RightBracket"};
    let hook = right_bracket(position, source); 
    println!{"Struct: RightBracket, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn right_bracket(position: u32, source: &str) -> (bool, u32) {
return _Terminal{arg:")".to_string().as_bytes()[0]}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Assignment;
impl Resolvable for Assignment {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Assignment"};
    let hook = assignment(position, source); 
    println!{"Struct: Assignment, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn assignment(position: u32, source: &str) -> (bool, u32) {
return _Terminal{arg:"=".to_string().as_bytes()[0]}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct EndRule;
impl Resolvable for EndRule {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: EndRule"};
    let hook = end_rule(position, source); 
    println!{"Struct: EndRule, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn end_rule(position: u32, source: &str) -> (bool, u32) {
return _Terminal{arg:";".to_string().as_bytes()[0]}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Ampersand;
impl Resolvable for Ampersand {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Ampersand"};
    let hook = ampersand(position, source); 
    println!{"Struct: Ampersand, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn ampersand(position: u32, source: &str) -> (bool, u32) {
return _Terminal{arg:"&".to_string().as_bytes()[0]}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct ExclamationMark;
impl Resolvable for ExclamationMark {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: ExclamationMark"};
    let hook = exclamation_mark(position, source); 
    println!{"Struct: ExclamationMark, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn exclamation_mark(position: u32, source: &str) -> (bool, u32) {
return _Terminal{arg:"!".to_string().as_bytes()[0]}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Plus;
impl Resolvable for Plus {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Plus"};
    let hook = plus(position, source); 
    println!{"Struct: Plus, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn plus(position: u32, source: &str) -> (bool, u32) {
return _Terminal{arg:"+".to_string().as_bytes()[0]}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Star;
impl Resolvable for Star {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Star"};
    let hook = star(position, source); 
    println!{"Struct: Star, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn star(position: u32, source: &str) -> (bool, u32) {
return _Terminal{arg:"*".to_string().as_bytes()[0]}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct QuestionMark;
impl Resolvable for QuestionMark {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: QuestionMark"};
    let hook = question_mark(position, source); 
    println!{"Struct: QuestionMark, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn question_mark(position: u32, source: &str) -> (bool, u32) {
return _Terminal{arg:"?".to_string().as_bytes()[0]}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Comma;
impl Resolvable for Comma {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Comma"};
    let hook = comma(position, source); 
    println!{"Struct: Comma, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn comma(position: u32, source: &str) -> (bool, u32) {
return _Terminal{arg:",".to_string().as_bytes()[0]}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Backslash;
impl Resolvable for Backslash {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Backslash"};
    let hook = backslash(position, source); 
    println!{"Struct: Backslash, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn backslash(position: u32, source: &str) -> (bool, u32) {
return _Terminal{arg:"/".to_string().as_bytes()[0]}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct VarName;
impl Resolvable for VarName {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: VarName"};
    let hook = var_name(position, source); 
    println!{"Struct: VarName, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn var_name(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:LeftAngleBracket}, arg_rhs:_SubExpression{arg:_OrderedChoice{arg_lhs:_VarName{arg:AlphabetLower}, arg_rhs:_VarName{arg:AlphabetUpper}}}}, arg_rhs:_ZeroOrMore{arg: _SubExpression{arg:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:AlphabetLower}, arg_rhs:_VarName{arg:AlphabetUpper}}, arg_rhs:_Terminal{arg:"_".to_string().as_bytes()[0]}}}}}, arg_rhs:_VarName{arg:RightAngleBracket}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Subexpression;
impl Resolvable for Subexpression {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Subexpression"};
    let hook = subexpression(position, source); 
    println!{"Struct: Subexpression, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn subexpression(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:LeftBracket}, arg_rhs:_VarName{arg:Rhs}}, arg_rhs:_VarName{arg:RightBracket}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Epsilon;
impl Resolvable for Epsilon {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Epsilon"};
    let hook = epsilon(position, source); 
    println!{"Struct: Epsilon, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn epsilon(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_VarName{arg:Apostrophe}, arg_rhs:_VarName{arg:Apostrophe}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Lterminal;
impl Resolvable for Lterminal {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Lterminal"};
    let hook = lterminal(position, source); 
    println!{"Struct: Lterminal, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn lterminal(position: u32, source: &str) -> (bool, u32) {
return _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_SubExpression{arg:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Apostrophe}, arg_rhs:_VarName{arg:Ascii}}, arg_rhs:_VarName{arg:Apostrophe}}}, arg_rhs:_SubExpression{arg:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Apostrophe}, arg_rhs:_Terminal{arg:'\\'.to_string().as_bytes()[0]}}, arg_rhs:_SubExpression{arg:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_Terminal{arg:"n".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"r".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"t".to_string().as_bytes()[0]}}}}, arg_rhs:_VarName{arg:Apostrophe}}}}, arg_rhs:_VarName{arg:Epsilon}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Nucleus;
impl Resolvable for Nucleus {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Nucleus"};
    let hook = nucleus(position, source); 
    println!{"Struct: Nucleus, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn nucleus(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_SubExpression{arg:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:Subexpression}, arg_rhs:_VarName{arg:Lterminal}}, arg_rhs:_VarName{arg:VarName}}}, arg_rhs:_VarName{arg:Whitespace}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Atom;
impl Resolvable for Atom {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Atom"};
    let hook = atom(position, source); 
    println!{"Struct: Atom, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn atom(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_SubExpression{arg:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:AndPredicate}, arg_rhs:_VarName{arg:NotPredicate}}, arg_rhs:_VarName{arg:OneOrMore}}, arg_rhs:_VarName{arg:ZeroOrMore}}, arg_rhs:_VarName{arg:Optional}}, arg_rhs:_VarName{arg:Nucleus}}}, arg_rhs:_VarName{arg:Whitespace}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct AndPredicate;
impl Resolvable for AndPredicate {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: AndPredicate"};
    let hook = and_predicate(position, source); 
    println!{"Struct: AndPredicate, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn and_predicate(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_VarName{arg:Ampersand}, arg_rhs:_VarName{arg:Nucleus}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct NotPredicate;
impl Resolvable for NotPredicate {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: NotPredicate"};
    let hook = not_predicate(position, source); 
    println!{"Struct: NotPredicate, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn not_predicate(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_VarName{arg:ExclamationMark}, arg_rhs:_VarName{arg:Nucleus}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Sequence;
impl Resolvable for Sequence {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Sequence"};
    let hook = sequence(position, source); 
    println!{"Struct: Sequence, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn sequence(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Atom}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Comma}}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Atom}}, arg_rhs:_ZeroOrMore{arg: _SubExpression{arg:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Comma}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Atom}}}}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct OrderedChoice;
impl Resolvable for OrderedChoice {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: OrderedChoice"};
    let hook = ordered_choice(position, source); 
    println!{"Struct: OrderedChoice, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn ordered_choice(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Atom}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Backslash}}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Atom}}, arg_rhs:_ZeroOrMore{arg: _SubExpression{arg:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Backslash}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Atom}}}}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct OneOrMore;
impl Resolvable for OneOrMore {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: OneOrMore"};
    let hook = one_or_more(position, source); 
    println!{"Struct: OneOrMore, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn one_or_more(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Nucleus}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Plus}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct ZeroOrMore;
impl Resolvable for ZeroOrMore {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: ZeroOrMore"};
    let hook = zero_or_more(position, source); 
    println!{"Struct: ZeroOrMore, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn zero_or_more(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Nucleus}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Star}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Optional;
impl Resolvable for Optional {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Optional"};
    let hook = optional(position, source); 
    println!{"Struct: Optional, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn optional(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Nucleus}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:QuestionMark}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Whitespace;
impl Resolvable for Whitespace {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Whitespace"};
    let hook = whitespace(position, source); 
    println!{"Struct: Whitespace, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn whitespace(position: u32, source: &str) -> (bool, u32) {
return _ZeroOrMore{arg: _SubExpression{arg:_OrderedChoice{arg_lhs:_Terminal{arg:" ".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"\n".to_string().as_bytes()[0]}}}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Rhs;
impl Resolvable for Rhs {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Rhs"};
    let hook = rhs(position, source); 
    println!{"Struct: Rhs, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn rhs(position: u32, source: &str) -> (bool, u32) {
return _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:Sequence}, arg_rhs:_VarName{arg:OrderedChoice}}, arg_rhs:_VarName{arg:Atom}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Lhs;
impl Resolvable for Lhs {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Lhs"};
    let hook = lhs(position, source); 
    println!{"Struct: Lhs, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn lhs(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_VarName{arg:VarName}, arg_rhs:_Optional{arg: _SubExpression{arg:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Whitespace}, arg_rhs:_VarName{arg:SemanticInstructions}}, arg_rhs:_VarName{arg:Whitespace}}}}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Rule;
impl Resolvable for Rule {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Rule"};
    let hook = rule(position, source); 
    println!{"Struct: Rule, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn rule(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Lhs}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Assignment}}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:Rhs}}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_VarName{arg:EndRule}}, arg_rhs:_VarName{arg:Whitespace}}, arg_rhs:_ZeroOrMore{arg: _VarName{arg:Comment}}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Grammar;
impl Resolvable for Grammar {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Grammar"};
    let hook = grammar(position, source); 
    println!{"Struct: Grammar, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn grammar(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_OneOrMore{arg: _VarName{arg:Rule}}, arg_rhs:_VarName{arg:Whitespace}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Comment;
impl Resolvable for Comment {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Comment"};
    let hook = comment(position, source); 
    println!{"Struct: Comment, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn comment(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_VarName{arg:Whitespace}, arg_rhs:_Terminal{arg:"#".to_string().as_bytes()[0]}}, arg_rhs:_ZeroOrMore{arg: _SubExpression{arg:_Sequence{arg_lhs:_NotPredicate{arg: _Terminal{arg:"#".to_string().as_bytes()[0]}}, arg_rhs:_VarName{arg:Ascii}}}}}, arg_rhs:_Terminal{arg:"#".to_string().as_bytes()[0]}}, arg_rhs:_VarName{arg:Whitespace}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct SemanticInstructions;
impl Resolvable for SemanticInstructions {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: SemanticInstructions"};
    let hook = semantic_instructions(position, source); 
    println!{"Struct: SemanticInstructions, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn semantic_instructions(position: u32, source: &str) -> (bool, u32) {
return _OrderedChoice{arg_lhs:_OrderedChoice{arg_lhs:_VarName{arg:Delete}, arg_rhs:_VarName{arg:Passthrough}}, arg_rhs:_VarName{arg:Collect}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Delete;
impl Resolvable for Delete {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Delete"};
    let hook = delete(position, source); 
    println!{"Struct: Delete, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn delete(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Terminal{arg:"D".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"E".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"L".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"E".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"T".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"E".to_string().as_bytes()[0]}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Passthrough;
impl Resolvable for Passthrough {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Passthrough"};
    let hook = passthrough(position, source); 
    println!{"Struct: Passthrough, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn passthrough(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Terminal{arg:"P".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"A".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"S".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"S".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"T".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"H".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"R".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"O".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"U".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"G".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"H".to_string().as_bytes()[0]}}.resolve(position, source);}

#[derive(Copy, Clone)]
pub struct Collect;
impl Resolvable for Collect {
fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
    println!{"Struct: Collect"};
    let hook = collect(position, source); 
    println!{"Struct: Collect, Bool: {:?}, Position: {:?}", hook.0, hook.1}
    return hook;
    }
}

fn collect(position: u32, source: &str) -> (bool, u32) {
return _Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Sequence{arg_lhs:_Terminal{arg:"C".to_string().as_bytes()[0]}, arg_rhs:_Terminal{arg:"O".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"L".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"L".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"E".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"C".to_string().as_bytes()[0]}}, arg_rhs:_Terminal{arg:"T".to_string().as_bytes()[0]}}.resolve(position, source);}
