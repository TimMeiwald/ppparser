#[derive(Debug, PartialEq)]
struct ParserObject {
    position: u32,
    source: String,
}


trait Resolvable {
    fn resolve(&self, po: &mut ParserObject) -> bool;
}
struct Call{
    func: fn(&mut ParserObject, dyn Resolvable) -> bool,
    args: dyn Resolvable
}
impl Resolvable for Call{
    fn resolve(&self, po: &mut ParserObject) -> bool {
        let func: fn(&mut ParserObject, dyn Resolvable) -> bool = self.func;
        let resolution: bool = func(po, self.args); 
        return resolution;
    }
}

// enum Core<'a, T: Resolvable, U: Resolvable>{
//     Terminal{po: &'a mut ParserObject, arg: u8},
//     Optional{po: &'a mut ParserObject, arg: T},
//     OrderedChoice{po: &'a mut ParserObject, arg_lhs: T, arg_rhs: U},
// }

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
    } else {
        return false;
    }
}


fn c_optional(po: &mut ParserObject, args: &dyn Resolvable) -> bool {
    /* True if matches, False if not. Increments position on a match */

    // Fn(&u8), u8
    // Fn(&Fn), Fn
    let temp_position = po.position;
    let bool: bool = args.resolve(po);
    if bool == true {
        return true
    }
    else{
        po.position = temp_position;
        return true;
    }
}

fn main(){
    let mut po = ParserObject{
        position: 0, 
        source: "This is a string".to_string(),
    };
    let args: Call<u8> = Call{
        func: c_terminal,
        args: "A".to_string().as_bytes()[0]
    };
    let s = c_optional(&mut po, &args);
    println!("{:?}", s);

}
