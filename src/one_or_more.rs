use crate::Resolvable;

#[derive(Copy, Clone)]
pub struct OneOrMore<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for OneOrMore<T> {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return one_or_more(position, source, self.arg);
    }
}


fn one_or_more<T: Resolvable>(position: u32, source: &str, arg: T) -> (bool, u32) {
    /* Always True, increments position each time the expression matches else continues without doing anything */

    let mut temp_position = position;
    let (mut bool, mut position) = arg.resolve(position, source);
    if bool {
        temp_position = position;
    } else {
        position = temp_position;
        return (false, position);
    }
    loop {
        let ret = arg.resolve(position, source);
        bool = ret.0;
        position = ret.1;
        if bool {
            temp_position = position;
            continue;
        } else {
            position = temp_position;
            break;
        }
    }
    return (true, position);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Terminal;

    #[test]
    fn test_one_or_more_1() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t3 = OneOrMore { arg: t };
        let s = t3.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 1);
    }

    #[test]
    fn test_one_or_more_10() {
        let source = "HHHHHHHHHHello World";
        let position: u32 = 0;
        let t = Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t3 = OneOrMore { arg: t };
        let s = t3.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 10);
    }

    #[test]
    fn test_one_or_more_0() {
        let source = "fello World";
        let position: u32 = 0;
        let t = Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t3 = OneOrMore { arg: t };
        let s = t3.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }
}
