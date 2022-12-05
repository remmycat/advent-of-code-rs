use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;

const NAME: &str = "day_05";

const EXAMPLE: &str = include_str!("../inputs/example.txt");
const PERSONAL: &str = include_str!("../inputs/personal.txt");

fn criterion_benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group(NAME);

	for input in [EXAMPLE, PERSONAL] {
		let bytes = input.len();

		group.throughput(Throughput::Bytes(bytes as u64));

		group.bench_with_input(BenchmarkId::new("solve_string", bytes), input, |b, file| {
			b.iter(|| aoc_2022_05::solve_string(black_box(file)))
		});

		group.bench_with_input(BenchmarkId::new("solve_bytes", bytes), input, |b, file| {
			let byte_input = file.as_bytes();
			b.iter(|| aoc_2022_05::solve_bytes(black_box(byte_input)))
		});
	}

	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
