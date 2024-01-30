use std::io::Read;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("read 65k");
    let input = std::fs::File::open("files.json").unwrap();
    let files = serde_json::from_reader::<_, Vec<String>>(input).unwrap();
    for id in files {
        group.throughput(criterion::Throughput::Bytes(65536));
        group.bench_with_input(BenchmarkId::new("read", &id), &id, |b, id| {
            b.iter(|| vfs_stress_test::bench_read(id))
        });
    }
    group.finish();
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
