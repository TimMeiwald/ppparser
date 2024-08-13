use crate::Rules;

impl From<u32> for Rules {
    fn from(i: u32) -> Rules {
        let element = num::FromPrimitive::from_u32(i);
        match element {
            Some(rule) => rule,
            None => panic!("Not a valid Rule"),
        }
    }
}

impl PartialEq for Rules {
    fn eq(&self, other: &Self) -> bool {
        (*self as u32) == (*other as u32)
    }
}
