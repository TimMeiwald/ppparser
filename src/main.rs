
fn token(position: u32, source: &str) -> Option<u8> {
    if position >= source.chars().count() as u32 {
        return Option::None;
    }
    let s: u8 = source.as_bytes()[position as usize];
    return Option::Some(s);
}

fn terminal(position: u32, source: &str, arg: u8) -> (bool, u32) {
    /* If character at po.position is equal to arg, increment position and return True, else return False */
    if arg == token(position, source).unwrap() {
        let position = position + 1;
        return (true, position);
    }
    else{
        return (false, position);
    }
}


trait Resolvable{
    fn resolve(&self, position: u32, source: &str) -> (bool, u32);
}

#[derive(Clone)]
#[derive(Copy)]
struct Terminal{
    arg: u8
}

impl Resolvable for Terminal{
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return terminal(position, source, self.arg)
    }
}

#[test]
fn test_terminal_true(){
    let source = "Hello World";
    let position: u32 = 0;
    let t = Terminal{arg: "H".to_string().as_bytes()[0]};
    let s = t.resolve(position, source);
    println!("{:?} {:?} {:?}", source, s.0, s.1);
    assert_eq!(s.0, true);
    assert_eq!(s.1, 1);
}

#[test]
fn test_terminal_false(){
    let source = "Hello World";
    let position: u32 = 0;
    let t = Terminal{arg: "h".to_string().as_bytes()[0]};
    let s = t.resolve(position, source);
    println!("{:?} {:?} {:?}", source, s.0, s.1);
    assert_eq!(s.0, false);
    assert_eq!(s.1, 0);
}


fn optional<T: Resolvable>(position: u32, source: &str, args: T)-> (bool, u32) {
    /* True if matches, False if not. Increments position on a match */

    // Fn(&u8), u8
    // Fn(&Fn), Fn
    let temp_position = position;
    let (bool, position) = args.resolve(position, source);

    if bool == true {
        return (true, position);
    }
    else{
        let position = temp_position;
        return (true, position);
    }
}

#[derive(Clone)]
#[derive(Copy)]
struct Optional<T: Resolvable>{
    arg: T,
}

impl <T: Resolvable + Copy> Resolvable for Optional<T>{
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return optional(position, source, self.arg);
    }
}

#[test]
fn test_optional_no_increment(){
    let source = "Hello World";
    let position: u32 = 0;
    let t = Terminal{arg: "f".to_string().as_bytes()[0]};
    let t2 = Optional{arg: t};
    let s = t2.resolve(position, source);
    println!("{:?} {:?} {:?}", source, s.0, s.1);
    assert_eq!(s.0, true);
    assert_eq!(s.1, 0);
}

#[test]
fn test_optional_increment(){
    let source = "Hello World";
    let position: u32 = 0;
    let t = Terminal{arg: "H".to_string().as_bytes()[0]};
    let t2 = Optional{arg: t};
    let s = t2.resolve(position, source);
    println!("{:?} {:?} {:?}", source, s.0, s.1);
    assert_eq!(s.0, true);
    assert_eq!(s.1, 1);
}

#[test]
fn test_optional_nested(){
    let source = "Hello World";
    let position: u32 = 0;
    let t = Terminal{arg: "f".to_string().as_bytes()[0]};
    let t2 = Optional{arg: t};
    let t3 = Optional{arg: t2};
    let s = t3.resolve(position, source);
    println!("{:?} {:?} {:?}", source, s.0, s.1);
    assert_eq!(s.0, true);
    assert_eq!(s.1, 0);
}


fn ordered_choice<T: Resolvable, U: Resolvable>(position: u32, source: &str, arg_lhs: T, arg_rhs: U) -> (bool, u32){
    /* True if one expression matches, then updates position, else false, no positional update */

    let tmp_pos = position;
    let (lhs_bool, position)= arg_lhs.resolve(position, source);
    if lhs_bool {
        return (true, position);
    }
    let position = tmp_pos;

    let (rhs_bool, position) = arg_rhs.resolve(position, source);
    if rhs_bool {
        return (true, position);
    }
    let position = tmp_pos;

    return (false, position);
}

#[derive(Clone)]
#[derive(Copy)]
struct OrderedChoice<T: Resolvable, U: Resolvable>{
    arg_lhs: T,
    arg_rhs: U,
}

impl <T: Resolvable + Copy, U: Resolvable + Copy> Resolvable for OrderedChoice<T, U>{
    fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
        return ordered_choice(position, source, self.arg_lhs, self.arg_rhs);
    }
}

#[test]
fn test_ordered_choice_true(){
    let source = "Hello World";
    let position: u32 = 0;
    let t = Terminal{arg: "f".to_string().as_bytes()[0]};
    let t2 = Terminal{arg: "H".to_string().as_bytes()[0]};
    let t3 = OrderedChoice{arg_lhs: t, arg_rhs: t2};
    let s = t3.resolve(position, source);
    println!("{:?} {:?} {:?}", source, s.0, s.1);
    assert_eq!(s.0, true);
    assert_eq!(s.1, 1);

}


#[test]
fn test_ordered_choice_false(){
    let source = "Hello World";
    let position: u32 = 0;
    let t = Terminal{arg: "f".to_string().as_bytes()[0]};
    let t2 = Terminal{arg: "e".to_string().as_bytes()[0]};
    let t3 = OrderedChoice{arg_lhs: t, arg_rhs: t2};
    let s = t3.resolve(position, source);
    println!("{:?} {:?} {:?}", source, s.0, s.1);
    assert_eq!(s.0, false);
    assert_eq!(s.1, 0);

}

#[test]
fn test_ordered_choice_nested(){
    let source = "Hello World";
    let position: u32 = 0;
    let t = Terminal{arg: "f".to_string().as_bytes()[0]};
    let t2 = Terminal{arg: "e".to_string().as_bytes()[0]};
    let t3 = Terminal{arg: "H".to_string().as_bytes()[0]};
    let t4 = Terminal{arg: "n".to_string().as_bytes()[0]};
    let t5 = OrderedChoice{arg_lhs: t, arg_rhs: t2};
    let t6 = OrderedChoice{arg_lhs: t3, arg_rhs: t4};
    let t7 = OrderedChoice{arg_lhs: t5, arg_rhs: t6};
    let s = t7.resolve(position, source);
    println!("{:?} {:?} {:?}", source, s.0, s.1)
}

#[test]
fn test_ordered_choice_deep_nested(){
    /* The purpose of this test is merely to see if it chokes on deep nesting */
    let source = "Hello World";
    let position: u32 = 0;
    let t = Terminal{arg: "f".to_string().as_bytes()[0]};
    let t2 = Terminal{arg: "e".to_string().as_bytes()[0]};
    let t3 = Terminal{arg: "h".to_string().as_bytes()[0]};
    let t4 = Terminal{arg: "n".to_string().as_bytes()[0]};
    let t5 = OrderedChoice{arg_lhs: t, arg_rhs: t2};
    let t6 = OrderedChoice{arg_lhs: t3, arg_rhs: t4};
    let t7 = OrderedChoice{arg_lhs: t5, arg_rhs: t6};
    let t8 = OrderedChoice{arg_lhs: t6, arg_rhs: t7};
    let t9 = OrderedChoice{arg_lhs: t7, arg_rhs: t8};
    let t10 = OrderedChoice{arg_lhs: t8, arg_rhs: t9};
    let t11 = OrderedChoice{arg_lhs: t9, arg_rhs: t10};
    let t12 = OrderedChoice{arg_lhs: t10, arg_rhs: t11};
    let t13 = OrderedChoice{arg_lhs: t11, arg_rhs: t12};
    let t14 = OrderedChoice{arg_lhs: t12, arg_rhs: t13};
    let t15 = OrderedChoice{arg_lhs: t13, arg_rhs: t14};
    let t16 = OrderedChoice{arg_lhs: t14, arg_rhs: t15};
    let t17 = OrderedChoice{arg_lhs: t15, arg_rhs: t16};
    let t18 = OrderedChoice{arg_lhs: t16, arg_rhs: t17};
    let t19 = OrderedChoice{arg_lhs: t17, arg_rhs: t18};
    let t20 = OrderedChoice{arg_lhs: t18, arg_rhs: t19};
    let t21 = OrderedChoice{arg_lhs: t19, arg_rhs: t20};
    let t22 = OrderedChoice{arg_lhs: t20, arg_rhs: t21};
    let t23 = OrderedChoice{arg_lhs: t21, arg_rhs: t22};
    let t24 = OrderedChoice{arg_lhs: t22, arg_rhs: t23};
    let t25 = OrderedChoice{arg_lhs: t23, arg_rhs: t24};
    let s = t25.resolve(position, source);
    println!("{:?} {:?} {:?}", source, s.0, s.1)
}


fn sequence<T: Resolvable, U: Resolvable>(position: u32, source: &str, arg_lhs: T, arg_rhs: U) -> (bool, u32) {
    //fn c_sequence<T: Copy, U: Copy>(po: &mut ParserObject, lhs: (&dyn Fn(&mut ParserObject, T) -> bool, T), rhs:(&dyn Fn(&mut ParserObject, U) -> bool, U)) -> bool {
        /* True if all expressions match, then updates position, else false, no positional update */
    
        let tmp_pos = position;
    
    
        let (lhs_bool, position) = arg_lhs.resolve(position, source);
        let (rhs_bool, position) = arg_rhs.resolve(position, source);
    
        if lhs_bool && rhs_bool {
            return (true, position);
        }
        else {
            let position = tmp_pos;
            return (false, position);
        }
    }
    


    #[derive(Clone)]
    #[derive(Copy)]
    struct Sequence<T: Resolvable, U: Resolvable>{
        arg_lhs: T,
        arg_rhs: U,
    }
    
    impl <T: Resolvable + Copy, U: Resolvable + Copy> Resolvable for Sequence<T, U>{
        fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
            return sequence(position, source, self.arg_lhs, self.arg_rhs);
        }
    }


    #[test]
    fn test_sequence_true(){
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal{arg: "H".to_string().as_bytes()[0]};
        let t2 = Terminal{arg: "e".to_string().as_bytes()[0]};
        let t3 = Sequence{arg_lhs: t, arg_rhs: t2};
        let s = t3.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 2);
    
    }
    
    
    #[test]
    fn test_sequence_false(){
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal{arg: "H".to_string().as_bytes()[0]};
        let t2 = Terminal{arg: "f".to_string().as_bytes()[0]};
        let t3 = Sequence{arg_lhs: t, arg_rhs: t2};
        let s = t3.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    
    }
    
    #[test]
    fn test_sequence_nested(){
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal{arg: "H".to_string().as_bytes()[0]};
        let t2 = Terminal{arg: "e".to_string().as_bytes()[0]};
        let t3 = Terminal{arg: "l".to_string().as_bytes()[0]};
        let t4 = Terminal{arg: "l".to_string().as_bytes()[0]};
        let t5 = Sequence{arg_lhs: t, arg_rhs: t2};
        let t6 = Sequence{arg_lhs: t3, arg_rhs: t4};
        let t7 = Sequence{arg_lhs: t5, arg_rhs: t6};
        let s = t7.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 4);
    }
    
    #[test]
    fn test_mixed_sequence_ordered_choice(){
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal{arg: "f".to_string().as_bytes()[0]};
        let t2 = Terminal{arg: "i".to_string().as_bytes()[0]};
        let t3 = Terminal{arg: "s".to_string().as_bytes()[0]};
        let t4 = Terminal{arg: "s".to_string().as_bytes()[0]};
        let t5 = Sequence{arg_lhs: t, arg_rhs: t2};
        let t6 = Sequence{arg_lhs: t3, arg_rhs: t4};
        let t7 = Sequence{arg_lhs: t5, arg_rhs: t6};

        let r = Terminal{arg: "H".to_string().as_bytes()[0]};
        let r2 = Terminal{arg: "e".to_string().as_bytes()[0]};
        let r3 = Terminal{arg: "l".to_string().as_bytes()[0]};
        let r4 = Terminal{arg: "l".to_string().as_bytes()[0]};
        let r5 = Sequence{arg_lhs: r, arg_rhs: r2};
        let r6 = Sequence{arg_lhs: r3, arg_rhs: r4};
        let r7 = Sequence{arg_lhs: r5, arg_rhs: r6};
        let t8  =  OrderedChoice{arg_lhs: t7, arg_rhs: r7};
        let s = t8.resolve(position, source);   
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 4);
    }
    

fn zero_or_more<T: Resolvable>(position: u32, source: &str, arg: T)-> (bool, u32) {
        /* Always True, increments position each time the expression matches else continues without doing anything */
    
        let mut temp_position = position;
        let mut bool;
        let mut position = position;
        loop {
            let ret = arg.resolve(position, source);
            bool = ret.0;
            position = ret.1;
            if bool {
                temp_position = position;
                continue;
            }
            else {
                position = temp_position;
                break;
            }
        }
        return (true, position);
    }

    #[derive(Clone)]
    #[derive(Copy)]
    struct ZeroOrMore<T: Resolvable>{
        arg: T,
    }
    
    impl <T: Resolvable + Copy> Resolvable for ZeroOrMore<T>{
        fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
            return zero_or_more(position, source, self.arg);
        }
    }
    



    #[test]
    fn test_zero_or_more_1(){
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal{arg: "H".to_string().as_bytes()[0]};
        let t3 = ZeroOrMore{arg: t};
        let s = t3.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 1);
    
    }
    
    #[test]
    fn test_zero_or_more_10(){
        let source = "HHHHHHHHHHello World";
        let position: u32 = 0;
        let t = Terminal{arg: "H".to_string().as_bytes()[0]};
        let t3 = ZeroOrMore{arg: t};
        let s = t3.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 10);
    
    
    }

    #[test]
    fn test_zero_or_more_0(){
        let source = "fello World";
        let position: u32 = 0;
        let t = Terminal{arg: "H".to_string().as_bytes()[0]};
        let t3 = ZeroOrMore{arg: t};
        let s = t3.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 0);
    
    }
    
    
    
    fn one_or_more<T: Resolvable>(position: u32, source: &str, arg: T)-> (bool, u32) {
        /* Always True, increments position each time the expression matches else continues without doing anything */
    
        let mut temp_position = position;
        let (mut bool, mut position)= arg.resolve(position, source);
        if bool {
            temp_position = position;
        } else {
            position = temp_position;
            return (false, position);
        }
        loop {
            let ret = arg.resolve(position, source);
            bool = ret.0;
            position = ret.1;
            if bool {
                temp_position = position;
                continue;
            }
            else {
                position = temp_position;
                break;
            }
        }
        return (true, position);
    }

    #[derive(Clone)]
    #[derive(Copy)]
    struct OneOrMore<T: Resolvable>{
        arg: T,
    }
    
    impl <T: Resolvable + Copy> Resolvable for OneOrMore<T>{
        fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
            return one_or_more(position, source, self.arg);
        }
    }
    



    #[test]
    fn test_one_or_more_1(){
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal{arg: "H".to_string().as_bytes()[0]};
        let t3 = OneOrMore{arg: t};
        let s = t3.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 1);
    
    }
    
    #[test]
    fn test_one_or_more_10(){
        let source = "HHHHHHHHHHello World";
        let position: u32 = 0;
        let t = Terminal{arg: "H".to_string().as_bytes()[0]};
        let t3 = OneOrMore{arg: t};
        let s = t3.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 10);
    
    
    }

    #[test]
    fn test_one_or_more_0(){
        let source = "fello World";
        let position: u32 = 0;
        let t = Terminal{arg: "H".to_string().as_bytes()[0]};
        let t3 = OneOrMore{arg: t};
        let s = t3.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    
    }
    
    

    
    fn and_predicate<T: Resolvable>(position: u32, source: &str, arg: T)-> (bool, u32) {
        /* Always True, increments position each time the expression matches else continues without doing anything */
    
        let temp_position = position;
        let ret = arg.resolve(position, source);
        let bool = ret.0;
        if bool {
            return (true, temp_position);
        } else {
            return (false, temp_position);
        }
    }

    #[derive(Clone)]
    #[derive(Copy)]
    struct AndPredicate<T: Resolvable>{
        arg: T,
    }
    
    impl <T: Resolvable + Copy> Resolvable for AndPredicate<T>{
        fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
            return and_predicate(position, source, self.arg);
        }
    }
    
    #[test]
    fn test_and_predicate_no_increment(){
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal{arg: "f".to_string().as_bytes()[0]};
        let t2 = AndPredicate{arg: t};
        let s = t2.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }

    #[test]
    fn test_and_predicate_increment(){
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal{arg: "H".to_string().as_bytes()[0]};
        let t2 = AndPredicate{arg: t};
        let s = t2.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 0);
    }
        



    fn not_predicate<T: Resolvable>(position: u32, source: &str, arg: T)-> (bool, u32) {
        /* Always True, increments position each time the expression matches else continues without doing anything */
    
        let ret = and_predicate(position, source, arg);
        return (!ret.0, ret.1)
    }

    #[derive(Clone)]
    #[derive(Copy)]
    struct NotPredicate<T: Resolvable>{
        arg: T,
    }
    
    impl <T: Resolvable + Copy> Resolvable for NotPredicate<T>{
        fn resolve(&self, position: u32, source: &str) -> (bool, u32) {
            return not_predicate(position, source, self.arg);
        }
    }
    
    
    #[test]
    fn test_not_predicate_no_increment(){
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal{arg: "f".to_string().as_bytes()[0]};
        let t2 = NotPredicate{arg: t};
        let s = t2.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, true);
        assert_eq!(s.1, 0);
    }

    #[test]
    fn test_not_predicate_increment(){
        let source = "Hello World";
        let position: u32 = 0;
        let t = Terminal{arg: "H".to_string().as_bytes()[0]};
        let t2 = NotPredicate{arg: t};
        let s = t2.resolve(position, source);
        println!("{:?} {:?} {:?}", source, s.0, s.1);
        assert_eq!(s.0, false);
        assert_eq!(s.1, 0);
    }
        








































fn main(){
    fn readinput() -> String {
        use std::io::stdin;
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        input
    }
    let s = readinput();
    let source =  &s[..];
    
    
    let position: u32 = 0;
    let t = Terminal{arg: "H".to_string().as_bytes()[0]};
    let t2 = Terminal{arg: "e".to_string().as_bytes()[0]};
    let t3 = Terminal{arg: "l".to_string().as_bytes()[0]};
    let t4 = Terminal{arg: "l".to_string().as_bytes()[0]};
    let t5 = Sequence{arg_lhs: t, arg_rhs: t2};
    let t6 = Sequence{arg_lhs: t3, arg_rhs: t4};
    let t7 = Sequence{arg_lhs: t5, arg_rhs: t6};
    let s = t7.resolve(position, source);
    println!("{:?} {:?} {:?}", source, s.0, s.1);
    assert_eq!(s.0, true);
    assert_eq!(s.1, 4);


}   