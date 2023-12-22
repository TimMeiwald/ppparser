use cache::{Cache, DenyLeftRecursionCache, MyCache1, MyCache2, MyCache3, MyCache4};
use grammar_parser::grammar;
use parser_core::{Context, Source};
use stack::{Stack, NoopStack};
use std::any::type_name;
use std::fs::canonicalize;
use std::fs::{read_to_string, write};
use std::time::{Duration, Instant};

fn get_grammar_string() -> String {
    let path = "../parser_core/tests/Grammar.txt";
    let pathbuf = canonicalize(path).expect("If it's moved change the string above");
    let string = read_to_string(pathbuf).expect("If it's moved change the string above");
    return string;
}

fn write_to_performance_profile(data: Vec<String>, path: &str) {
    let pathbuf = canonicalize(path).expect("If it's moved change the string above");
    write(pathbuf, data.concat()).expect("No reason for it to fail")
}

fn run_on_grammar<T: Cache, S: Stack>(n: u32) -> (Duration, String) {
    let src = get_grammar_string();
    let src_len = src.len() as u32;
    let source = Source::new(src);
    let position: u32 = 0;
    let time = Instant::now();

    // Context get's created once because some caches can reuse context and so amortize the initial
    // Memory allocations.
    let context = Context::<T, S>::new(src_len, 42);
    for _i in 0..n {
        //let parse_time = Instant::now();
        let (bol, _position) = grammar(&context, &source, position);
        //println!("Parse time elapsed: {:.2?}", parse_time.elapsed());

        assert!(bol); //-> To test it actually parsed correctly
        assert_eq!(_position, src_len); //
                                        //let cache_time = Instant::now();
        context.clear_cache();
    }
    (time.elapsed(), type_name::<T>().to_string())
}
fn create_performance_string(duration: Duration, cach: String) -> String {
    format!("{:?} {:?}\n", cach, duration)
}

#[allow(unused_mut)]
fn profile_cache_kernel(n_release: u32, n_debug: u32, release_path: &str, debug_path: &str) {
    let mut data = Vec::<String>::new();
    let mut n = n_release;
    let mut path = release_path;
    data.push(format!("RELEASE\nn = {:?}\n", n).to_string());
    #[cfg(debug_assertions)]
    {
        data.clear();
        n = n_debug;
        data.push(format!("DEBUG\nn = {:?}\n", n).to_string());
        path = debug_path;
    }
    // MyCache1
    let res = run_on_grammar::<MyCache1, NoopStack>(n);
    let perf_str = create_performance_string(res.0, res.1);
    data.push(perf_str);

    // MyCache2
    let res = run_on_grammar::<MyCache2, NoopStack>(n);
    let perf_str = create_performance_string(res.0, res.1);
    data.push(perf_str);

    // MyCache3
    let res = run_on_grammar::<MyCache3, NoopStack>(n);
    let perf_str = create_performance_string(res.0, res.1);
    data.push(perf_str);

    // MyCache4
    let res = run_on_grammar::<MyCache4, NoopStack>(n);
    let perf_str = create_performance_string(res.0, res.1);
    data.push(perf_str);

    write_to_performance_profile(data, path)
}

#[test]
fn profile_caches() {
    let release_path = "../grammar_parser/tests/cache_performance_data_release.txt";
    let debug_path = "../grammar_parser/tests/cache_performance_data_debug.txt";

    profile_cache_kernel(1000, 10, release_path, debug_path);
    let release_path = "../grammar_parser/tests/cache_performance_data_release_n1.txt";
    let debug_path = "../grammar_parser/tests/cache_performance_data_debug_n1.txt";
    profile_cache_kernel(1, 1, release_path, debug_path);
}
