use std::io::Empty;

#[derive(Debug)]
#[derive(PartialEq)]
struct ParserObject{
    position: u32,
    source: String,
}


fn main() {
    let something: &str = "This is a string";
}
// def _token(self):
//  if self.position >= len(self.src):
//      return ""
//  return self.src[self.position]
    

fn token(po: &ParserObject) -> Option<u8> {
    if po.position >= po.source.chars().count() as u32 {
        return Option::None;
    }
    let s: u8 = po.source.as_bytes()[po.position as usize];
    return Option::Some(s);
}

#[test]
fn test_token(){
    let po = ParserObject{position: 4, source: "Hello there".to_string()};
    let f: Option<u8> = token(&po);
    let value: u8 = f.unwrap();
    println!("{:?}", value as char);
}




// @cache - Ignore cache for now
// def _TERMINAL(self, arg: str):
//     if arg == self._token():
//         self.position += 1
//         return True
//     else:
//         return False

fn terminal(po: &mut ParserObject, arg: u8) -> bool {
    if arg == token(&po).unwrap() {
        po.position = po.position + 1;
        return true;
    }
    else{
        return false;
    }

}



// def _OPTIONAL(self, args):
//     """True if matches, False if not. Increments position on a match"""
//     func, arg = args
//     temp_position = self.position
//     bool = func(arg)
//     if bool:
//         return True
//     else:
//         self.position = temp_position
//         return False

fn optional_tuple<T>(po: &mut ParserObject, pair: (&dyn Fn(&mut ParserObject, T) -> bool, T))-> bool {
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

fn optional<T>(po: &mut ParserObject, func: &dyn Fn(&mut ParserObject, T) -> bool, arg: T) -> bool{
    // Fn(&u8), u8
    // Fn(&Fn), Fn
    let temp_position = po.position;
    let bool: bool = func(po, arg);
    if bool == true {
        return true
    }
    else{
        po.position = temp_position;
        return true;
    }
}

// def _SEQUENCE(self, args):
//     """True if all expressions match, then updates position, else false, no positional update"""
//     LHS_func, LHS_arg = args[0]
//     RHS_func, RHS_arg = args[1]
//     tmp_pos = self.position
//     bool = LHS_func(LHS_arg)
//     if bool:
//         bool = RHS_func(RHS_arg)
//         if bool:
//             return True
//     self.position = tmp_pos
//     return False

fn sequence_tuple<T, U>(po: &mut ParserObject, lhs: (&dyn Fn(&mut ParserObject, T) -> bool, T), rhs:(&dyn Fn(&mut ParserObject, U) -> bool, U)) -> bool {
    let tmp_pos = po.position;

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

fn sequence<T, U>(po: &mut ParserObject, lhs_func: &dyn Fn(&mut ParserObject, T) -> bool, lhs_arg: T, rhs_func: &dyn Fn(&mut ParserObject, U) -> bool, rhs_arg: U) -> bool {

    let tmp_pos = po.position;
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

// #@cache
//     def _ORDERED_CHOICE(self, args):
//         """True if one expression matches, then updates position, else false, no positional update"""
//         LHS_func, LHS_arg = args[0]
//         RHS_func, RHS_arg = args[1]
//         tmp_pos = self.position
//         bool = LHS_func(LHS_arg)
//         if bool:
//             return True
//         self.position = tmp_pos
//         bool = RHS_func(RHS_arg)
//         if bool:
//             return True
//         self.position = tmp_pos
//         return False


fn ordered_choice_tuple<T, U>(po: &mut ParserObject, lhs: (&dyn Fn(&mut ParserObject, T) -> bool,T), rhs: (&dyn Fn(&mut ParserObject, U) -> bool,U)) -> bool {
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

fn ordered_choice<T, U>(po: &mut ParserObject, lhs_func: &dyn Fn(&mut ParserObject, T) -> bool, lhs_arg: T, rhs_func: &dyn Fn(&mut ParserObject, U) -> bool, rhs_arg: U) -> bool {
    let tmp_pos = po.position;
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

// #@cache
//     def _ZERO_OR_MORE(self, args):
//         """Always True, increments position each time the expression matches else continues without doing anything"""
//         temp_position = self.position
//         func, arg = args
//         while True:
//             bool = func(arg)
//             if bool:
//                 temp_position = self.position
//                 continue
//             else:
//                 self.position = temp_position
//                 break
//         return True

fn zero_or_more_tuple<T: Copy>(po: &mut ParserObject, pair: (&dyn Fn(&mut ParserObject, T) -> bool, T))-> bool {
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

fn zero_or_more<T: Copy>(po: &mut ParserObject, func: &dyn Fn(&mut ParserObject, T) -> bool, arg: T) -> bool {
    let mut temp_position = po.position;
    let mut bool: bool = true;

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

// #@cache
//     def _ONE_OR_MORE(self, args):
//         """True if matches at least once, increments position each time the expression matches"""
//         temp_position = self.position
//         func, arg = args
//         bool = func(arg)
//         if bool:
//             temp_position = self.position
//         else:
//             self.position = temp_position
//             return False
//         while True:
//             bool = func(arg)
//             if bool:
//                 temp_position = self.position
//                 continue
//             else:
//                 self.position = temp_position
//                 break
//         return True

fn one_or_more_tuple<T: Copy>(po: &mut ParserObject, pair: (&dyn Fn(&mut ParserObject, T) -> bool, T)) -> bool {
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

fn one_or_more<T: Copy>(po: &mut ParserObject, func: &dyn Fn(&mut ParserObject, T) -> bool, arg: T) -> bool {
    let mut temp_position = po.position;
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

#[test]
fn test_parser_object(){
    let mut poo: ParserObject = ParserObject{
        position: 20,
        source: "tim is a poo-poo head".to_string(),
    };

    // assert_eq!(poo, ParserObject{
    //     position: 0,
    //     source: "Poo".to_string(),
    // });

    let s = terminal(&mut poo, "d".as_bytes()[0]);
    assert_eq!(true, s);
}

#[test]
fn optional_test() {
    let mut myobj: ParserObject = ParserObject { position: 0, source: "Hello".to_string() };
    let char = "E".as_bytes()[0];
    // let term = &terminal;
    let my_bool: bool = optional(&mut myobj, &terminal, char);
    assert_eq!(true, my_bool);
    assert_eq!(myobj.position, 0)
}


#[test]
fn sequence_test() {
    let mut myobj: ParserObject = ParserObject { position: 0, source: "Hello".to_string() };
    let char1 = "H".as_bytes()[0];
    let char2 = "e".as_bytes()[0];

    let my_bool: bool = sequence(&mut myobj, &terminal, char1, &terminal, char2);
    assert_eq!(true, my_bool);
    assert_eq!(myobj.position, 2)
}


#[test]
fn ordered_choice_test() {
    let mut myobj: ParserObject = ParserObject { position: 0, source: "Hello".to_string() };
    let char1 = "H".as_bytes()[0];
    let char2 = "e".as_bytes()[0];

    let my_bool: bool = ordered_choice(&mut myobj, &terminal, char1, &terminal, char2);
    assert_eq!(true, my_bool);
    assert_eq!(myobj.position, 1)
}

#[test]
fn zero_or_more_test() {
    let mut myobj: ParserObject = ParserObject { position: 0, source: "fHHHHHelllo".to_string() };
    let char1 = "H".as_bytes()[0];

    let my_bool: bool = zero_or_more(&mut myobj, &terminal, char1);
    assert_eq!(true, my_bool);
    assert_eq!(myobj.position, 0)
}

#[test]
fn one_or_more_test() {
    let mut myobj: ParserObject = ParserObject { position: 0, source: "fHHHHHelllo".to_string() };
    let char1 = "H".as_bytes()[0];

    let my_bool: bool = one_or_more(&mut myobj, &terminal, char1);
    assert_eq!(false, my_bool);
    assert_eq!(myobj.position, 0)
}

#[test]
fn more_complex_test(){
    let mut myobj: ParserObject = ParserObject { position: 0, source: "AAABBBC".to_string() };
    let char1 = "A".as_bytes()[0];
    let char2 = "B".as_bytes()[0];

    let my_bool: bool = sequence_tuple(&mut myobj,  (&one_or_more_tuple, (&terminal, char1)),(&one_or_more_tuple, (&terminal, char2)));
    assert_eq!(true, my_bool);
    assert_eq!(myobj.position, 6)
}

