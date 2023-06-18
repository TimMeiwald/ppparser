use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

pub fn get_code_snapshot(str: &str) -> Vec<String> {
    /*Gets all lines of a file from startline to endline*/
    let cwd = current_dir().unwrap();
    let mut after = PathBuf::new();
    after.push(cwd);
    after.push("src");
    after.push(str);
    let contents = fs::read_to_string(after).expect("Failed to read message");
    let content = contents.split("\n").map(str::to_string).collect();
    return content;
}
#[test]
fn test_code_snapshot()->(){
    let s = get_code_snapshot("terminal.rs");
    for i in s{
        println!("{:?}", i);
    }
}

pub fn remove_prefix(arg: Vec<String>) -> Vec<String>{
    let mut new_vec: Vec<String> = Vec::new();
    let mut add = false;
    for i in arg{
        if i == ""{
            add = true;
        }
        if add == false{
            continue;
        }
        else{
            new_vec.push(i);
        }
    }
    return new_vec;
}
pub fn remove_suffix(arg: Vec<String>) -> Vec<String>{
    let mut new_vec: Vec<String> = Vec::new();
    let mut add = true;
    for i in arg{
        if i == "#[cfg(test)]"{
            add = false;
        }
        if add == false{
            continue;
        }
        else{
            new_vec.push(i);
        }
    }
    return new_vec;
}

fn create_full_code_snapshot() -> () {
    let mut code = Vec::new();
    code.push("terminal.rs");
    code.push("and_predicate.rs");
    code.push("not_predicate.rs");
    code.push("optional.rs");
    code.push("subexpression.rs");
    code.push("var_name.rs");
    code.push("zero_or_more.rs");
    code.push("one_or_more.rs");
    let mut ret: Vec<String> = Vec::new();
    for file in code {
        let content = get_code_snapshot(file);
        let content = remove_prefix(content);
        let content = remove_suffix(content);
        for line in content{
            ret.push(line);
            ret.push("\n".to_string());
        }
    }
    let ret = ret.concat();
    println!("{:?}\n", ret);

    let cwd = current_dir().unwrap();
    let mut path = PathBuf::new();
    path.push(cwd);
    path.push("src");
    path.push("generated_parser_core.rs");
    fs::write(path, ret).expect("Unable to write file");
}

#[test]
fn test_get_full_code_snapshot() -> () {
    let s = create_full_code_snapshot();
    println!("{:?}", s);
}




fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/and_predicate.rs");
    println!("cargo:rerun-if-changed=src/not_predicate.rs");
    println!("cargo:rerun-if-changed=src/optional.rs");
    println!("cargo:rerun-if-changed=src/sequence.rs");
    println!("cargo:rerun-if-changed=src/ordered_choice.rs");
    println!("cargo:rerun-if-changed=src/subexpression.rs");
    println!("cargo:rerun-if-changed=src/terminal.rs");
    println!("cargo:rerun-if-changed=src/var_name.rs");
    println!("cargo:rerun-if-changed=src/one_or_more.rs");
    println!("cargo:rerun-if-changed=src/zero_or_more.rs");
    // Use the `cc` crate to build a C file and statically link it.
    create_full_code_snapshot();
}