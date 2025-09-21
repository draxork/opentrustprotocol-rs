//! Benchmarks for OpenTrust Protocol fusion operations

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use opentrustprotocol::{
    conflict_aware_weighted_average, optimistic_fusion, pessimistic_fusion, NeutrosophicJudgment,
};

fn create_judgment_batch(size: usize) -> Vec<NeutrosophicJudgment> {
    (0..size)
        .map(|i| {
            NeutrosophicJudgment::new(
                0.5 + (i as f64 * 0.001),
                0.3 - (i as f64 * 0.001),
                0.2,
                vec![(format!("source_{}", i), "2023-01-01T00:00:00Z".to_string())],
            )
            .unwrap()
        })
        .collect()
}

fn bench_conflict_aware_weighted_average(c: &mut Criterion) {
    let mut group = c.benchmark_group("conflict_aware_weighted_average");

    for size in [2, 5, 10, 20, 50, 100].iter() {
        let judgments = create_judgment_batch(*size);
        let judgment_refs: Vec<&NeutrosophicJudgment> = judgments.iter().collect();
        let weights: Vec<f64> = (0..*size).map(|i| 1.0 + (i as f64 * 0.1)).collect();

        group.bench_with_input(BenchmarkId::new("cawa", size), size, |b, _| {
            b.iter(|| {
                black_box(
                    conflict_aware_weighted_average(black_box(&judgment_refs), black_box(&weights))
                        .unwrap(),
                )
            });
        });
    }

    group.finish();
}

fn bench_optimistic_fusion(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimistic_fusion");

    for size in [2, 5, 10, 20, 50, 100].iter() {
        let judgments = create_judgment_batch(*size);
        let judgment_refs: Vec<&NeutrosophicJudgment> = judgments.iter().collect();
        
        group.bench_with_input(BenchmarkId::new("optimistic", size), size, |b, _| {
            b.iter(|| black_box(optimistic_fusion(black_box(&judgment_refs)).unwrap()));
        });
    }

    group.finish();
}

fn bench_pessimistic_fusion(c: &mut Criterion) {
    let mut group = c.benchmark_group("pessimistic_fusion");
    
    for size in [2, 5, 10, 20, 50, 100].iter() {
        let judgments = create_judgment_batch(*size);
        let judgment_refs: Vec<&NeutrosophicJudgment> = judgments.iter().collect();
        
        group.bench_with_input(BenchmarkId::new("pessimistic", size), size, |b, _| {
            b.iter(|| {
                black_box(pessimistic_fusion(black_box(&judgment_refs)).unwrap())
            });
        });
    }

    group.finish();
}

fn bench_judgment_creation(c: &mut Criterion) {
    c.bench_function("judgment_creation", |b| {
        b.iter(|| {
            black_box(NeutrosophicJudgment::new(
                black_box(0.8),
                black_box(0.2),
                black_box(0.0),
                black_box(vec![("test".to_string(), "2023-01-01T00:00:00Z".to_string())])
            ).unwrap())
        });
    });
}

fn bench_json_serialization(c: &mut Criterion) {
    let judgment = create_judgment_batch(1).into_iter().next().unwrap();
    
    c.bench_function("json_serialization", |b| {
        b.iter(|| {
            black_box(judgment.to_json().unwrap())
        });
    });
}

fn bench_json_deserialization(c: &mut Criterion) {
    let judgment = create_judgment_batch(1).into_iter().next().unwrap();
    let json = judgment.to_json().unwrap();
    
    c.bench_function("json_deserialization", |b| {
        b.iter(|| {
            black_box(NeutrosophicJudgment::from_json(black_box(&json)).unwrap())
        });
    });
}

criterion_group!(
    benches,
    bench_conflict_aware_weighted_average,
    bench_optimistic_fusion,
    bench_pessimistic_fusion,
    bench_judgment_creation,
    bench_json_serialization,
    bench_json_deserialization
);
criterion_main!(benches);
