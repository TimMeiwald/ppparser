use crate::Resolvable;

#[derive(Copy, Clone)]
pub struct _OrderedChoice<T: Resolvable, U: Resolvable> {
    pub arg_lhs: T,
    pub arg_rhs: U,
}

impl<T: Resolvable + Copy, U: Resolvable + Copy> Resolvable for _OrderedChoice<T, U> {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return ordered_choice(position, source, self.arg_lhs, self.arg_rhs);
    }
}

fn ordered_choice<T: Resolvable, U: Resolvable>(
    position: u32,
    source: &str,
    arg_lhs: T,
    arg_rhs: U,
) -> (bool, u32) {
    /* True if one expression matches, then updates position, else false, no positional update */

    let tmp_pos = position;
    let (lhs_bool, position) = arg_lhs.resolve(position, source);
    if lhs_bool {
        return (true, position);
    }
    let position = tmp_pos;

    let (rhs_bool, position) = arg_rhs.resolve(position, source);
    if rhs_bool {
        return (true, position);
    }
    let position = tmp_pos;

    return (false, position);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_Terminal;
    #[test]
    fn test_ordered_choice_true() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = _Terminal {
            arg: "f".to_string().as_bytes()[0],
        };
        let t2 = _Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t3 = _OrderedChoice {
            arg_lhs: t,
            arg_rhs: t2,
        };
        let s = t3.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 1);
    }

    #[test]
    fn test_ordered_choice_false() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = _Terminal {
            arg: "f".to_string().as_bytes()[0],
        };
        let t2 = _Terminal {
            arg: "e".to_string().as_bytes()[0],
        };
        let t3 = _OrderedChoice {
            arg_lhs: t,
            arg_rhs: t2,
        };
        let s = t3.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }

    #[test]
    fn test_ordered_choice_nested() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = _Terminal {
            arg: "f".to_string().as_bytes()[0],
        };
        let t2 = _Terminal {
            arg: "e".to_string().as_bytes()[0],
        };
        let t3 = _Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t4 = _Terminal {
            arg: "n".to_string().as_bytes()[0],
        };
        let t5 = _OrderedChoice {
            arg_lhs: t,
            arg_rhs: t2,
        };
        let t6 = _OrderedChoice {
            arg_lhs: t3,
            arg_rhs: t4,
        };
        let t7 = _OrderedChoice {
            arg_lhs: t5,
            arg_rhs: t6,
        };
        let s = t7.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1)
    }

    #[test]
    fn test_ordered_choice_deep_nested() {
        /* The purpose of this test is merely to see if it chokes on deep nesting */
        let source = "Hello World";
        let position: u32 = 0;
        let t = _Terminal {
            arg: "f".to_string().as_bytes()[0],
        };
        let t2 = _Terminal {
            arg: "e".to_string().as_bytes()[0],
        };
        let t3 = _Terminal {
            arg: "h".to_string().as_bytes()[0],
        };
        let t4 = _Terminal {
            arg: "n".to_string().as_bytes()[0],
        };
        let t5 = _OrderedChoice {
            arg_lhs: t,
            arg_rhs: t2,
        };
        let t6 = _OrderedChoice {
            arg_lhs: t3,
            arg_rhs: t4,
        };
        let t7 = _OrderedChoice {
            arg_lhs: t5,
            arg_rhs: t6,
        };
        let t8 = _OrderedChoice {
            arg_lhs: t6,
            arg_rhs: t7,
        };
        let t9 = _OrderedChoice {
            arg_lhs: t7,
            arg_rhs: t8,
        };
        let t10 = _OrderedChoice {
            arg_lhs: t8,
            arg_rhs: t9,
        };
        let t11 = _OrderedChoice {
            arg_lhs: t9,
            arg_rhs: t10,
        };
        let t12 = _OrderedChoice {
            arg_lhs: t10,
            arg_rhs: t11,
        };
        let t13 = _OrderedChoice {
            arg_lhs: t11,
            arg_rhs: t12,
        };
        let t14 = _OrderedChoice {
            arg_lhs: t12,
            arg_rhs: t13,
        };
        let t15 = _OrderedChoice {
            arg_lhs: t13,
            arg_rhs: t14,
        };
        let t16 = _OrderedChoice {
            arg_lhs: t14,
            arg_rhs: t15,
        };
        let t17 = _OrderedChoice {
            arg_lhs: t15,
            arg_rhs: t16,
        };
        let t18 = _OrderedChoice {
            arg_lhs: t16,
            arg_rhs: t17,
        };
        let t19 = _OrderedChoice {
            arg_lhs: t17,
            arg_rhs: t18,
        };
        let t20 = _OrderedChoice {
            arg_lhs: t18,
            arg_rhs: t19,
        };
        let t21 = _OrderedChoice {
            arg_lhs: t19,
            arg_rhs: t20,
        };
        let t22 = _OrderedChoice {
            arg_lhs: t20,
            arg_rhs: t21,
        };
        let t23 = _OrderedChoice {
            arg_lhs: t21,
            arg_rhs: t22,
        };
        let t24 = _OrderedChoice {
            arg_lhs: t22,
            arg_rhs: t23,
        };
        let t25 = _OrderedChoice {
            arg_lhs: t23,
            arg_rhs: t24,
        };
        let s = t25.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1)
    }
}
