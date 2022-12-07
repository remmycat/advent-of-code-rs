#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

struct Directory<'i> {
	name: &'i [u8],
	dirs: Vec<Directory<'i>>,
	files_size: usize,
	is_root: bool,
	total_size: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum HaltReason<'i> {
	GoUpAnd(&'i [u8]),
	GoRootAnd(&'i [u8]),
	Done,
}

impl<'i> Directory<'i> {
	fn new(name: &'i [u8]) -> Self {
		Directory {
			name,
			dirs: Default::default(),
			files_size: 0,
			is_root: false,
			total_size: 0,
		}
	}

	fn new_root() -> Self {
		Directory {
			is_root: true,
			..Directory::new(b"/")
		}
	}

	fn add_contents(&mut self, list: &'i [u8]) {
		list.split(|b| *b == b'\n').for_each(|line| {
			if line[0].is_ascii_digit() {
				self.files_size += parse_usize(line);
			} else {
				self.dirs.push(Directory::new(&line[4..]));
			}
		})
	}

	fn execute_in_dir(&mut self, dirname: &'i [u8], commands: &'i [u8]) -> HaltReason<'i> {
		let dir = self
			.dirs
			.iter_mut()
			.find(|dir| dir.name == dirname)
			.expect("bad input, dir doesn't exist");

		dir.execute_commands(commands)
	}

	fn execute_commands(&mut self, commands: &'i [u8]) -> HaltReason<'i> {
		let mut rest = commands;

		while let Some((command, next_rest)) = {
			if rest.is_empty() {
				None
			} else {
				let next_command_pos = rest.iter().skip(1).position(|b| *b == b'$');
				if let Some(pos) = next_command_pos {
					Some(rest.split_at(pos + 1))
				} else {
					// last command
					Some((rest, Default::default()))
				}
			}
		} {
			rest = next_rest;

			match Command::from(command) {
				Command::Ls(Some(list)) => self.add_contents(list),
				Command::Ls(None) => (),
				Command::Cd(b"..") => return HaltReason::GoUpAnd(rest),
				Command::Cd(b"/") if self.is_root => (),
				Command::Cd(b"/") => return HaltReason::GoRootAnd(rest),
				Command::Cd(dirname) => match self.execute_in_dir(dirname, rest) {
					HaltReason::Done => return HaltReason::Done,
					HaltReason::GoUpAnd(new_rest) => rest = new_rest,
					HaltReason::GoRootAnd(new_rest) if self.is_root => rest = new_rest,
					go_root => return go_root,
				},
			}
		}

		HaltReason::Done
	}

	fn update_total_size(&mut self) -> usize {
		let dirs_size = self
			.dirs
			.iter_mut()
			.map(|f| f.update_total_size())
			.sum::<usize>();
		self.total_size = self.files_size + dirs_size;
		self.total_size
	}

	fn get_small_dir_sum(&self) -> usize {
		let own_size = self.total_size;
		let own_size = if own_size > 100_000 { 0 } else { own_size };

		own_size
			+ self
				.dirs
				.iter()
				.map(|d| d.get_small_dir_sum())
				.sum::<usize>()
	}

	fn collect_big_dirs(&self, dir_sizes: &mut Vec<usize>, min_space: usize) {
		if self.total_size >= min_space {
			dir_sizes.push(self.total_size);
		}
		self.dirs
			.iter()
			.for_each(|dir| dir.collect_big_dirs(dir_sizes, min_space));
	}
}

const NUMBERS_START: u8 = b'0';
fn parse_usize(b: &[u8]) -> usize {
	let mut num = (b[0] - NUMBERS_START) as usize;
	for digit in &b[1..] {
		if !digit.is_ascii_digit() {
			return num;
		}
		num = num * 10 + (digit - NUMBERS_START) as usize;
	}
	num
}

enum Command<'i> {
	Cd(&'i [u8]),
	Ls(Option<&'i [u8]>),
}

impl<'i> From<&'i [u8]> for Command<'i> {
	fn from(bytes: &'i [u8]) -> Self {
		match bytes[2] {
			// $ ls\n...
			// 01234567
			b'l' => {
				if bytes.len() == 5 {
					Self::Ls(None)
				} else {
					Self::Ls(Some(&bytes[5..(bytes.len() - 1)]))
				}
			}
			// $ cd ...
			// 01234567
			b'c' => Self::Cd(&bytes[5..(bytes.len() - 1)]),
			_ => panic!("unknown command"),
		}
	}
}

pub fn solve<'i>(input: &'i [u8]) -> Solution {
	let mut root: Directory<'i> = Directory::new_root();

	let halt = root.execute_commands(input);
	assert_eq!(halt, HaltReason::Done);

	root.update_total_size();

	assert!(root.total_size < 70_000_000);
	let available_space = 70_000_000 - root.total_size;

	assert!(available_space < 30_000_000);
	let min_space_needed = 30_000_000 - available_space;

	let mut dir_sizes = vec![];
	root.collect_big_dirs(&mut dir_sizes, min_space_needed);

	let deletion_candidate_size = dir_sizes
		.into_iter()
		.min()
		.expect("bad input - no deletion candidates found");

	Solution(root.get_small_dir_sum(), deletion_candidate_size)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case(include_bytes!("../inputs/example.txt"), Solution(95437,24933642))]
	#[case(include_bytes!("../inputs/personal.txt"), Solution(1581595,1544176))]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
