use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;

const NAME: &str = "day_15";

const EXAMPLE: &[u8] = include_bytes!("../inputs/example.txt");
const PERSONAL: &[u8] = include_bytes!("../inputs/personal.txt");

fn criterion_benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group(NAME);

	for (input, y_check, search_scope) in [
		(EXAMPLE, 10_isize, 20_isize),
		(PERSONAL, 2_000_000_isize, 4_000_000_isize),
	] {
		let bytes = input.len();

		group.throughput(Throughput::Bytes(bytes as u64));

		group.bench_with_input(BenchmarkId::new("solve", bytes), input, |b, input| {
			b.iter(|| {
				aoc_2022_15::solve(
					black_box(input),
					black_box(y_check),
					black_box(search_scope),
				)
			})
		});
	}

	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
