// def cache(func):
//     """Handles regular PEG"""
//     name = func.__name__
//     @wraps(func)
//     def kernel(*args):
//         obj = args[0]
//         args = args[1:]
//         position = obj.position
//         try:
//             bool, pos = obj.cache.get(name, position, args)
//             bool, pos = obj.cache.get(name, position, args)
//             if bool:
//                 obj.position = pos
//             if(func.__name__ in ["And_Predicate", "Not_Predicate", "Optional", "Ordered_Choice", "Sequence", "Var_Name","_TERMINAL", "many_A"] and bool == True):
//                 print(f"k: Token: {position}, {func.__name__} -> '{obj.src[position:obj.position]}'")
//             return bool
//         except KeyError:
//             bool = func(obj, *args)
//             obj.cache.set(name, position, args, (bool, obj.position))
//             if(func.__name__ in ["And_Predicate", "Not_Predicate", "Optional", "Ordered_Choice", "Sequence", "Var_Name", "_TERMINAL", "many_A"] and bool == True):
//                 print(f"nk: Token: {position}, {func.__name__} -> '{obj.src[position:obj.position]}'")
//             return bool

use std::string;

//     return kernel
use crate::{terminal::Resolvable};

pub struct CacheEntry<'a>{
    start_position: u32, 
    argument: &'a dyn Resolvable, 
    bool: bool,
    end_position: u32,
}
impl<'a> CacheEntry<'a>{
    fn represent(&self) -> String{
        return format!("{:?}, {:?}, {:?}", self.start_position, self.bool, self.end_position).to_string()
    }
}


// Cache needs to implicitly use Start_Position as it's index into other vectors which then contain every entry for that position to minimize computation.

pub struct Cache<'a>{
    entries: Vec<CacheEntry<'a>>
}

impl<'a> Cache<'a>{
    fn push(&mut self, entry: CacheEntry<'a>){
        self.entries.push(entry)
    }

    fn test(&self, start_position: u32, arg: &dyn Resolvable){
        for i in &self.entries{
            println!("{:?} {:?} {:?}", i.start_position, i.bool, i.end_position);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::_Terminal;

    #[test]
    fn test_cache_entry() {
        let t = _Terminal {
            arg: "H".to_string().as_bytes()[0],
        };
        let s: CacheEntry = CacheEntry { start_position: 0, argument: &t, bool: true, end_position: 1 };
        println!("{}", s.represent())

    }

}
