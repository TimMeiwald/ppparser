pub mod source;
pub mod terminal;
pub mod sequence;
pub mod not_predicate;
pub mod one_or_more;
pub mod subexpression;
pub mod and_predicate;
pub mod optional;
pub mod var_name;
pub mod ordered_choice;
pub mod zero_or_more;
// Parser must accept a reference to a string. It must then return whether it's validly parsed or not.
// It should have primitives of _Terminal, _OrderedChoice, _Sequence, _OneOrMore, _ZeroOrMore, _NotPredicate, _AndPredicate, _Subexpression, _VarName, _Optional and they should be composable
// Stretch goal 1 -> also return a stack containing a list of references to the string and the start and end positions of each valid token
// Stretch goal 2 -> Use a cache to implement packrat parsing
// Stretch goal 3 -> Use cache to implement direct left recursion
// Stretch goal 4 -> Use cache to also implement indirect left recursion
