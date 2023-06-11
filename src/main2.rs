#[derive(Debug, PartialEq)]
struct ParserObject {
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
    } else {
        return false;
    }
}

fn Num(&mut self) -> bool {
    let tmp = self.position;
    let bool = c_terminal(self, "0".to_string().as_bytes()[0]);
    if bool {
        return true;
    } else {
        self.position = tmp;
        return false;
    }
}
