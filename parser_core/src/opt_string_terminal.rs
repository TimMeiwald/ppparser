#![allow(unused_variables)]
use crate::source::Source;
use cache::{Cache, AST};
use publisher::Publisher;

fn _string_terminal_kernel(source: &Source, position: u32, data: &[char]) -> (bool, u32) {
    let mut end_position = position; // This is a used variable in line 24.
    for char in data {
        let chr_len = char.len_utf8() as u32;
        let r = source.get_multiple_chars(position, chr_len - 1);
        let c = match r {
            None => return (false, position),
            Some(value) => value,
        };
        let str = String::from_utf8(c.to_vec());
        match str {
            Err(_) => return (false, position),
            Ok(chr) => {
                let f = chr.chars().next();
                match f {
                    None => return (false, position),
                    Some(value) => {
                        if *char == value {
                            end_position += chr_len;
                        } else {
                            return (false, position);
                        }
                    }
                };
            }
        };
    }
    (false, position)
}

pub fn _string_terminal<T: Cache, S: Publisher>(
) -> impl Fn(&Source, u32, &[char]) -> (bool, u32) + 'static {
    move |source: &Source, position: u32, data: &[char]| {
        _string_terminal_kernel(source, position, data)
    }
}
