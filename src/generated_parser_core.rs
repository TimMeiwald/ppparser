
pub trait Resolvable {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32);
}

#[derive(Copy, Clone)]
pub struct Terminal {
    pub arg: u8,
}

impl Resolvable for Terminal {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return terminal(position, source, self.arg);
    }
}

fn terminal(position: u32, source: &str, arg: u8) -> (bool, u32) {
    /* If character at po.position is equal to arg, increment position and return True, else return False */
    if arg == token(position, source).unwrap() {
        let position = position + 1;
        return (true, position);
    } else {
        return (false, position);
    }
}


#[derive(Copy, Clone)]
pub struct AndPredicate<T: Resolvable> {
    arg: T,
}

impl<T: Resolvable + Copy> Resolvable for AndPredicate<T> {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return and_predicate(position, source, self.arg);
    }
}

pub fn and_predicate<T: Resolvable>(position: u32, source: &str, arg: T) -> (bool, u32) {
    /* Always True, increments position each time the expression matches else continues without doing anything */
    // Only public so Not Predicate can use it since it just inverts it.

    let temp_position = position;
    let ret = arg.resolve(position, source);
    let bool = ret.0;
    if bool {
        return (true, temp_position);
    } else {
        return (false, temp_position);
    }
}


#[derive(Copy, Clone)]
pub struct NotPredicate<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for NotPredicate<T> {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return not_predicate(position, source, self.arg);
    }
}

fn not_predicate<T: Resolvable>(position: u32, source: &str, arg: T) -> (bool, u32) {
    /* Always True, increments position each time the expression matches else continues without doing anything */

    let (bool, position) = and_predicate::and_predicate(position, source, arg);
    return (!bool, position);
}


#[derive(Copy, Clone)]
pub struct Optional<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for Optional<T> {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return optional(position, source, self.arg);
    }
}

fn optional<T: Resolvable>(position: u32, source: &str, args: T) -> (bool, u32) {
    /* True if matches, False if not. Increments position on a match */

    // Fn(&u8), u8
    // Fn(&Fn), Fn
    let temp_position = position;
    let (bool, position) = args.resolve(position, source);

    if bool == true {
        return (true, position);
    } else {
        let position = temp_position;
        return (true, position);
    }
}


#[derive(Copy, Clone)]
pub struct SubExpression<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for SubExpression<T> {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return subexpression(position, source, self.arg);
    }
}

fn subexpression<T: Resolvable>(position: u32, source: &str, arg: T) -> (bool, u32) {
    /* Subexpression is any expression inside a pair of () brackets
    SUBEXPR essentially does nothing but allows for order of precedent
    more importantly order of precedence is very restricted because it made my life hard
    (mostly because I can't find a good definition of what order of precedence is in PEG) so use SUBEXPR
    to make more complicated rules */

    let temp_position = position;
    let (bool, position) = arg.resolve(position, source);

    if bool {
        return (true, position);
    } else {
        return (false, temp_position);
    }
}


#[derive(Copy, Clone)]
pub struct VarName<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for VarName<T> {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return var_name(position, source, self.arg);
    }
}

fn var_name<T: Resolvable>(position: u32, source: &str, arg: T) -> (bool, u32) {
    /* Always True, increments position each time the expression matches else continues without doing anything */
    // NB: Currently Identical to subexpression but only because an AST isn't being built yet.

    let temp_position = position;
    let (bool, position) = arg.resolve(position, source);

    if bool {
        return (true, position);
    } else {
        return (false, temp_position);
    }
}


#[derive(Copy, Clone)]
pub struct ZeroOrMore<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for ZeroOrMore<T> {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return zero_or_more(position, source, self.arg);
    }
}

fn zero_or_more<T: Resolvable>(position: u32, source: &str, arg: T) -> (bool, u32) {
    /* Always True, increments position each time the expression matches else continues without doing anything */

    let mut temp_position = position;
    let mut bool;
    let mut position = position;
    loop {
        let ret = arg.resolve(position, source);
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


#[derive(Copy, Clone)]
pub struct OneOrMore<T: Resolvable> {
    pub arg: T,
}

impl<T: Resolvable + Copy> Resolvable for OneOrMore<T> {
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return one_or_more(position, source, self.arg);
    }
}

fn one_or_more<T: Resolvable>(position: u32, source: &str, arg: T) -> (bool, u32) {
    /* Always True, increments position each time the expression matches else continues without doing anything */

    let mut temp_position = position;
    let (mut bool, mut position) = arg.resolve(position, source);
    if bool {
        temp_position = position;
    } else {
        position = temp_position;
        return (false, position);
    }
    loop {
        let ret = arg.resolve(position, source);
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
