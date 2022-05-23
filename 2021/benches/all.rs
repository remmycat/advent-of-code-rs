use std::fs;
use std::time::Duration;

use aoc2021::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn y2021(c: &mut Criterion) {
	let mut group = c.benchmark_group("2021");

	let input_01 = fs::read_to_string("assets/01.txt").unwrap();
	group.bench_with_input("Day 01: Sonar Sweep", &input_01, |b, i| {
		b.iter(|| d01::solve(i))
	});

	let input_02 = fs::read_to_string("assets/02.txt").unwrap();
	group.bench_with_input("Day 02: Dive!", &input_02, |b, i| b.iter(|| d02::solve(i)));

	let input_03 = fs::read_to_string("assets/03.txt").unwrap();
	group.bench_with_input("Day 03: Binary Diagnostic", &input_03, |b, i| {
		b.iter(|| d03::solve::<12>(i))
	});

	let input_04 = fs::read_to_string("assets/04.txt").unwrap();
	group.bench_with_input("Day 04: Giant Squid", &input_04, |b, i| {
		b.iter(|| d04::solve(i))
	});

	let input_05 = fs::read_to_string("assets/05.txt").unwrap();
	group.bench_with_input("Day 05: Hydrothermal Venture", &input_05, |b, i| {
		b.iter(|| d05::solve(i))
	});

	let input_06 = fs::read_to_string("assets/06.txt").unwrap();
	group.bench_with_input("Day 06: Lanternfish", &input_06, |b, i| {
		b.iter(|| d06::solve(i))
	});

	let input_07 = fs::read_to_string("assets/07.txt").unwrap();
	group.bench_with_input("Day 07: The Treachery of Whales", &input_07, |b, i| {
		b.iter(|| d07::solve(i))
	});

	let input_08 = fs::read_to_string("assets/08.txt").unwrap();
	group.bench_with_input("Day 08: Seven Segment Search", &input_08, |b, i| {
		b.iter(|| d08::solve(i))
	});

	let input_09 = fs::read_to_string("assets/09.txt").unwrap();
	group.bench_with_input("Day 09: Smoke Basin", &input_09, |b, i| {
		b.iter(|| d09::solve(i))
	});

	let input_10 = fs::read_to_string("assets/10.txt").unwrap();
	group.bench_with_input("Day 10: Syntax Scoring", &input_10, |b, i| {
		b.iter(|| d10::solve(i))
	});

	let input_11 = fs::read_to_string("assets/11.txt").unwrap();
	group.bench_with_input("Day 11: Dumbo Octopus", &input_11, |b, i| {
		b.iter(|| d11::solve(i))
	});

	let input_12 = fs::read_to_string("assets/12.txt").unwrap();
	group.bench_with_input("Day 12: Passage Pathing", &input_12, |b, i| {
		b.iter(|| d12::solve(i))
	});

	let input_13 = fs::read_to_string("assets/13.txt").unwrap();
	group.bench_with_input("Day 13: Transparent Origami", &input_13, |b, i| {
		b.iter(|| d13::solve(i))
	});

	let input_14 = fs::read_to_string("assets/14.txt").unwrap();
	group.bench_with_input("Day 14: Extended Polymerization", &input_14, |b, i| {
		b.iter(|| d14::solve(i))
	});

	let input_15 = fs::read_to_string("assets/15.txt").unwrap();
	group.bench_with_input("Day 15: Chiton", &input_15, |b, i| b.iter(|| d15::solve(i)));

	let input_16 = fs::read_to_string("assets/16.txt").unwrap();
	group.bench_with_input("Day 16: Packet Decoder", &input_16, |b, i| {
		b.iter(|| d16::solve(i))
	});

	let input_17 = fs::read_to_string("assets/17.txt").unwrap();
	group.bench_with_input("Day 17: Trick Shot", &input_17, |b, i| {
		b.iter(|| d17::solve(i))
	});

	let input_18 = fs::read_to_string("assets/18.txt").unwrap();
	group.bench_with_input("Day 18: Snailfish", &input_18, |b, i| {
		b.iter(|| d18::solve(i))
	});

	let input_19 = fs::read_to_string("assets/19.txt").unwrap();
	group.bench_with_input("Day 19: Beacon Scanner", &input_19, |b, i| {
		b.iter(|| d19::solve(i))
	});

	let input_20 = fs::read_to_string("assets/20.txt").unwrap();
	group.bench_with_input("Day 20: Trench Map", &input_20, |b, i| {
		b.iter(|| d20::solve(i))
	});

	let input_21 = fs::read_to_string("assets/21.txt").unwrap();
	group.bench_with_input("Day 21: Dirac Dice", &input_21, |b, i| {
		b.iter(|| d21::solve(i))
	});
}

criterion_group! {
	name = benches;
	config = Criterion::default().warm_up_time(Duration::from_millis(2000)).sample_size(50);
	targets = y2021
}
criterion_main!(benches);
