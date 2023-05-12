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
