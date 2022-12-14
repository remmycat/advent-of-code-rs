use std::{cmp::Ordering, fmt::Debug, iter::Peekable, slice};

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

#[derive(PartialEq, Eq, Clone)]
enum Packet {
	List(Vec<Packet>),
	Value(u8),
}

impl Packet {
	pub fn from_bytes(bytes: &[u8]) -> Self {
		let bytes = bytes.iter().peekable();

		let (packet, rest) = Packet::from_byte_iter(bytes);
		assert_eq!(rest.len(), 0);

		packet
	}

	fn from_byte_iter(
		mut bytes: Peekable<slice::Iter<'_, u8>>,
	) -> (Packet, Peekable<slice::Iter<'_, u8>>) {
		let start = bytes.next().expect("unexpected end of input");

		match start {
			b'[' => {
				let mut list = vec![];
				loop {
					match bytes.peek().expect("unexpected end of input") {
						b']' => {
							bytes.next();
							return (Self::List(list), bytes);
						}
						b',' => {
							bytes.next();
						}
						_ => {
							let (child, rest) = Packet::from_byte_iter(bytes);
							list.push(child);
							bytes = rest;
						}
					}
				}
			}
			b if b.is_ascii_digit() => {
				let mut num = *b - b'0';
				if matches!(bytes.peek(), Some(d) if d.is_ascii_digit()) {
					num = num * 10 + (bytes.next().unwrap() - b'0');
				}
				(Self::Value(num), bytes)
			}
			_ => panic!("bad input, packet must start with digit or ["),
		}
	}
}

impl Debug for Packet {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::List(arg0) => arg0.fmt(f),
			Self::Value(arg0) => arg0.fmt(f),
		}
	}
}

impl Ord for Packet {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		use Packet::{List, Value};

		match (self, other) {
			(Value(a), Value(b)) => a.cmp(b),
			(List(a), List(b)) => {
				let mut a_iter = a.iter();
				let mut b_iter = b.iter();

				loop {
					match (a_iter.next(), b_iter.next()) {
						(None, None) => return Ordering::Equal,
						(Some(_), None) => return Ordering::Greater,
						(None, Some(_)) => return Ordering::Less,
						(Some(a), Some(b)) => {
							let order = a.cmp(b);
							if !order.is_eq() {
								return order;
							}
						}
					}
				}
			}
			(a @ Value(..), List(b)) => {
				if b.is_empty() {
					return Ordering::Greater;
				}

				let order = a.cmp(&b[0]);

				if order.is_eq() {
					1.cmp(&b.len())
				} else {
					order
				}
			}
			(a @ List(..), b @ Value(..)) => b.cmp(a).reverse(),
		}
	}
}

impl PartialOrd for Packet {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

pub fn solve(input: &[u8]) -> Solution {
	let mut packets = input.split(|b| *b == b'\n');

	let mut correct_count = 0_usize;

	let mut index = 0_usize;

	let mut parsed_packets = vec![];

	while let (Some(packet_a), Some(packet_b), _) = (packets.next(), packets.next(), packets.next())
	{
		index += 1;

		let packet_a = Packet::from_bytes(packet_a);
		let packet_b = Packet::from_bytes(packet_b);

		match packet_a.cmp(&packet_b) {
			Ordering::Equal => panic!("unexpected packet pair of equal order"),
			Ordering::Greater => (),
			Ordering::Less => correct_count += index,
		}

		parsed_packets.push(packet_a);
		parsed_packets.push(packet_b);
	}

	let div_1: Packet = Packet::List(vec![Packet::List(vec![Packet::Value(2)])]);
	let div_2: Packet = Packet::List(vec![Packet::List(vec![Packet::Value(6)])]);

	parsed_packets.push(div_1.clone());
	parsed_packets.push(div_2.clone());

	parsed_packets.sort_unstable();

	let div_1_pos = parsed_packets.iter().position(|p| *p == div_1).unwrap() + 1;
	let div_2_pos = parsed_packets.iter().position(|p| *p == div_2).unwrap() + 1;

	let decoder_key = div_1_pos * div_2_pos;

	Solution(correct_count, decoder_key)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case(include_bytes!("../inputs/example.txt"), Solution(13,140))]
	#[case(include_bytes!("../inputs/personal.txt"), Solution(5003,20280))]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
