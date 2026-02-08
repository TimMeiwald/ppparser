use std::collections::HashSet;

// For non-context sensitive grammars this is not needed at all.
// This can be used in the hooked calls to allow you to carry some specific state that is needed for your parser.
// For example C would need to track typedefs to parse correctly
// Non context sensitive grammars might still decide to use it to track e.g statistical information etc.
// Point is this is passed through the generated layers to hooked calls allowing you to do whatever you want.
// For now this is a struct, it may become a trait in future to allow you to easily swap out implementations for e,g testing.
// If you do not need this at all you can just leave it. The compiler will optimise it out anyway as it does nothing by default
// and isn't used anywhere by default either.
pub struct UserState {
    typedef_names: HashSet<String>,
}
impl Default for UserState {
    fn default() -> Self {
        Self::new()
    }
}

impl UserState {
    pub fn new() -> Self {
        UserState {
            typedef_names: HashSet::new(),
        }
    }
}
