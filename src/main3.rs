#[derive(Debug)]
#[derive(PartialEq)]
struct ParserObject{
    position: u32,
    source: String,
}

fn token(po: &ParserObject) -> Option<u8> {
    if po.position >= po.source.chars().count() as u32 {
        return Option::None;
    }
    let s: u8 = po.source.as_bytes()[po.position as usize];
    return Option::Some(s);
}

fn terminal(po: &mut ParserObject, arg: u8) -> bool {
    /* If character at po.position is equal to arg, increment position and return True, else return False */
    if arg == token(&po).unwrap() {
        po.position = po.position + 1;
        return true;
    }
    else{
        return false;
    }
}
enum OrderedChoice{
    OrderedChoiceTerminal{pair: (((fn(&mut ParserObject, u8) -> bool,u8), (fn(&mut ParserObject, u8) -> bool,u8)))},
    OrderedChoiceLeaf{pair: ((fn(&mut ParserObject, u8) -> bool,u8), Box<OrderedChoice>)},
    Optional{pair: (fn(&mut ParserObject, u8) -> bool,u8)},
    Nil,
}

trait Resolve { fn resolve(&self, _: &mut ParserObject) -> bool; }

fn eval_ordered_choice(po: &mut ParserObject, oc: OrderedChoice) -> bool {
    match oc {
        OrderedChoice::OrderedChoiceTerminal{pair}
        => {
            let (lhs, rhs) = pair;
            let (lhs_func, lhs_arg) = lhs;
            let (rhs_func, rhs_arg) = rhs;
            let l = lhs_func(po, lhs_arg);
            let r = rhs_func(po, rhs_arg);
            if(l || r) == true{
                return true
            }
            else{
                return false
            }
        }
        OrderedChoice::OrderedChoiceLeaf{pair} => {
            let (lhs, rhs) = pair;
            let (lhs_func, lhs_arg) = lhs;
            let rhs = *rhs;
            let l = lhs_func(po, lhs_arg);
            let r = eval_ordered_choice(po, rhs);
            if(l || r) == true{
                return true
            }
            else{
                return false
            }
        },
        OrderedChoice::Nil => {true}
    }

}



#[test]

fn test_oc(){
    use crate::OrderedChoice::{OrderedChoiceTerminal, OrderedChoiceLeaf, Nil};
    let char1 = "H".to_string().as_bytes()[0];
    let char2 = "e".to_string().as_bytes()[0];
    let mut po = ParserObject{position: 0, source: "fello there".to_string()};
    let composed_function = OrderedChoiceTerminal{pair:((terminal, char1), (terminal, char2))};
    let f = eval_ordered_choice(&mut po, composed_function);
    println!("{:?}", f)
}

#[test]
fn test_oc2(){
    use crate::OrderedChoice::{OrderedChoiceTerminal, OrderedChoiceLeaf, Nil};
    let char1 = "f".to_string().as_bytes()[0];
    let char2 = "e".to_string().as_bytes()[0];
    let char3 = "H".to_string().as_bytes()[0];
    let mut po = ParserObject{position: 0, source: "Hello there".to_string()};
    let composed_function1 = OrderedChoiceTerminal{pair: ((terminal, char1), (terminal, char2))};
    let composed_function2 = OrderedChoiceLeaf{pair: ((terminal, char3) , Box::new(composed_function1))};
    let f = eval_ordered_choice(&mut po, composed_function2);
    println!("{:?}", f)
}