#[derive(Copy, Clone, Debug)]
pub struct Key(pub usize);
// Whatever method the publisher uses to refer to other locations in the tree/stack or whatever is being used for output
// Up to the publisher. 

impl From<Key> for usize {
    fn from(i: Key) -> usize {
        i.0
    }
}