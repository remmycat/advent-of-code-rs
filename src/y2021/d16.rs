use itertools::Itertools;

pub struct Solution {
	version_sum: usize,
	value: usize,
}

struct BitStream {
	bytes_stack: Vec<u8>,
	rest_bits: Vec<bool>,
}

impl BitStream {
	fn new(bytes: Vec<u8>) -> Self {
		BitStream {
			bytes_stack: bytes.into_iter().rev().collect(),
			rest_bits: Vec::new(),
		}
	}

	fn from_hex(data: &str) -> Self {
		let bytes = data
			.chars()
			.chunks(2)
			.into_iter()
			.map(|mut chunk| {
				let a = chunk.next().unwrap().to_digit(16).unwrap() as u8;
				if let Some(b) = chunk.next() {
					let b = b.to_digit(16).unwrap() as u8;
					(a << 4) | b
				} else {
					// trailing zeros are fine
					a << 4
				}
			})
			.collect();

		BitStream::new(bytes)
	}

	fn len(&self) -> usize {
		self.bytes_stack.len() * 8 + self.rest_bits.len()
	}

	// this could be optimized to take whole bytes directly from the stack if possible for the bit_amt
	fn read(&mut self, bit_amt: u8) -> u32 {
		if bit_amt > 32 {
			panic!("Cannot take more than 32 bits at a time")
		}

		if self.len() < bit_amt.into() {
			panic!("Tried to take more bits than available")
		};

		let mut acc = 0;

		for _ in 0..bit_amt {
			if self.rest_bits.is_empty() {
				let next_byte = self.bytes_stack.pop().unwrap();
				for i in 0..8 {
					// first in, last out
					self.rest_bits.push(next_byte & (1 << i) != 0)
				}
			}

			acc = (acc << 1) | (self.rest_bits.pop().unwrap() as u32)
		}

		acc
	}
}

enum OpKind {
	Sum,
	Product,
	Minimum,
	Maximum,
	GreaterThan,
	LessThan,
	EqualTo,
}

enum PacketContents {
	LiteralValue(u64),
	Operation { kind: OpKind, packets: Vec<Packet> },
}

struct Packet {
	version: u8,
	contents: PacketContents,
}

impl Packet {
	fn from_bits(bits: &mut BitStream) -> Self {
		let version = bits.read(3) as u8;
		let type_id = bits.read(3) as u8;

		if type_id == 4 {
			let mut value = 0;
			loop {
				let last = bits.read(1) == 0;
				value = value << 4 | bits.read(4) as u64;
				if last {
					break;
				}
			}
			return Packet {
				version,
				contents: PacketContents::LiteralValue(value),
			};
		}

		let mut child_packets = Vec::new();

		let length_type_id = bits.read(1);
		if length_type_id == 0 {
			let packets_width = bits.read(15) as usize;
			let bits_left_before = bits.len();
			loop {
				let read_len = bits_left_before - bits.len();
				if read_len > packets_width {
					panic!("Faulty data?")
				}
				if read_len == packets_width {
					break;
				}
				child_packets.push(Packet::from_bits(bits));
			}
		} else {
			let packets_len = bits.read(11) as usize;
			loop {
				if child_packets.len() == packets_len {
					break;
				}
				child_packets.push(Packet::from_bits(bits));
			}
		}

		let op_kind = match type_id {
			0 => OpKind::Sum,
			1 => OpKind::Product,
			2 => OpKind::Minimum,
			3 => OpKind::Maximum,
			// 4 not an op
			5 => OpKind::GreaterThan,
			6 => OpKind::LessThan,
			7 => OpKind::EqualTo,
			_ => panic!("Unknown type_id"),
		};

		Packet {
			version,
			contents: PacketContents::Operation {
				kind: op_kind,
				packets: child_packets,
			},
		}
	}

	fn sum_versions(&self) -> usize {
		let inner_versions = match &self.contents {
			PacketContents::LiteralValue(_) => 0,
			PacketContents::Operation { packets, .. } => {
				packets.iter().map(|p| p.sum_versions()).sum()
			}
		};
		self.version as usize + inner_versions
	}

	fn value(&self) -> usize {
		match &self.contents {
			PacketContents::LiteralValue(value) => *value as usize,
			PacketContents::Operation { kind, packets } => match kind {
				OpKind::Sum => packets.iter().map(|p| p.value()).sum(),
				OpKind::Product => packets.iter().map(|p| p.value()).product(),
				OpKind::Minimum => packets.iter().map(|p| p.value()).min().unwrap(),
				OpKind::Maximum => packets.iter().map(|p| p.value()).max().unwrap(),
				OpKind::GreaterThan => {
					(packets.get(0).unwrap().value() > packets.get(1).unwrap().value()) as usize
				}
				OpKind::LessThan => {
					(packets.get(0).unwrap().value() < packets.get(1).unwrap().value()) as usize
				}
				OpKind::EqualTo => {
					(packets.get(0).unwrap().value() == packets.get(1).unwrap().value()) as usize
				}
			},
		}
	}
}

pub fn solve(input: &str) -> Solution {
	let mut bits = BitStream::from_hex(input);

	let packet = Packet::from_bits(&mut bits);

	Solution {
		version_sum: packet.sum_versions(),
		value: packet.value(),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		assert_eq!(solve("D2FE28").version_sum, 6);
		assert_eq!(solve("38006F45291200").version_sum, 9);
		assert_eq!(solve("EE00D40C823060").version_sum, 14);

		assert_eq!(solve("8A004A801A8002F478").version_sum, 16);
		assert_eq!(solve("620080001611562C8802118E34").version_sum, 12);
		assert_eq!(solve("C0015000016115A2E0802F182340").version_sum, 23);
		assert_eq!(solve("A0016C880162017C3686B18A3D4780").version_sum, 31);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2021/input_16.txt").unwrap();

		assert_eq!(solve(&input).version_sum, 877);
	}

	#[test]
	fn part_2_example_cases() {
		assert_eq!(solve("D2FE28").value, 2021);
		assert_eq!(solve("38006F45291200").value, 1);
		assert_eq!(solve("EE00D40C823060").value, 3);

		assert_eq!(solve("C200B40A82").value, 3);
		assert_eq!(solve("04005AC33890").value, 54);
		assert_eq!(solve("880086C3E88112").value, 7);
		assert_eq!(solve("CE00C43D881120").value, 9);
		assert_eq!(solve("D8005AC2A8F0").value, 1);
		assert_eq!(solve("F600BC2D8F").value, 0);
		assert_eq!(solve("9C005AC2F8F0").value, 0);
		assert_eq!(solve("9C0141080250320F1802104A08").value, 1);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2021/input_16.txt").unwrap();

		assert_eq!(solve(&input).value, 194435634456);
	}
}
