use criterion::{criterion_group, criterion_main, Criterion, black_box};
use rustc_hash::FxHashMap;
use ahash::AHashMap;
use std::collections::HashMap;
use static_id::StaticId;
use client::unique_id::UniqueId;

fn bench_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("string");
    let mut fx = FxHashMap::default();
    let mut ahash = AHashMap::default();
    let mut id_map = HashMap::new();
    let mut id_vec = Vec::new();
    // insert 1000 elements
    for i in 0..100 {
        let string = format!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa{}", i);
        id_vec.push(string.clone());
        fx.insert(string.clone(), i);
        ahash.insert(string.clone(), i);
        id_map.insert(string.clone(), i);
    }

    group.bench_function("fx", |b| b.iter(|| {
        for id in id_vec.iter() {
            black_box(fx.get(id));
        }
    }));

    group.bench_function("ahash", |b| b.iter(|| {
        for id in id_vec.iter() {
            black_box(ahash.get(id));
        }
    }));

    group.bench_function("hashmap", |b| b.iter(|| {
        for id in id_vec.iter() {
            black_box(id_map.get(id));
        }
    }));

    group.finish();
}

fn bench_uid(c: &mut Criterion) {
    let mut group = c.benchmark_group("unique_id");
    let mut fx = FxHashMap::default();
    let mut ahash = AHashMap::default();
    let mut id_map = HashMap::new();
    let mut id_vec = Vec::new();
    // insert 1000 elements
    for i in 0..100 {
        let string = format!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa{}", i);
        let id = UniqueId::from_str(&string);
        id_vec.push(id);
        fx.insert(id, i);
        ahash.insert(id, i);
        id_map.insert(id, i);
    }

    group.bench_function("fx", |b| b.iter(|| {
        for id in id_vec.iter() {
            black_box(fx.get(id));
        }
    }));

    group.bench_function("ahash", |b| b.iter(|| {
        for id in id_vec.iter() {
            black_box(ahash.get(id));
        }
    }));

    group.bench_function("hashmap", |b| b.iter(|| {
        for id in id_vec.iter() {
            black_box(id_map.get(id));
        }
    }));

    group.finish();
}

fn bench_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("u64");
    let mut fx = FxHashMap::default();
    let mut ahash = AHashMap::default();
    let mut id_map = HashMap::new();
    // insert 1000 elements
    for i in 0..100 {
        fx.insert(i, i);
        ahash.insert(i, i);
        id_map.insert(i, i);
    }

    group.bench_function("fx", |b| b.iter(|| {
        for i in 0..100 {
            black_box(fx.get(&i));
        }
    }));

    group.bench_function("ahash", |b| b.iter(|| {
        for i in 0..100 {
            black_box(ahash.get(&i));
        }
    }));

    group.bench_function("hashmap", |b| b.iter(|| {
        for i in 0..100 {
            black_box(id_map.get(&i));
        }
    }));

    group.finish();
}

fn bench_staticid(c: &mut Criterion) {
    let mut group = c.benchmark_group("static_id");
    // insert 1000 elements
    let mut fx = FxHashMap::default();
    let mut ahash = AHashMap::default();
    let mut id_vec = Vec::new();
    let mut id_map = HashMap::new();
    for i in 0..100 {
        let string = format!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa{}", i);
        let id = StaticId::from_str(&string, "KRX");
        id_vec.push(id);
        fx.insert(id, i);
        ahash.insert(id, i);
        id_map.insert(id, i);
    }

    group.bench_function("fx", |b| b.iter(|| {
        for id in id_vec.iter() {
            black_box(fx.get(id));
        }
    }));


    group.bench_function("ahash", |b| b.iter(|| {
        for id in id_vec.iter() {
            black_box(ahash.get(id));
        }
    }));

    group.bench_function("hashmap", |b| b.iter(|| {
        for id in id_vec.iter() {
            black_box(id_map.get(id));
        }
    }));

    group.finish();
}

criterion_group!(
    benches, 
    bench_string,
    bench_uid,
    bench_staticid,
    bench_u64,
);
criterion_main!(benches);

