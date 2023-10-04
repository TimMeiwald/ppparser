// use crate::context::Context;
// pub fn _var_name<F>(context: &mut Context, func: F) -> bool 
//     where F: Fn(&mut Context, u8) -> bool{
//     if func(context, 97){
//         return true
//     }
//     else{
//         return false
//     }
// }


// #[cfg(test)]
// mod tests {
//     use crate::terminal::_terminal;
//     use crate::source::Source;
//     use crate::context::Context;
//     use crate::var_name::_var_name;
//     #[test]
//     fn test_var_name() {
//         let s = "aaa".to_string();
//         let s = Source::new(s);
//         let mut s = Context{source:s, position:0};
//         let valid = _var_name(&mut s, _terminal);
//         println!("{:?}, {:?}", s.position, valid);
//         assert_eq!(valid, true);
//         assert_eq!(s.position, 1);
//     }   

// }