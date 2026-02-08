mod test;
use std::cell::RefCell;

use test::shared;

use c_parser::{BasicContext, Rules, UserState, function_definition, type_specifier, user_state};

use crate::test::shared_custom_user_state;

#[test]
fn test_1() {
    let src = "struct";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (false, 0));
}

#[test]
fn test_2() {
    let src = "int main(){}";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_3() {
    let src = "int main(int x){return 0;}";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_4() {
    let src = "int main(int x, void y){}";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_5() {
    let src = "int main(int x, void y, myOwnCustomStruct z){}";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_6() {
    let src = "struct customStruct main(int x, void y, myOwnCustomStruct z){}";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}
#[test]
fn test_7() {
    let src = "some_typedefed_type main(int x, void y, myOwnCustomStruct z){}";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_8() {
    let src = "int main (){}";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_9() {
    let src = "int main (){}";
    let result = shared(src, type_specifier::<BasicContext>, Rules::Type_specifier);
    assert_eq!(result, (true, 3));
}


#[test]
fn test_10() {
    let src = "int main (int x, void y){}";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_11() {
    // whatever hasn't been defined so not valid.
    let src = "int main (int x, void y, whatever z){}";
    let result = shared(
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (false, 0));
}

#[test]
fn test_12() {
    // whatever hasn't been defined so we create a user state that has it contained
    // as if it was been inserted by a typedef.
    let src = "int main (int x, void y, whatever z){}";
    let user_state = RefCell::new(UserState::new());
    {
        user_state.borrow_mut().typedef_names.insert("whatever".to_string());
    }
    let result = shared_custom_user_state(
        &user_state,
        src,
        function_definition::<BasicContext>,
        Rules::Function_definition,
    );
    assert_eq!(result, (true, src.len() as u32));
}