use criterion::{criterion_group, criterion_main, Criterion};
use marketplace_sku::SKU;

fn criterion_benchmark(c: &mut Criterion) {
    let sku_str = "16310;15;u703;w2;pk310";
    
    c.bench_function("parses_sku", |b| b.iter(||
        SKU::try_from(sku_str)
    ));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
}

criterion_main!(benches);