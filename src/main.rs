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

fn terminal(mut po: ParserObject, arg: u8) -> bool {
    if arg == token(&po).unwrap() {
        po.position = po.position + 1;
        return true;
    }
    else{
        return false;
    }
}

#[test]
fn test_parser_object(){
    let poo = ParserObject{
        position: 0,
        source: "Poo".to_string(),
    };

    assert_eq!(poo, ParserObject{
        position: 0,
        source: "Poo".to_string(),
    });

    let s = terminal(poo, "P".as_bytes()[0]);
    assert_eq!(true, s);
}
