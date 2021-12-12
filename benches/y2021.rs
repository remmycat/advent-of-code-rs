use std::fs;
use std::time::Duration;

use aoc::y2021::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn y2021(c: &mut Criterion) {
	let mut group = c.benchmark_group("2021");

	let input_01 = fs::read_to_string("assets/2021/input_01.txt").unwrap();
	group.bench_with_input("Day 01: Sonar Sweep", &input_01, |b, i| {
		b.iter(|| d01::solve(i))
	});

	let input_02 = fs::read_to_string("assets/2021/input_02.txt").unwrap();
	group.bench_with_input("Day 02: Dive!", &input_02, |b, i| b.iter(|| d02::solve(i)));

	let input_03 = fs::read_to_string("assets/2021/input_03.txt").unwrap();
	group.bench_with_input("Day 03: Binary Diagnostic", &input_03, |b, i| {
		b.iter(|| d03::solve::<12>(i))
	});

	let input_04 = fs::read_to_string("assets/2021/input_04.txt").unwrap();
	group.bench_with_input("Day 04: Giant Squid", &input_04, |b, i| {
		b.iter(|| d04::solve(i))
	});

	let input_05 = fs::read_to_string("assets/2021/input_05.txt").unwrap();
	group.bench_with_input("Day 05: Hydrothermal Venture", &input_05, |b, i| {
		b.iter(|| d05::solve(i))
	});

	let input_06 = fs::read_to_string("assets/2021/input_06.txt").unwrap();
	group.bench_with_input("Day 06: Lanternfish", &input_06, |b, i| {
		b.iter(|| d06::solve(i))
	});

	let input_07 = fs::read_to_string("assets/2021/input_07.txt").unwrap();
	group.bench_with_input("Day 07: The Treachery of Whales", &input_07, |b, i| {
		b.iter(|| d07::solve(i))
	});

	let input_08 = fs::read_to_string("assets/2021/input_08.txt").unwrap();
	group.bench_with_input("Day 08: Seven Segment Search", &input_08, |b, i| {
		b.iter(|| d08::solve(i))
	});

	let input_09 = fs::read_to_string("assets/2021/input_09.txt").unwrap();
	group.bench_with_input("Day 09: Smoke Basin", &input_09, |b, i| {
		b.iter(|| d09::solve(i))
	});

	let input_10 = fs::read_to_string("assets/2021/input_10.txt").unwrap();
	group.bench_with_input("Day 10: Syntax Scoring", &input_10, |b, i| {
		b.iter(|| d10::solve(i))
	});

	let input_11 = fs::read_to_string("assets/2021/input_11.txt").unwrap();
	group.bench_with_input("Day 11: Dumbo Octopus", &input_11, |b, i| {
		b.iter(|| d11::solve(i))
	});

	let input_12 = fs::read_to_string("assets/2021/input_12.txt").unwrap();
	group.bench_with_input("Day 12: Passage Pathing", &input_12, |b, i| {
		b.iter(|| d12::solve(i))
	});
}

criterion_group! {
	name = benches;
	config = Criterion::default().warm_up_time(Duration::from_millis(100)).sample_size(10);
	targets = y2021
}
criterion_main!(benches);
