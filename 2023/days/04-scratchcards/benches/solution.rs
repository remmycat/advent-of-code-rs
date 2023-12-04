use aoc_2023_04::solve;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;

const NAME: &str = env!("CARGO_PKG_NAME");

const INPUTS: [(&str, &[u8]); 2] = [
	("example", include_bytes!("../inputs/example.txt")),
	("personal", include_bytes!("../inputs/personal.txt")),
];

fn criterion_benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group(NAME);

	for (id, input) in INPUTS {
		let bytes = input.len();

		group.throughput(Throughput::Bytes(bytes as u64));

		group.bench_with_input(BenchmarkId::new("solve", id), input, |b, file| {
			b.iter(|| solve(black_box(file)))
		});
	}

	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
