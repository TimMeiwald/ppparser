

#[derive(Copy, Clone)]
pub struct Index(pub u32);


impl From<Index> for u32 {
    fn from(i: Index) -> u32 {
        i.0
    }
}
pub trait Cache {
    fn new(size_of_source: u32, number_of_structs: u32) -> Self;
    fn push(&mut self, rule: u32, is_true: bool, start_position: u32, end_position: u32, reference: Index);
    fn check(&self, rule: u32, start_position: u32) -> Option<(bool, u32, Index)>;
    fn clear(&mut self);
    fn reinitialize(&mut self); //Reset state without deallocating memory for reuse.
}
