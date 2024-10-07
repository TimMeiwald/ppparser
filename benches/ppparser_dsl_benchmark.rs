use core::cell::RefCell;
use criterion::Throughput;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ppparser::parse;
use std::env;
use std::fs::{canonicalize, read_to_string};

fn test_grammar() {
    // println!("{:?}", env::current_dir().unwrap());
    let path = "tests/test_data/Grammar.txt";
    let pathbuf = canonicalize(path).expect("If it's moved change the string above");
    let string = read_to_string(pathbuf).expect("If it's moved change the string above");
    let src_len = string.len();
    let result = ppparser::parse(&string);
    assert_eq!((result.0, result.1), (true, src_len as u32));
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test pppparser grammar", |b| b.iter(|| test_grammar()));
}
pub fn criterion_benchmark_throughput(c: &mut Criterion) {
    let path = "tests/test_data/Grammar.txt";
    let pathbuf = canonicalize(path).expect("If it's moved change the string above");
    let string = read_to_string(pathbuf).expect("If it's moved change the string above");
    let src_len = string.len();
    let mut group = c.benchmark_group("throughput-example");
    group.throughput(Throughput::Bytes(src_len as u64));
    group.bench_function("test pppparser grammar_throughput", |b| {
        b.iter(|| ppparser::parse(black_box(&string)))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark_throughput);
criterion_main!(benches);
