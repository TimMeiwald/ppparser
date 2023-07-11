use crate::cache::Cache;
use crate::Resolvable;

//
#[derive(Copy, Clone)]
pub struct _Sequence<T: Resolvable, U: Resolvable> {
    pub arg_lhs: T,
    pub arg_rhs: U,
}

impl<T: Resolvable + Copy, U: Resolvable + Copy> Resolvable for _Sequence<T, U> {
    fn resolve(&self, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) {
        return sequence(cache, position, source, self.arg_lhs, self.arg_rhs);
    }
}

fn sequence<T: Resolvable, U: Resolvable>(
    cache: &mut Cache,
    position: u32,
    source: &str,
    arg_lhs: T,
    arg_rhs: U,
) -> (bool, u32) {
    /* True if all expressions match, then updates position, else false, no positional update */

    let tmp_pos = position;

    let (lhs_bool, position) = arg_lhs.resolve(cache, position, source);

    if lhs_bool {
        let (rhs_bool, position) = arg_rhs.resolve(cache, position, source);
        if rhs_bool {
            return (true, position);
        } else {
            let position = tmp_pos;
            return (false, position);
        };
    } else {
        let position = tmp_pos;
        return (false, position);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_OrderedChoice;
    use crate::_Terminal;
    use crate::cache::Cache;
    #[test]
    fn test_sequence_true() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = _Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t2 = _Terminal {
            arg: "e".to_string().as_bytes()[0],
        };
        let t3 = _Sequence {
            arg_lhs: t,
            arg_rhs: t2,
        };
        let mut cache = Cache::new(100, 1);

        let s = t3.resolve(&mut cache, position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 2);
    }

    #[test]
    fn test_sequence_false() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = _Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t2 = _Terminal {
            arg: "f".to_string().as_bytes()[0],
        };
        let t3 = _Sequence {
            arg_lhs: t,
            arg_rhs: t2,
        };
        let mut cache = Cache::new(100, 1);

        let s = t3.resolve(&mut cache, position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }

    #[test]
    fn test_sequence_nested() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = _Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t2 = _Terminal {
            arg: "e".to_string().as_bytes()[0],
        };
        let t3 = _Terminal {
            arg: "l".to_string().as_bytes()[0],
        };
        let t4 = _Terminal {
            arg: "l".to_string().as_bytes()[0],
        };
        let t5 = _Sequence {
            arg_lhs: t,
            arg_rhs: t2,
        };
        let t6 = _Sequence {
            arg_lhs: t3,
            arg_rhs: t4,
        };
        let t7 = _Sequence {
            arg_lhs: t5,
            arg_rhs: t6,
        };
        let mut cache = Cache::new(100, 1);

        let s = t7.resolve(&mut cache, position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 4);
    }

    #[test]
    fn test_mixed_sequence_ordered_choice() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = _Terminal {
            arg: "f".to_string().as_bytes()[0],
        };
        let t2 = _Terminal {
            arg: "i".to_string().as_bytes()[0],
        };
        let t3 = _Terminal {
            arg: "s".to_string().as_bytes()[0],
        };
        let t4 = _Terminal {
            arg: "s".to_string().as_bytes()[0],
        };
        let t5 = _Sequence {
            arg_lhs: t,
            arg_rhs: t2,
        };
        let t6 = _Sequence {
            arg_lhs: t3,
            arg_rhs: t4,
        };
        let t7 = _Sequence {
            arg_lhs: t5,
            arg_rhs: t6,
        };

        let r = _Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let r2 = _Terminal {
            arg: "e".to_string().as_bytes()[0],
        };
        let r3 = _Terminal {
            arg: "l".to_string().as_bytes()[0],
        };
        let r4 = _Terminal {
            arg: "l".to_string().as_bytes()[0],
        };
        let r5 = _Sequence {
            arg_lhs: r,
            arg_rhs: r2,
        };
        let r6 = _Sequence {
            arg_lhs: r3,
            arg_rhs: r4,
        };
        let r7 = _Sequence {
            arg_lhs: r5,
            arg_rhs: r6,
        };
        let t8 = _OrderedChoice {
            arg_lhs: t7,
            arg_rhs: r7,
        };
        let mut cache = Cache::new(100, 1);

        let s = t8.resolve(&mut cache, position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 4);
    }
}
