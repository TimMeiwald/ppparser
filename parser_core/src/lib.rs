mod and_predicate;
mod context;
mod not_predicate;
mod one_or_more;
mod optional;
mod ordered_choice;
mod rules;
mod sequence;
mod source;
mod subexpression;
mod terminal;
mod var_name;
mod zero_or_more;

pub use and_predicate::_and_predicate;
pub use context::Context;
pub use not_predicate::_not_predicate;
pub use one_or_more::_one_or_more;
pub use optional::_optional;
pub use ordered_choice::_ordered_choice;
pub use rules::Rules;
pub use sequence::_sequence;
pub use source::Source;
pub use subexpression::_subexpression;
pub use terminal::_terminal;
pub use var_name::_var_name;
pub use zero_or_more::_zero_or_more;

// Parser must accept a reference to a string. It must then return whether it's validly parsed or not.
// It should have primitives of _Terminal, _OrderedChoice, _Sequence, _OneOrMore, _ZeroOrMore, _NotPredicate, _AndPredicate, _Subexpression, _VarName, _Optional and they should be composable
// Stretch goal 1 -> also return a stack containing a list of references to the string and the start and end positions of each valid token
// Stretch goal 2 -> Use a cache to implement packrat parsing
// Stretch goal 3 -> Use cache to implement direct left recursion
// Stretch goal 4 -> Use cache to also implement indirect left recursion
