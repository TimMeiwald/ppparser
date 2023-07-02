use crate::Resolvable;
use crate::cache::Cache;
#[derive(Copy, Clone)]
pub struct _VarName<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for _VarName<T> {
    fn resolve(&self,cache: &mut Cache, position: u32, source: &str) -> (bool, u32) {
        return var_name(cache,position ,source, self.arg);
    }
}

fn var_name<T: Resolvable>(cache: &mut Cache, position:u32, source: &str, arg: T) -> (bool, u32) {
    /* Always True, increments position each time the expression matches else continues without doing anything */
    // NB: Currently Identical to subexpression but only because an AST isn't being built yet.

    let temp_position = position;
    let (bool, position) = arg.resolve(cache, position, source);

    if bool {
        return (true, position);
    } else {
        return (false, temp_position);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_Terminal;
    #[test]
    fn test_var_name_true() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = _Terminal {
            arg: "f".to_string().as_bytes()[0],
        };
        let t2 = _VarName { arg: t };
        let s = t2.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }

    #[test]
    fn test_var_name_false() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = _Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let t2 = _VarName { arg: t };
        let s = t2.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 1);
    }
}
