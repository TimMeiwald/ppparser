use cache::AST;

use crate::source::Source;

fn _string_terminal_kernel_ascii_opt(source: &Source, position: u32, data: &[u8]) -> (bool, u32) {
    let mut end_position = position;
    for char in data {
        let s = match source.get_char(end_position) {
            None => return (false, position),
            Some(val) => val,
        };
        if *char == s {
            end_position += 1;
        } else {
            return (false, position);
        }
    }
    (true, end_position)
}

pub fn _string_terminal_opt_ascii(data: &[u8]) -> impl Fn(&Source, u32) -> (bool, u32) + '_ {
    move |source: &Source, position: u32| _string_terminal_kernel_ascii_opt(source, position, data)
}
