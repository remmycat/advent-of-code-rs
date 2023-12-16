use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use day_10_pipe_maze::{raycast, shoelace_picks};
use std::hint::black_box;

const NAME: &str = env!("CARGO_PKG_NAME");

const INPUTS: [(&str, &[u8]); 6] = [
	("example_1", include_bytes!("../inputs/example_1.txt")),
	("personal", include_bytes!("../inputs/personal.txt")),
	("example_2", include_bytes!("../inputs/example_2.txt")),
	("example_3", include_bytes!("../inputs/example_3.txt")),
	("example_4", include_bytes!("../inputs/example_4.txt")),
	("example_5", include_bytes!("../inputs/example_5.txt")),
];

fn criterion_benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group(NAME);

	for (id, input) in INPUTS {
		let bytes = input.len();

		group.throughput(Throughput::Bytes(bytes as u64));

		group.bench_with_input(BenchmarkId::new("raycast", id), input, |b, file| {
			b.iter(|| raycast::solve(black_box(file)))
		});

		group.bench_with_input(BenchmarkId::new("shoelace_picks", id), input, |b, file| {
			b.iter(|| shoelace_picks::solve(black_box(file)))
		});
	}

	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
