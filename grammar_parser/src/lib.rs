use parser_core::_terminal;
use parser_core::Source;
use parser_core::_ordered_choice;

fn alphabet_upper(source: &Source, position: u32) -> (bool, u32){
        let t1 = _terminal("A".as_bytes()[0]);
        let t2 = _terminal("B".as_bytes()[0]);
        let oc1 = _ordered_choice(&t1, &t2);
        
        let t1 = _terminal("C".as_bytes()[0]);
        let t2 = _terminal("D".as_bytes()[0]);
        let oc2 = _ordered_choice(&t1, &t2);
        let hoc = _ordered_choice(&oc1, &oc2);

        let t1 = _terminal("E".as_bytes()[0]);
        let t2 = _terminal("F".as_bytes()[0]);
        let oc3 = _ordered_choice(&t1, &t2);
        let hoc = _ordered_choice(&hoc, &oc3);

        let t1 = _terminal("G".as_bytes()[0]);
        let t2 = _terminal("H".as_bytes()[0]);
        let oc4 = _ordered_choice(&t1, &t2);
        let hoc = _ordered_choice(&hoc, &oc4);

        let t1 = _terminal("I".as_bytes()[0]);
        let t2 = _terminal("J".as_bytes()[0]);
        let oc5 = _ordered_choice(&t1, &t2);
        let hoc = _ordered_choice(&hoc, &oc5);

        let t1 = _terminal("K".as_bytes()[0]);
        let t2 = _terminal("L".as_bytes()[0]);
        let oc6 = _ordered_choice(&t1, &t2);
        let hoc = _ordered_choice(&hoc, &oc6);

        let t1 = _terminal("M".as_bytes()[0]);
        let t2 = _terminal("N".as_bytes()[0]);
        let oc7 = _ordered_choice(&t1, &t2);
        let hoc = _ordered_choice(&hoc, &oc7);

        let t1 = _terminal("O".as_bytes()[0]);
        let t2 = _terminal("P".as_bytes()[0]);
        let oc8 = _ordered_choice(&t1, &t2);
        let hoc = _ordered_choice(&hoc, &oc8);

        let t1 = _terminal("Q".as_bytes()[0]);
        let t2 = _terminal("R".as_bytes()[0]);
        let oc9 = _ordered_choice(&t1, &t2);
        let hoc = _ordered_choice(&hoc, &oc9);

        let t1 = _terminal("S".as_bytes()[0]);
        let t2 = _terminal("T".as_bytes()[0]);
        let oc10 = _ordered_choice(&t1, &t2);
        let hoc = _ordered_choice(&hoc, &oc10);

        let t1 = _terminal("U".as_bytes()[0]);
        let t2 = _terminal("V".as_bytes()[0]);
        let oc11 = _ordered_choice(&t1, &t2);
        let hoc = _ordered_choice(&hoc, &oc11);

        let t1 = _terminal("W".as_bytes()[0]);
        let t2 = _terminal("X".as_bytes()[0]);
        let oc12 = _ordered_choice(&t1, &t2);
        let hoc = _ordered_choice(&hoc, &oc12);

        let t1 = _terminal("Y".as_bytes()[0]);
        let t2 = _terminal("Z".as_bytes()[0]);
        let oc13 = _ordered_choice(&t1, &t2);
        let hoc = _ordered_choice(&hoc, &oc13);
        
        hoc(source, position)

    }


    
#[cfg(test)]
mod tests {
    use parser_core::Source;
    use super::*;
    #[test]
    fn test_alphabet_upper_false() {
        let string = "aaa".to_string();
        let source = Source::new(string);
        let position: u32 = 0;
        let result = alphabet_upper(&source, position);
        assert_eq!(result, (false, 0));
    }
    #[test]
    fn test_alphabet_upper_true() {
        let string = "AAA".to_string();
        let source = Source::new(string);
        let position: u32 = 0;
        let result = alphabet_upper(&source, position);
        assert_eq!(result, (true, 1));
    }
}
