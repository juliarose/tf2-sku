use criterion::{criterion_group, criterion_main, Criterion};
use tf2_sku::SKU;

fn criterion_benchmark(c: &mut Criterion) {
    let sku_str = "16310;15;u703;w2;pk310";
    let key_sku_str = "5021;6";
    let particle_sku_str = "16310;15;u703";
    let sku = SKU::try_from(sku_str).unwrap();
    let key_sku = SKU::try_from(key_sku_str).unwrap();
    let particle_sku = SKU::try_from(particle_sku_str).unwrap();
    
    c.bench_function("parses 16310;15;u703;w2;pk310", |b| b.iter(||
        SKU::try_from(sku_str)
    ));
    
    c.bench_function("parses 5021;6", |b| b.iter(||
        SKU::try_from(key_sku_str)
    ));
    
    c.bench_function("formats 16310;15;u703;w2;pk310", |b| b.iter(||
        sku.to_string()
    ));
    
    c.bench_function("formats 5021;6", |b| b.iter(||
        key_sku.to_string()
    ));
    
    c.bench_function("formats 16310;15;u703", |b| b.iter(||
        particle_sku.to_string()
    ));
    
    c.bench_function("serialize", |b| b.iter(||
        serde_json::to_string(&sku).unwrap()
    ));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
}

criterion_main!(benches);