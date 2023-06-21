fn token(position: u32, source: &str) -> Option<u8> {
    if position >= source.chars().count() as u32 {
        return Option::None;
    }
    let s: u8 = source.as_bytes()[position as usize];
    return Option::Some(s);
}

pub trait Resolvable {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32);
}

#[derive(Copy, Clone)]
pub struct _Terminal {
    pub arg: u8,
}

impl Resolvable for _Terminal {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return terminal(position, source, self.arg);
    }
}

fn terminal(position: u32, source: &str, arg: u8) -> (bool, u32) {
    /* If character at po.position is equal to arg, increment position and return True, else return False */
    if arg == token(position, source).unwrap() {
        let position = position + 1;
        return (true, position);
    } else {
        return (false, position);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_terminal_true() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = _Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let s = t.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 1);
    }

    #[test]
    fn test_terminal_false() {
        let source = "Hello World";
        let position: u32 = 0;
        let t = _Terminal {
            arg: "h".to_string().as_bytes()[0],
        };
        let s = t.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }
}
