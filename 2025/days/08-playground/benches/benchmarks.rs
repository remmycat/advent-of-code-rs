use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use day_08_playground::solve;
use std::hint::black_box;

const NAME: &str = env!("CARGO_PKG_NAME");

const INPUTS: [(&str, usize, &[u8]); 2] = [
	("example", 10, include_bytes!("../inputs/example.txt")),
	("personal", 1000, include_bytes!("../inputs/personal.txt")),
];

fn criterion_benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group(NAME);

	for (id, first_n, input) in INPUTS {
		let bytes = input.len();

		group.throughput(Throughput::Bytes(bytes as u64));

		group.bench_with_input(BenchmarkId::new("solve", id), input, |b, file| {
			b.iter(|| solve(black_box(file), first_n))
		});
	}

	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
