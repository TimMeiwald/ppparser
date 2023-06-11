#[derive(Debug)]
#[derive(PartialEq)]
struct ParserObject{
    position: u32,
    source: String,
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

fn c_optional<T>(po: &mut ParserObject, pair: (Box<&dyn Fn(&mut ParserObject, T) -> bool > , T)) -> bool {
    /* True if matches, False if not. Increments position on a match */

    // Fn(&u8), u8
    // Fn(&Fn), Fn
    let temp_position = po.position;
    let (func, arg) = pair;
    let bool: bool = func(po, arg);

    if bool == true {
        return true
    }
    else{
        po.position = temp_position;
        return true;
    }
}


// fn c_ordered_choice<T, U>(po: &mut ParserObject, pair: ((Box<fn(&mut ParserObject, T) -> bool>,T), (Box<fn(&mut ParserObject, U) -> bool>,U))) -> bool{
//     /* True if one expression matches, then updates position, else false, no positional update */

//     let tmp_pos = po.position;
//     let (lhs, rhs) = pair;
//     let (lhs_func, lhs_arg) = lhs;
//     let (rhs_func, rhs_arg) = rhs;

//     let lhs_bool: bool = lhs_func(po, lhs_arg);
//     if lhs_bool {
//         return true;
//     }
//     po.position = tmp_pos;

//     let rhs_bool: bool = rhs_func(po, rhs_arg);
//     if rhs_bool {
//         return true;
//     }
//     po.position = tmp_pos;

//     return false;
// }



// fn c_subexpression<T>(po: &mut ParserObject, pair: (Box<fn(&mut ParserObject, T) -> bool>, T)) -> bool {
//     /* Subexpression is any expression inside a pair of () brackets
//     SUBEXPR essentially does nothing but allows for order of precedent
//     more importantly order of precedence is very restricted because it made my life hard
//     (mostly because I can't find a good definition of what order of precedence is in PEG) so use SUBEXPR
//     to make more complicated rules */

//     let (func, arg) = pair;
//     let temp_position = po.position;
//     let bool = func(po, arg);

//     if bool {
//         return true;
//     } else {
//         po.position = temp_position;
//         return false;
//     }
// }


fn num(po: &mut ParserObject) -> bool {
    /*
    <Num> = "0"/"1"/"2"/"3"/"4"/"5"/"6"/"7"/"8"/"9" ;
    */
    return c_subexpression(po, (Box::new(c_ordered_choice), ((Box::new(c_ordered_choice), ((Box::new(c_ordered_choice), ((Box::new(c_ordered_choice), ((Box::new(c_ordered_choice), ((Box::new(c_ordered_choice), ((Box::new(c_ordered_choice), ((Box::new(c_ordered_choice), ((Box::new(c_ordered_choice), ((Box::new(c_terminal), 48), (Box::new(c_terminal), 49))), (Box::new(c_terminal), 50))), (Box::new(c_terminal), 51))), (Box::new(c_terminal), 52))), (Box::new(c_terminal), 53))), (Box::new(c_terminal), 54))), (Box::new(c_terminal), 55))), (Box::new(c_terminal), 56))), (Box::new(c_terminal), 57))));
}

#[test]
fn test_ordered_choice(){
    let mut po = ParserObject{position: 0, source: "fHello there".to_string()};
    //let b: bool = Box::new(c_optional)(&mut po, (Box::new(c_optional), Box::new((c_terminal, 20))));
    //let s= Box::new(c_terminal);
    let char1 = "H".to_string().as_bytes()[0];
    let char2 = "e".to_string().as_bytes()[0];
    let b: bool = c_ordered_choice(&mut po, ((Box::new(c_terminal), char1), (Box::new(c_terminal), char2)));
    println!("{:?} {:?}", b, po.position)
}

#[test]
fn test_c_optional(){
    let mut po = ParserObject{position: 0, source: "Hello there".to_string()};
    //let b: bool = Box::new(c_optional)(&mut po, (Box::new(c_optional), Box::new((c_terminal, 20))));
    //let s= Box::new(c_terminal);
    let b: bool = c_optional(&mut po, (Box::new(c_terminal), 20));
    println!("{:?}", b)
}


#[test]
fn test_c_optional_nested(){
    let mut po = ParserObject{position: 0, source: "Hello there".to_string()};
    //let b: bool = Box::new(c_optional)(&mut po, (Box::new(c_optional), Box::new((c_terminal, 20))));
    //let s= Box::new(c_terminal);
    let b: bool = c_optional(&mut po, (Box::new(c_optional), (Box::new(c_terminal), "h".to_string().as_bytes()[0])));
    println!("{:?} {:?}", b, po.position)
}

fn main(){
    let mut po = ParserObject{position: 0, source: "Hello there".to_string()};
    let bool = num(&mut po);
}