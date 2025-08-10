use criterion::Throughput;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use example_10_basic_bench_parser::{
    lr, rr, BasicContext, BasicPublisher, Context, Rules, Source, _var_name,
    _var_name_indirect_left_recursion, RULES_SIZE, Key
};
use std::cell::RefCell;
use std::fs::{canonicalize, read_to_string};


fn create_valid_string_to_parse(size: usize) -> String {
    let mut string = String::with_capacity(size + 1);
    for _i in 0..size {
        string.push_str("1");
    }
    return string;
}

fn parse_lr(source: &str) -> (bool, u32, BasicPublisher) {
    let src_len = source.len() as u32;
    let source = Source::new(source);
    let position: u32 = 0;
    let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
    let result: (bool, u32);
        {
            let involved_set: Vec<Rules> = [Rules::Lr].to_vec();
            let executor =
                _var_name_indirect_left_recursion(&involved_set, Rules::Lr, &context, lr);
            result = executor(Key(0), &source, position);
        }
    let gen_code = context.into_inner().get_publisher().clear_false();
    (result.0, result.1, gen_code)
}

fn parse_rr(source: &str) -> (bool, u32, BasicPublisher) {
    let src_len = source.len() as u32;
    let source = Source::new(source);
    let position: u32 = 0;
    let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
    let result: (bool, u32);
    {
        let executor = _var_name(Rules::Rr, &context, rr);
        result = executor(Key(0), &source, position);
    }
    let gen_code = context.into_inner().get_publisher().clear_false();
    (result.0, result.1, gen_code)
}

pub fn criterion_benchmark_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("example_10_basic_bench");
    for i in [
        1000, 5000, 10000, 20000, 50000, 100000, 200000, 600000, 1000000
    ] {
        // 600K and above may crash due to lack of stack.
        println!("Size: {i:?}");
        let string = create_valid_string_to_parse(i);
        group.throughput(Throughput::Bytes(i as u64));
        group.bench_function("lr throughput", |b| {
            b.iter(|| {
                let (result, position, publisher) = parse_lr(black_box(&string));
                assert_eq!(result, true);
                assert_eq!(position as usize, i)
            });
        });
        group.bench_function("rr throughput", |b| {
            b.iter(|| {
                let (result, position, publisher) = parse_rr(black_box(&string));
                assert_eq!(result, true);
                assert_eq!(position as usize, i)
            });
        });
    }
    group.finish();
}

criterion_group!(name = benches; config = Criterion::default().sample_size(500); targets = criterion_benchmark_throughput);
criterion_main!(benches);
