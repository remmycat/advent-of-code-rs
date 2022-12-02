use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use std::include_str;

const NAME: &str = "day_02";

const EXAMPLE: &str = include_str!("../inputs/example.txt");
const PERSONAL: &str = include_str!("../inputs/personal.txt");

fn criterion_benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group(NAME);

	for input in [EXAMPLE, PERSONAL] {
		let lines = input.lines().count();

		group.throughput(Throughput::Elements(lines as u64));

		group.bench_with_input(BenchmarkId::new("solve", lines), input, |b, file| {
			b.iter(|| aoc_2022_02::solve(black_box(file)))
		});
	}

	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
