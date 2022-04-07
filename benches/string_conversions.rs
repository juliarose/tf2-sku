use criterion::{criterion_group, criterion_main, Criterion};
use marketplace_sku::MarketplaceSKU;

fn criterion_benchmark(c: &mut Criterion) {
    let australium = "16310;15;u703;w2;pk310";
    
    c.bench_function("australium", |b| b.iter(||
        MarketplaceSKU::try_from(australium)
    ));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
}

criterion_main!(benches);