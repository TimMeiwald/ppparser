use crate::source::Source;
use crate::Context;
use cache::Cache;
use publisher::Publisher;
use rules::Rules;

fn _ordered_choice_match_range_kernel(
    source: &Source,
    position: u32,
    start: u32,
    end: u32,
) -> (bool, u32) {
    let chr = source.get_char(position);
    match chr {
        None => return (false, position),
        Some(value) => {
            if (value as u32 >= start) && (value as u32 <= end) {
                return (true, position + 1);
            } else {
                return (false, position);
            }
        }
    }
}

pub fn _ordered_choice_match_range(start: u32, end: u32) -> impl Fn(&Source, u32) -> (bool, u32) {
    move |source: &Source, position: u32| {
        _ordered_choice_match_range_kernel(source, position, start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optional::{_optional, _optional_kernel};
    use crate::source::Source;
    use crate::terminal::_terminal;
    #[test]
    fn test_ordered_choice_kernel_1() {
        let s = "0".to_string();
        let s = Source::new(s);
        let x = _ordered_choice_match_range_kernel(&s, 0, 48, 57);
        assert_eq!(x, (true, 1));
    }
    #[test]
    fn test_ordered_choice_kernel_2() {
        let s = "9".to_string();
        let s = Source::new(s);
        let x = _ordered_choice_match_range_kernel(&s, 0, 48, 57);
        assert_eq!(x, (true, 1));
    }
}
