use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;

const NAME: &str = "day_06";

const EXAMPLE_07: &[u8] = include_bytes!("../inputs/example_7.txt");
const PERSONAL: &[u8] = include_bytes!("../inputs/personal.txt");

fn criterion_benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group(NAME);

	for input in [EXAMPLE_07, PERSONAL] {
		let bytes = input.len();

		group.throughput(Throughput::Elements(bytes as u64));

		group.bench_with_input(BenchmarkId::new("solve", bytes), input, |b, file| {
			b.iter(|| aoc_2022_06::solve(black_box(file)))
		});
	}

	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
