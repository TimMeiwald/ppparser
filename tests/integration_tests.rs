extern crate ppparser;
use crate::ppparser::Resolvable;
use crate::ppparser::Terminal;

#[test]
fn test_terminal_true() {
    let source = "Hello World";
    let position: u32 = 0;
    let t = Terminal {
        arg: "H".to_string().as_bytes()[0],
    };
    let s = t.resolve(position, source);
    println!("{:?} {:?} {:?}", source, s.0, s.1);
    assert_eq!(s.0, true);
    assert_eq!(s.1, 1);
}
