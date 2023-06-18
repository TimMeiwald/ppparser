use crate::Resolvable;
fn sequence<T: Resolvable, U: Resolvable>(
    position: u32,
    source: &str,
    arg_lhs: T,
    arg_rhs: U,
) -> (bool, u32) {
    //fn c_sequence<T: Copy, U: Copy>(po: &mut ParserObject, lhs: (&dyn Fn(&mut ParserObject, T) -> bool, T), rhs:(&dyn Fn(&mut ParserObject, U) -> bool, U)) -> bool {
    /* True if all expressions match, then updates position, else false, no positional update */

    let tmp_pos = position;

    let (lhs_bool, position) = arg_lhs.resolve(position, source);
    let (rhs_bool, position) = arg_rhs.resolve(position, source);

    if lhs_bool && rhs_bool {
        return (true, position);
    } else {
        let position = tmp_pos;
        return (false, position);
    }
}

#[derive(Copy, Clone)]
pub struct Sequence<T: Resolvable, U: Resolvable> {
    pub arg_lhs: T,
    pub arg_rhs: U,
}

impl<T: Resolvable + Copy, U: Resolvable + Copy> Resolvable for Sequence<T, U> {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return sequence(position, source, self.arg_lhs, self.arg_rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OrderedChoice;
    use crate::Terminal;
    #[test]
    fn test_sequence_true() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t2 = Terminal {
            arg: "e".to_string().as_bytes()[0],
        };
        let t3 = Sequence {
            arg_lhs: t,
            arg_rhs: t2,
        };
        let s = t3.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 2);
    }

    #[test]
    fn test_sequence_false() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t2 = Terminal {
            arg: "f".to_string().as_bytes()[0],
        };
        let t3 = Sequence {
            arg_lhs: t,
            arg_rhs: t2,
        };
        let s = t3.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }

    #[test]
    fn test_sequence_nested() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t2 = Terminal {
            arg: "e".to_string().as_bytes()[0],
        };
        let t3 = Terminal {
            arg: "l".to_string().as_bytes()[0],
        };
        let t4 = Terminal {
            arg: "l".to_string().as_bytes()[0],
        };
        let t5 = Sequence {
            arg_lhs: t,
            arg_rhs: t2,
        };
        let t6 = Sequence {
            arg_lhs: t3,
            arg_rhs: t4,
        };
        let t7 = Sequence {
            arg_lhs: t5,
            arg_rhs: t6,
        };
        let s = t7.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 4);
    }

    #[test]
    fn test_mixed_sequence_ordered_choice() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal {
            arg: "f".to_string().as_bytes()[0],
        };
        let t2 = Terminal {
            arg: "i".to_string().as_bytes()[0],
        };
        let t3 = Terminal {
            arg: "s".to_string().as_bytes()[0],
        };
        let t4 = Terminal {
            arg: "s".to_string().as_bytes()[0],
        };
        let t5 = Sequence {
            arg_lhs: t,
            arg_rhs: t2,
        };
        let t6 = Sequence {
            arg_lhs: t3,
            arg_rhs: t4,
        };
        let t7 = Sequence {
            arg_lhs: t5,
            arg_rhs: t6,
        };

        let r = Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let r2 = Terminal {
            arg: "e".to_string().as_bytes()[0],
        };
        let r3 = Terminal {
            arg: "l".to_string().as_bytes()[0],
        };
        let r4 = Terminal {
            arg: "l".to_string().as_bytes()[0],
        };
        let r5 = Sequence {
            arg_lhs: r,
            arg_rhs: r2,
        };
        let r6 = Sequence {
            arg_lhs: r3,
            arg_rhs: r4,
        };
        let r7 = Sequence {
            arg_lhs: r5,
            arg_rhs: r6,
        };
        let t8 = OrderedChoice {
            arg_lhs: t7,
            arg_rhs: r7,
        };
        let s = t8.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 4);
    }
}
