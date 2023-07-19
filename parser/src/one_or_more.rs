use crate::cache::Cache;
use crate::Resolvable;
use crate::output_stack::Stack;

//
#[derive(Copy, Clone)]
pub struct _OneOrMore<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for _OneOrMore<T> {
    fn resolve(&self,stack: &mut Stack, cache: &mut Cache, position: i32, source: &str) -> (bool, i32) {
        return one_or_more(stack, cache, position, source, self.arg);
    }
}

fn one_or_more<T: Resolvable>(
    stack: &mut Stack, 
    cache: &mut Cache,
    position: i32,
    source: &str,
    arg: T,
) -> (bool, i32) {
    /* Always True, increments position each time the expression matches else continues without doing anything */

    let mut temp_position = position;
    let (mut bool, mut position) = arg.resolve(stack, cache, position, source);
    if bool {
        temp_position = position;
    } else {
        position = temp_position;
        return (false, position);
    }
    loop {
        let ret = arg.resolve(stack, cache, position, source);
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
    use crate::_Terminal;
    use crate::cache::Cache;
    #[test]
    fn test_one_or_more_1() {
        let source = "Hello World";
        let position: i32 = 0;
        let t = _Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t3 = _OneOrMore { arg: t };
        let mut cache = Cache::new(100, 1);
        let mut stack = Stack::new(100,100);

        let s = t3.resolve(&mut stack, &mut cache, position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 1);
    }

    #[test]
    fn test_one_or_more_10() {
        let source = "HHHHHHHHHHello World";
        let position: i32 = 0;
        let t = _Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t3 = _OneOrMore { arg: t };
        let mut cache = Cache::new(100, 1);
        let mut stack = Stack::new(100,100);
        let s = t3.resolve(&mut stack, &mut cache, position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 10);
    }

    #[test]
    fn test_one_or_more_0() {
        let source = "fello World";
        let position: i32 = 0;
        let t = _Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t3 = _OneOrMore { arg: t };
        let mut cache = Cache::new(100, 1);
        let mut stack = Stack::new(100,100);
        let s = t3.resolve(&mut stack, &mut cache, position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }
}
