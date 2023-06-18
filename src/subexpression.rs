use crate::Resolvable;

#[derive(Copy, Clone)]
pub struct SubExpression<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for SubExpression<T> {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return subexpression(position, source, self.arg);
    }
}

fn subexpression<T: Resolvable>(position: u32, source: &str, arg: T) -> (bool, u32) {
    /* Subexpression is any expression inside a pair of () brackets
    SUBEXPR essentially does nothing but allows for order of precedent
    more importantly order of precedence is very restricted because it made my life hard
    (mostly because I can't find a good definition of what order of precedence is in PEG) so use SUBEXPR
    to make more complicated rules */

    let temp_position = position;
    let (bool, position) = arg.resolve(position, source);

    if bool {
        return (true, position);
    } else {
        return (false, temp_position);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Terminal;
    #[test]
    fn test_subexpression_true() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal {
            arg: "f".to_string().as_bytes()[0],
        };
        let t2 = SubExpression { arg: t };
        let s = t2.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }

    #[test]
    fn test_subexpression_false() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t2 = SubExpression { arg: t };
        let s = t2.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 1);
    }
}
