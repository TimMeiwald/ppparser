/* Example: passing function as argument
fn string(accept_string: &str) {
    println!("This is a string: {}", accept_string);
}

fn accept_function(function: &dyn Fn(&str)){
    let input_string: &str = "Poo";
    function(input_string);
} 

#[test]
fn test_accept_function(){
    let poo = accept_function(&string);
    assert_eq!(poo, ());
} */
