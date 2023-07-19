
pub fn token(position: u32, source: &str) -> u8 {
    if position < source.chars().count() as u32 {
        let s: u8 = source.as_bytes()[position as usize];
        return s;
    } else {
        //println!("END OF TOKEN STREAM");
        return 0;
    }
}

pub trait Resolvable {
    fn resolve(&self, output: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32);
}

#[derive(Copy, Clone)]
pub struct _Terminal {
    pub arg: u8,
}

impl Resolvable for _Terminal {
    fn resolve(&self, _output: &mut Stack, _cache: &mut Cache, position: u32, source: &str) -> (bool, u32) {
        return terminal(position, source, self.arg);
    }
}

fn terminal(position: u32, source: &str, arg: u8) -> (bool, u32) {
    let t = token(position, source);
    //println!("Arg: {:?}, Token: {:?}", std::str::from_utf8(&[arg]), std::str::from_utf8(&[t]));
    if t == 0 {
        return (false, position);
    }
    if arg == t {
        let position = position + 1;
        return (true, position);
    } else {
        return (false, position);
    }
}


impl<T: Resolvable + Copy> Resolvable for _AndPredicate<T> {
    fn resolve(&self, stack: &mut Stack,  cache: &mut Cache, position: u32, source: &str) -> (bool, u32) {
        return and_predicate(stack, cache, position, source, self.arg);
    }
}

pub fn and_predicate<T: Resolvable>(
    stack: &mut Stack,
    cache: &mut Cache,
    position: u32,
    source: &str,
    arg: T,
) -> (bool, u32) {
    /* Always True, increments position each time the expression matches else continues without doing anything */
    // Only public so Not Predicate can use it since it just inverts it.

    let temp_position = position;
    let ret = arg.resolve(stack, cache, position, source);
    let bool = ret.0;
    if bool {
        return (true, temp_position);
    } else {
        return (false, temp_position);
    }
}


//
#[derive(Copy, Clone)]
pub struct _NotPredicate<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for _NotPredicate<T> {
    fn resolve(&self,stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) {
        return not_predicate(stack, cache, position, source, self.arg);
    }
}

fn not_predicate<T: Resolvable>(
    stack: &mut Stack, 
    cache: &mut Cache,
    position: u32,
    source: &str,
    arg: T,
) -> (bool, u32) {
    /* Always True, increments position each time the expression matches else continues without doing anything */

    let (bool, position) = and_predicate(stack, cache, position, source, arg);
    return (!bool, position);
}


//
#[derive(Copy, Clone)]
pub struct _Optional<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for _Optional<T> {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) {
        return optional(stack, cache, position, source, self.arg);
    }
}

fn optional<T: Resolvable>(stack: &mut Stack, cache: &mut Cache, position: u32, source: &str, args: T) -> (bool, u32) {
    /* True if matches, False if not. Increments position on a match */

    // Fn(&u8), u8
    // Fn(&Fn), Fn
    let temp_position = position;
    let (bool, position) = args.resolve(stack, cache, position, source);

    if bool == true {
        return (true, position);
    } else {
        let position = temp_position;
        return (true, position);
    }
}


//
#[derive(Copy, Clone)]
pub struct _SubExpression<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for _SubExpression<T> {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) {
        return subexpression(stack, cache, position, source, self.arg);
    }
}

fn subexpression<T: Resolvable>(
    stack: &mut Stack,
    cache: &mut Cache,
    position: u32,
    source: &str,
    arg: T,
) -> (bool, u32) {
    /* Subexpression is any expression inside a pair of () brackets
    SUBEXPR essentially does nothing but allows for order of precedent
    more importantly order of precedence is very restricted because it made my life hard
    (mostly because I can't find a good definition of what order of precedence is in PEG) so use SUBEXPR
    to make more complicated rules */

    let temp_position = position;
    let (bool, position) = arg.resolve(stack, cache, position, source);

    if bool {
        return (true, position);
    } else {
        return (false, temp_position);
    }
}


//
#[derive(Copy, Clone)]
pub struct _VarName<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for _VarName<T> {
    fn resolve(&self, stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) {
        return var_name(stack, cache, position, source, self.arg);
    }
}

fn var_name<T: Resolvable>(stack: &mut Stack, cache: &mut Cache, position: u32, source: &str, arg: T) -> (bool, u32) {
    /* Always True, increments position each time the expression matches else continues without doing anything */
    // NB: Currently Identical to subexpression but only because an AST isn't being built yet.

    let temp_position = position;
    let (bool, position) = arg.resolve(stack, cache, position, source);

    if bool {
        return (true, position);
    } else {
        return (false, temp_position);
    }
}


//
#[derive(Copy, Clone)]
pub struct _ZeroOrMore<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for _ZeroOrMore<T> {
    fn resolve(&self, stack: &mut Stack,  cache: &mut Cache, position: u32, source: &str) -> (bool, u32) {
        return zero_or_more(stack, cache, position, source, self.arg);
    }
}

fn zero_or_more<T: Resolvable>(
    stack: &mut Stack,
    cache: &mut Cache,
    position: u32,
    source: &str,
    arg: T,
) -> (bool, u32) {
    /* Always True, increments position each time the expression matches else continues without doing anything */

    let mut temp_position = position;
    let mut bool;
    let mut position = position;
    loop {
        let ret = arg.resolve(stack, cache, position, source);
        bool = ret.0;
        position = ret.1;
        if bool {
            temp_position = position;
            continue;
        } else {
            position = temp_position;
            break;
        }
    }
    return (true, position);
}


//
#[derive(Copy, Clone)]
pub struct _OneOrMore<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for _OneOrMore<T> {
    fn resolve(&self,stack: &mut Stack, cache: &mut Cache, position: u32, source: &str) -> (bool, u32) {
        return one_or_more(stack, cache, position, source, self.arg);
    }
}

fn one_or_more<T: Resolvable>(
    stack: &mut Stack, 
    cache: &mut Cache,
    position: u32,
    source: &str,
    arg: T,
) -> (bool, u32) {
    /* Always True, increments position each time the expression matches else continues without doing anything */

    let mut temp_position = position;
    let (mut bool, mut position) = arg.resolve(stack, cache, position, source);
    if bool {
        temp_position = position;
    } else {
        position = temp_position;
        return (false, position);
    }
    loop {
        let ret = arg.resolve(stack, cache, position, source);
        bool = ret.0;
        position = ret.1;
        if bool {
            temp_position = position;
            continue;
        } else {
            position = temp_position;
            break;
        }
    }
    return (true, position);
}


// One per Parse
pub struct Cache {
    entries: Vec<ArgCache>, // Start Position encoded in the indexing of the Cache
}

impl Cache {
    
    pub fn new(size_of_source: u32, number_of_structs: u32) -> Self {
        let mut c = Cache {
            entries: Vec::with_capacity(size_of_source as usize),
        };
        for i in 0..size_of_source {
            // Ensures the Vector in Cache is as large as the input source
            c.entries.push(ArgCache {
                entries: Vec::with_capacity(number_of_structs as usize),
            });
            for _j in 0..number_of_structs {
                // Ensures the Vector in ArgCache is as large as the number of structs(Aka possible arguments since each struct implements resolvable, which is known at parser generation time)
                c.entries[i as usize].entries.push((false, u32::MAX));
            }
        }
        return c;
        // for every arg cache in c set size to <number_of_structs>
    }

    fn push(&mut self, position: u32, arg_key: u32, bool: bool, end_position: u32) {
        let arg_cache: &mut ArgCache = &mut self.entries[position as usize];
        arg_cache.entries[arg_key as usize] = (bool, end_position);
    }
    fn check(&self, position: u32, arg_key: u32) -> Option<(bool, u32)> {
        let ret: (bool, u32) = self.entries[position as usize].entries[arg_key as usize];
        if ret.1 != u32::MAX {
            // Result is returned to callee to unwrap
            return Some(ret);
        } else {
            // Tells callee to simply run the actual code instead of using cached value since one does not exist.
            return None;
        };
    }
}

// Create 1 per Position in Cache
pub struct ArgCache {
    entries: Vec<(bool, u32)>, // Struct type encoded in the position of the entries
}



pub fn cache_struct_wrapper<T: Resolvable>(
    stack: &mut Stack,
    cache: &mut Cache,
    rule: T,
    arg_key: u32,
    position: u32,
    source: &str,
) -> (bool, u32) {
    let ret = cache.check(position, arg_key);
    if ret != None {
        return ret.unwrap();
    } else {
        cache.push(position, arg_key, false, 0);
        let ret = rule.resolve(stack, cache, position, source);
        cache.push(position, arg_key, ret.0, ret.1);
        return ret;
    }
}

pub fn cache_fn_wrapper(
    stack: &mut Stack,
    cache: &mut Cache,
    rule_function: fn(&mut Stack, &mut Cache, u32, &str) -> (bool, u32),
    arg_key: u32,
    position: u32,
    source: &str,
) -> (bool, u32) {
    /*
       Use this to wrap functions, i.e if using handwritten functions to improve performance.
    */
    let ret = cache.check(position, arg_key);
    if ret != None {
        return ret.unwrap();
    } else {
        cache.push(position, arg_key, false, 0);
        let ret = rule_function(stack, cache, position, source);
        cache.push(position, arg_key, ret.0, ret.1);
        return ret;
    }
}


