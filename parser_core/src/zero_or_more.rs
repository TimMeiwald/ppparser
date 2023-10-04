// use crate::source::Source;
// use crate::context::Context;
// pub fn _zero_or_more<F>(func: F, context: &mut Context, arg: u8) -> bool
//     where F: Fn(&mut Context, u8) -> bool  {
//         loop{
//             let valid = func(context, arg);
//             if valid == false {
//                 break;
//             }
//         }
//         // Always true but may consume zero positions
//         return true;
//     }

// pub fn _one_or_more<F>(func: F, context: &mut Context, arg: u8) -> bool
//     where F: Fn(&mut Context, u8) -> bool  {
//         let valid = func(context, arg);
//         if !valid{
//             return false;
//         }
//         loop{
//             let valid = func(context, arg);
//             if !valid {
//                 break;
//             }
//         }
//         // Always true but may consume zero positions
//         return true;
//     }




// #[cfg(test)]
// mod tests {
//     use crate::source::Source;
//     use crate::terminal::_terminal;
//     use super::_one_or_more;
//     use super::_zero_or_more;

//     use crate::context::Context;
//     #[test]
//     fn test_zero_or_more() {
//         let string = "aaa".to_string();
//         let source = Source::new(string);
//         let mut s = Context{source, position:0};
//         let x = _zero_or_more(_terminal, &mut s, "a".to_string().as_bytes()[0]);
//         println!("{:?}", x);
//         assert_eq!(x, true);
//         assert_eq!(s.position, 3);
//      }
//     #[test]
//     fn test_one_or_more_true(){
//         let string = "a".to_string();
//         let source = Source::new(string);
//         let mut s = Context{source, position:0};
//         let x = _one_or_more(_terminal, &mut s,"a".to_string().as_bytes()[0]);
//         println!("{:?}", x);
//         assert_eq!(x, true);
//         assert_eq!(s.position, 1);
//     }
//     #[test]
//     fn test_one_or_more_multiple(){
//         let string = "aaab".to_string();
//         let source = Source::new(string);
//         let mut s = Context{source, position:0};
//         let x = _one_or_more(_terminal, &mut s, "a".to_string().as_bytes()[0]);
//         println!("{:?}", x);
//         assert_eq!(x, true);
//         assert_eq!(s.position, 3);
//     }
//     #[test]
//     fn test_one_or_more_false(){
//         let string = "".to_string();
//         let source = Source::new(string);
//         let mut s = Context{source, position:0};
//         let x = _one_or_more(_terminal, &mut s, "a".to_string().as_bytes()[0]);
//         println!("{:?}", x);
//         assert_eq!(x, false);
//         assert_eq!(s.position, 0);

//     }
//     #[test]
//     fn test_composition(){
//         let string = "".to_string();
//         let source = Source::new(string);
//         let mut s = Context{source, position:0};

//         let x = _zero_or_more(_one_or_more, &mut s, "a".to_string().as_bytes()[0]);
//         println!("{:?}", x);
//         assert_eq!(x, false);
//         assert_eq!(s.position, 1);
//     }

// }