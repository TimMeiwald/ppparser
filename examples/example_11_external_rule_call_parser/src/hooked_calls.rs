
use crate::{Key, Source};

pub fn logger(
    func: &impl Fn(Key, &Source, u32) -> (bool, u32),
) -> impl Fn(Key, &Source, u32) -> (bool, u32) + '_ {
    move |parent: Key, source: &Source, position: u32| {
        println!("Before function execution");
        let result = func(parent, source, position);
        println!("Result is {:?} {:?}", result.0, result.1);
        println!("After function execution");
        result
    }
}
