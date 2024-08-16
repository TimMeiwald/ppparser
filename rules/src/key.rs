#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Key(pub u32);
// Whatever method the publisher uses to refer to other locations in the tree/stack or whatever is being used for output
// Up to the publisher.

impl From<Key> for usize {
    fn from(i: Key) -> usize {
        i.0 as usize
    }
}
