use crate::Resolvable;

fn optional<T: Resolvable>(position: u32, source: &str, args: T) -> (bool, u32) {
    /* True if matches, False if not. Increments position on a match */

    // Fn(&u8), u8
    // Fn(&Fn), Fn
    let temp_position = position;
    let (bool, position) = args.resolve(position, source);

    if bool == true {
        return (true, position);
    } else {
        let position = temp_position;
        return (true, position);
    }
}

#[derive(Copy, Clone)]
pub struct Optional<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for Optional<T> {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return optional(position, source, self.arg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Terminal;
    #[test]
    fn test_optional_no_increment() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal {
            arg: "f".to_string().as_bytes()[0],
        };
        let t2 = Optional { arg: t };
        let s = t2.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 0);
    }

    #[test]
    fn test_optional_increment() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t2 = Optional { arg: t };
        let s = t2.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 1);
    }

    #[test]
    fn test_optional_nested() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal {
            arg: "f".to_string().as_bytes()[0],
        };
        let t2 = Optional { arg: t };
        let t3 = Optional { arg: t2 };
        let s = t3.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 0);
    }
}
