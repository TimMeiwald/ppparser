pub mod source;
mod terminal;
mod sequence;
mod one_or_more;
mod var_name;
mod ordered_choice;
mod zero_or_more;
// Parser must accept a reference to a string. It must then return whether it's validly parsed or not.
// It should have primitives of _Terminal, _OrderedChoice, _Sequence, _OneOrMore, _ZeroOrMore, _NotPredicate, _AndPredicate, _Subexpression, _VarName, _Optional and they should be composable
// Stretch goal 1 -> also return a stack containing a list of references to the string and the start and end positions of each valid token
// Stretch goal 2 -> Use a cache to implement packrat parsing
// Stretch goal 3 -> Use cache to implement direct left recursion
// Stretch goal 4 -> Use cache to also implement indirect left recursion
