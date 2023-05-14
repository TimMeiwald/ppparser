#[derive(Debug)]
#[derive(PartialEq)]
struct ParserObject{
    position: u32,
    source: String,
}

fn main() {
    let something: &str = "This is a string";
}

fn c_token(po: &ParserObject) -> Option<u8> {
    if po.position >= po.source.chars().count() as u32 {
        return Option::None;
    }
    let s: u8 = po.source.as_bytes()[po.position as usize];
    return Option::Some(s);
}

fn c_terminal(po: &mut ParserObject, arg: u8) -> bool {
    /* If character at po.position is equal to arg, increment position and return True, else return False */

    if arg == c_token(&po).unwrap() {
        po.position = po.position + 1;
        return true;
    }
    else{
        return false;
    }
}

fn c_optional<T: Copy>(po: &mut ParserObject, pair: (&dyn Fn(&mut ParserObject, T) -> bool, T))-> bool {
    /* True if matches, False if not. Increments position on a match */

    // Fn(&u8), u8
    // Fn(&Fn), Fn
    let temp_position = po.position;
    let (func, arg) = pair; // unpack
    let bool = func(po, arg);

    if bool == true {
        return true
    }
    else{
        po.position = temp_position;
        return true;
    }
}


fn c_sequence<T: Copy, U: Copy>(po: &mut ParserObject, pair: ((&dyn Fn(&mut ParserObject, T) -> bool,T), (&dyn Fn(&mut ParserObject, U) -> bool,U))) -> bool{
//fn c_sequence<T: Copy, U: Copy>(po: &mut ParserObject, lhs: (&dyn Fn(&mut ParserObject, T) -> bool, T), rhs:(&dyn Fn(&mut ParserObject, U) -> bool, U)) -> bool {
    /* True if all expressions match, then updates position, else false, no positional update */

    let tmp_pos = po.position;
    let (lhs, rhs) = pair;
    let (lhs_func, lhs_arg) = lhs;
    let (rhs_func, rhs_arg) = rhs;

    let lhs_bool: bool = lhs_func(po, lhs_arg);
    let rhs_bool: bool = rhs_func(po, rhs_arg);

    if lhs_bool && rhs_bool {
        return true;
    }
    else {
        po.position = tmp_pos;
        return false;
    }
}

fn c_ordered_choice<T: Copy, U: Copy>(po: &mut ParserObject, lhs: (&dyn Fn(&mut ParserObject, T) -> bool,T), rhs: (&dyn Fn(&mut ParserObject, U) -> bool,U)) -> bool {
    /* True if one expression matches, then updates position, else false, no positional update */

    let tmp_pos = po.position;
    let (lhs_func, lhs_arg) = lhs;
    let (rhs_func, rhs_arg) = rhs;

    let lhs_bool: bool = lhs_func(po, lhs_arg);
    if lhs_bool {
        return true;
    }
    po.position = tmp_pos;

    let rhs_bool: bool = rhs_func(po, rhs_arg);
    if rhs_bool {
        return true;
    }
    po.position = tmp_pos;

    return false;
}

fn c_zero_or_more<T: Copy>(po: &mut ParserObject, pair: (&dyn Fn(&mut ParserObject, T) -> bool, T))-> bool {
    /* Always True, increments position each time the expression matches else continues without doing anything */

    let mut temp_position = po.position;
    let (func, arg) = pair; // unpack

    let mut bool = func(po, arg);

    loop {
        bool = func(po, arg);

        if bool {
            temp_position = po.position;
            continue;
        }
        else {
            po.position = temp_position;
            break;
        }
    }
    return true;
}

fn c_one_or_more<T: Copy>(po: &mut ParserObject, pair: (&dyn Fn(&mut ParserObject, T) -> bool, T)) -> bool {
    /* True if matches at least once, increments position each time the expression matches */

    let mut temp_position = po.position;
    let (func, arg) = pair; // unpack

    let mut bool = func(po, arg);

    if bool {
        temp_position = po.position;
    } else {
        po.position = temp_position;
        return false;
    }

    loop {
        bool = func(po, arg);
        if bool {
            temp_position = po.position;
            continue;
        } else {
            po.position = temp_position;
            break;
        }
    }
    return true;
}

fn c_and<T: Copy>(po: &mut ParserObject, pair: (&dyn Fn(&mut ParserObject, T) -> bool, T)) -> bool {
    /* True if the function results in True, never increments position */

    let temp_position = po.position;
    let (func, arg) = pair; // unpack
    let bool = func(po, arg);

    if bool {
        po.position = temp_position;
        return true;
    } else {
        po.position = temp_position;
        return false;
    }
}

fn c_not<T: Copy>(po: &mut ParserObject, pair: (&dyn Fn(&mut ParserObject, T) -> bool, T)) -> bool {
    /* True if the function results in False, never increments position */

    return !c_and(po, pair);
}

fn c_subexpression<T: Copy>(po: &mut ParserObject, pair: (&dyn Fn(&mut ParserObject, T) -> bool, T)) -> bool {
    /* Subexpression is any expression inside a pair of () brackets
    SUBEXPR essentially does nothing but allows for order of precedent
    more importantly order of precedence is very restricted because it made my life hard
    (mostly because I can't find a good definition of what order of precedence is in PEG) so use SUBEXPR
    to make more complicated rules */

    let (func, arg) = pair;
    let temp_position = po.position;
    let bool = func(po, arg);

    if bool {
        return true;
    } else {
        po.position = temp_position;
        return false;
    }
}


fn c_var_name<T: Copy>(po: &mut ParserObject, pair: (&dyn Fn(&mut ParserObject, T) -> bool, T)) -> bool{
    /* True if called function evaluates to true else false, Is used to call other functions*/

    let (func, arg) = pair;
    let temp_position = po.position;
    let bool = func(po, arg);

    if bool {
        return true;
    }
    else {
        po.position = temp_position;
        return false;
    }
}

// fn thing(po: &ParserObject)-> bool{
//     return c_subexpression(&mut po, (&c_sequence, (&c_sequence, (&c_var_name, (&Left_Bracket, 0)), (&c_var_name, (&RHS, 0))), (&c_var_name, (&Right_Bracket, 0))))

// }







// #[test]
// fn more_complex_test(){
//     let mut myobj: ParserObject = ParserObject { position: 0, source: "AAABBBC".to_string() };
//     let char1 = "A".as_bytes()[0];
//     let char2 = "B".as_bytes()[0];

//     let my_bool: bool = c_sequence(&mut myobj,  (&c_one_or_more, (&c_terminal, char1)),(&c_one_or_more, (&c_terminal, char2)));
//     assert_eq!(true, my_bool);
//     assert_eq!(myobj.position, 6)
// }



#[test]
fn more_complex_test2(){
    let mut myobj: ParserObject = ParserObject { position: 0, source: "eAAABBBC".to_string() };
    let char1 = "A".as_bytes()[0];
    let char2 = "B".as_bytes()[0];

    let my_bool: bool = c_optional(&mut myobj, (&c_one_or_more, (&c_terminal, char1)));

    assert_eq!(true, my_bool);
    assert_eq!(myobj.position, 0)
}


#[test]
fn more_complex_test3(){
    let mut myobj: ParserObject = ParserObject { position: 0, source: "eAAABBBC".to_string() };
    let char1 = "A".as_bytes()[0];
    let char2 = "B".as_bytes()[0];

    let my_bool_2: bool = c_one_or_more(&mut myobj, (&c_sequence, ((&c_terminal, char1), (&c_terminal, char1))));
    assert_eq!(true, my_bool_2);
    assert_eq!(myobj.position, 0)
}

