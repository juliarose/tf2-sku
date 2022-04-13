use criterion::{criterion_group, criterion_main, Criterion};
use tf2_sku::SKU;

fn criterion_benchmark(c: &mut Criterion) {
    let sku_str = "16310;15;u703;w2;pk310";
    let sku = SKU::try_from(sku_str).unwrap();
    
    c.bench_function("parses sku", |b| b.iter(||
        SKU::try_from(sku_str)
    ));
    
    c.bench_function("formats sku to string", |b| b.iter(||
        sku.to_string()
    ));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
}

criterion_main!(benches);