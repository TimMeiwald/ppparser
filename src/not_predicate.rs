use crate::and_predicate;
use crate::Resolvable; //Solely to just invert and predicate

#[derive(Copy, Clone)]
pub struct NotPredicate<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for NotPredicate<T> {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return not_predicate(position, source, self.arg);
    }
}

fn not_predicate<T: Resolvable>(position: u32, source: &str, arg: T) -> (bool, u32) {
    /* Always True, increments position each time the expression matches else continues without doing anything */

    let (bool, position) = and_predicate::and_predicate(position, source, arg);
    return (!bool, position);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Terminal;

    #[test]
    fn test_not_predicate_true() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal {
            arg: "f".to_string().as_bytes()[0],
        };
        let t2 = NotPredicate { arg: t };
        let s = t2.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 0);
    }

    #[test]
    fn test_not_predicate_false() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t2 = NotPredicate { arg: t };
        let s = t2.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }
}