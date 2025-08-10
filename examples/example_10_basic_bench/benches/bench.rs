use criterion::Throughput;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use example_5_lr_num_parser::parse;
use std::fs::{canonicalize, read_to_string};

fn create_valid_string_to_parse(size: usize) -> String {
    let mut string = String::with_capacity(size + 2);
    string.push('1');
    for _i in 0..(size / 2) {
        string.push_str("-1");
    }
    return string;
}

pub fn criterion_benchmark_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("example_10_basic_bench");
    for i in [
        1000, 5000, 10000, 20000, 50000, 100000, 200000, 600000, 1000000,
    ] {
        // 600K and above may crash due to lack of stack.
        println!("Size: {i:?}");
        let string = create_valid_string_to_parse(i);
        group.throughput(Throughput::Bytes(i as u64));
        group.bench_function("test pppparser grammar_throughput", |b| {
            b.iter(|| {
                let (result, position, publisher) = parse(black_box(&string));
                assert_eq!(result, true);
                assert_eq!(position as usize, i + 1)
            });
        });
    }
    group.finish();
}

criterion_group!(name = benches; config = Criterion::default().sample_size(500); targets = criterion_benchmark_throughput);
criterion_main!(benches);
