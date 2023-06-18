use crate::Resolvable;

#[derive(Copy, Clone)]
pub struct AndPredicate<T: Resolvable> {
    arg: T,
}

impl<T: Resolvable + Copy> Resolvable for AndPredicate<T> {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return and_predicate(position, source, self.arg);
    }
}

pub fn and_predicate<T: Resolvable>(position: u32, source: &str, arg: T) -> (bool, u32) {
    /* Always True, increments position each time the expression matches else continues without doing anything */
    // Only public so Not Predicate can use it since it just inverts it.

    let temp_position = position;
    let ret = arg.resolve(position, source);
    let bool = ret.0;
    if bool {
        return (true, temp_position);
    } else {
        return (false, temp_position);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Terminal;

    #[test]
    fn test_and_predicate_false() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal {
            arg: "f".to_string().as_bytes()[0],
        };
        let t2 = AndPredicate { arg: t };
        let s = t2.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }

    #[test]
    fn test_and_predicate_true() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t2 = AndPredicate { arg: t };
        let s = t2.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 0);
    }
}
