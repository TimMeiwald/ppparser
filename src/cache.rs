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



// One per Parse
pub struct Cache{
    entries: Vec<ArgCache>, // Start Position encoded in the indexing of the Cache
}

impl Cache{
    fn push(&mut self, position: u32, arg_key: u32, bool: bool, end_position: u32){
        let arg_cache: &mut ArgCache = &mut self.entries[position as usize];
        println!("GOT HERE");
        arg_cache.entries[arg_key as usize] = (bool, end_position);
        println!("GOT HERE2")
        
    }
    fn check(&self, position: u32, arg_key: u32) -> Option<(bool, u32)>{
        let ret: (bool, u32) = self.entries[position as usize].entries[arg_key as usize];
        if ret.1 != u32::MAX{
            // Result is returned to callee to unwrap
            println!("Used Cached Value");
            return Some(ret);
        }
        else{
            // Tells callee to simply run the actual code instead of using cached value since one does not exist.
            println!("Did not use Cache Value");
            return None;
        };
    }
}

// Create 1 per Position in Cache
pub struct ArgCache{
    entries: Vec<(bool, u32)> // Struct type encoded in the position of the entries
}


pub fn cache_constructor(size_of_source: u32, number_of_structs: u32) -> Cache {
    let mut c = Cache{entries: Vec::with_capacity(size_of_source as usize)};
    for i in 0..size_of_source {
        // Ensures the Vector in Cache is as large as the input source
        c.entries.push(ArgCache { entries: Vec::with_capacity(number_of_structs as usize) });
        for _j in 0..number_of_structs{
            // Ensures the Vector in ArgCache is as large as the number of structs(Aka possible arguments since each struct implements resolvable, which is known at parser generation time)
            c.entries[i as usize].entries.push((false, u32::MAX));
        }
    }
    return c;
    // for every arg cache in c set size to <number_of_structs>
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_cache_nothing_cached() {
        // Simulating what would happen after a rule that consumes one character parses and returns true
        let arg_key: u32 = 1; 
        let start_position = 0;
        let f = cache_constructor(10,10); // 10 just cos it's a test no particular meaning
        let s: Option<(bool, u32)> = f.check(start_position, arg_key);
        assert!(s.is_none());
    }

    #[test]
    fn test_cache() {
        // Simulating what would happen after a rule that consumes one character parses and returns true
        let arg_key: u32 = 1; 
        let start_position = 0;
        let result = true;
        let end_position = 1;
        let mut f = cache_constructor(10,10); // 10 just cos it's a test no particular meaning
        f.push(start_position, arg_key, result, end_position);
        let (b, p) = f.check(start_position, arg_key).unwrap();
        println!("{:?}, {:?}", b, p);
        assert_eq!(b, true);
        assert_eq!(p, 1);
    }

}
