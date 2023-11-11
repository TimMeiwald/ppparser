// The purpose of this binary is purely to test whether left recursion works without contaminating the actual parsers crate since it's 
// to actually part of it. This should be stripped out as a dev dependency ideally.

// Making it a binary so I can run it from CI. 

mod expr;