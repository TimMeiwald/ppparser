use crate::cache::Cache;
use crate::Resolvable;
use crate::output_stack::Stack;

//
#[derive(Copy, Clone)]
pub struct _Optional<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for _Optional<T> {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) {
        return optional(stack, cache, position, source, self.arg);
    }
}

fn optional<T: Resolvable>(stack: &mut Stack, cache: &mut Cache, position: u32, source: &str, args: T) -> (bool, u32) {
    /* True if matches, False if not. Increments position on a match */

    // Fn(&u8), u8
    // Fn(&Fn), Fn
    let temp_position = position;
    let (bool, position) = args.resolve(stack, cache, position, source);

    if bool == true {
        return (true, position);
    } else {
        let position = temp_position;
        return (true, position);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_Terminal;
    use crate::cache::Cache;
    #[test]
    fn test_optional_no_increment() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = _Terminal {
            arg: "f".to_string().as_bytes()[0],
        };
        let t2 = _Optional { arg: t };
        let mut cache = Cache::new(100, 1);
        let mut stack = Stack::new(100,100);

        let s = t2.resolve(&mut stack, &mut cache, position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 0);
    }

    #[test]
    fn test_optional_increment() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = _Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t2 = _Optional { arg: t };
        let mut cache = Cache::new(100, 1);
        let mut stack = Stack::new(100,100);

        let s = t2.resolve(&mut stack, &mut cache, position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 1);
    }

    #[test]
    fn test_optional_nested() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = _Terminal {
            arg: "f".to_string().as_bytes()[0],
        };
        let t2 = _Optional { arg: t };
        let t3 = _Optional { arg: t2 };
        let mut cache = Cache::new(100, 1);
        let mut stack = Stack::new(100,100);

        let s = t3.resolve(&mut stack, &mut cache, position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 0);
    }
}
