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

fn terminal(po: &mut ParserObject, arg: u8) -> bool {
    if arg == token(&po).unwrap() {
        po.position = po.position + 1;
        return true;
    }
    else{
        return false;
    }

}

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


#[test]
fn more_complex_test(){
    let mut myobj: ParserObject = ParserObject { position: 0, source: "AAABBBC".to_string() };
    let char1 = "A".as_bytes()[0];
    let char2 = "B".as_bytes()[0];

    let my_bool: bool = sequence_tuple(&mut myobj,  (&one_or_more_tuple, (&terminal, char1)),(&one_or_more_tuple, (&terminal, char2)));
    assert_eq!(true, my_bool);
    assert_eq!(myobj.position, 6)
}
