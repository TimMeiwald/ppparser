mod and_predicate;
mod not_predicate;
mod one_or_more;
mod optional;
mod ordered_choice;
mod sequence;
mod subexpression;
mod terminal;
mod var_name;
mod zero_or_more;
mod utils;

pub use crate::and_predicate::AndPredicate;
pub use crate::not_predicate::NotPredicate;
pub use crate::one_or_more::OneOrMore;
pub use crate::optional::Optional;
pub use crate::ordered_choice::OrderedChoice;
pub use crate::sequence::Sequence;
pub use crate::terminal::Resolvable;
pub use crate::terminal::Terminal;
pub use crate::var_name::VarName;
pub use crate::zero_or_more::ZeroOrMore;
pub use crate::utils::read_input;








/* The following is not an integration test. It's an example of the code that need's to be generated for each Rule so that it works with the rest of the codebase. */
#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Copy, Clone)] // Needed because I copy around position which as a u32 is likely faster than passing a reference, Need's profiling.
    struct Example; // Top level functions don't require T since they're defined via the primitives used in a given function.
    #[derive(Copy, Clone)]
    struct Example2;

    impl Resolvable for Example {
        fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
            return example1(position, source); // Define which function to run using impl Resolvable
        }
    }

    impl Resolvable for Example2 {
        fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
            return example2(position, source);
        }
    }

    fn example1(position: u32, source: &str) -> (bool, u32) {
        let t1 = Example2;
        let t2 = Terminal {
            arg: "e".to_string().as_bytes()[0],
        };
        let t3 = Sequence {
            arg_lhs: t1,
            arg_rhs: t2,
        };
        let s = t3.resolve(position, source); // Each Top Level Rule can still call other Top Level Rules as well as primitives.
        return s;
    }

    fn example2(position: u32, source: &str) -> (bool, u32) {
        let t = Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let s = t.resolve(position, source);
        return s;
    }

    #[test]
    fn test_example_true() {
        let source = "Hello World";
        let position: u32 = 0;
        let s = example1(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 2);
    }

    #[test]
    fn test_example_false() {
        let source = "Hfllo World";
        let position: u32 = 0;
        let s = example1(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }

    #[test]
    fn test_example_false2() {
        let source = "fello World";
        let position: u32 = 0;
        let s = example1(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }
}
