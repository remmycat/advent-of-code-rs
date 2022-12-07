#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

struct Directory<'i> {
	name: &'i str,
	dirs: Vec<Directory<'i>>,
	file_sizes: Vec<usize>,
	is_root: bool,
	total_size: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum HaltReason<'i> {
	GoUpAnd(&'i str),
	GoRootAnd(&'i str),
	Done,
}

impl<'i> Directory<'i> {
	fn new(name: &'i str) -> Self {
		Directory {
			name,
			dirs: Default::default(),
			file_sizes: Default::default(),
			is_root: false,
			total_size: 0,
		}
	}

	fn new_root() -> Self {
		Directory {
			is_root: true,
			..Directory::new("/")
		}
	}

	fn add_contents(&mut self, list: &'i str) {
		for entry in list.lines().map(DirChild::from) {
			match entry {
				DirChild::File(file) => {
					self.file_sizes.push(file);
				}
				DirChild::Dir(directory) => {
					self.dirs.push(directory);
				}
			}
		}
	}

	fn execute_in_dir(&mut self, dirname: &'i str, commands: &'i str) -> HaltReason<'i> {
		let dir = self
			.dirs
			.iter_mut()
			.find(|dir| dir.name == dirname)
			.expect("bad input, dir doesn't exist");

		dir.execute_commands(commands)
	}

	fn execute_commands(&mut self, commands: &'i str) -> HaltReason<'i> {
		let mut rest = if self.is_root {
			commands.split_once("$ ").expect("bad input").1
		} else {
			commands
		};

		while let Some((command, next_rest)) = rest.split_once("$ ").or({
			if rest.is_empty() {
				None
			} else {
				Some((rest, ""))
			}
		}) {
			rest = next_rest;

			match Command::from(command) {
				Command::Ls(Some(list)) => self.add_contents(list),
				Command::Ls(None) => (),
				Command::Cd("..") => return HaltReason::GoUpAnd(rest),
				Command::Cd("/") if self.is_root => (),
				Command::Cd("/") => return HaltReason::GoRootAnd(rest),
				Command::Cd(dirname) => match self.execute_in_dir(dirname, rest) {
					HaltReason::Done => return HaltReason::Done,
					HaltReason::GoUpAnd(new_rest) => rest = new_rest,
					HaltReason::GoRootAnd(new_rest) if self.is_root => rest = new_rest,
					go_root => return go_root,
				},
			}
		}

		// in the last command we can ignore everything that doesn't give us more data
		if !rest.is_empty() {
			if let Command::Ls(Some(list)) = Command::from(rest) {
				self.add_contents(list)
			}
		}

		HaltReason::Done
	}

	fn update_total_size(&mut self) -> usize {
		let files_size = self.file_sizes.iter().sum::<usize>();
		let dirs_size = self
			.dirs
			.iter_mut()
			.map(|f| f.update_total_size())
			.sum::<usize>();
		self.total_size = files_size + dirs_size;
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

enum DirChild<'i> {
	File(usize),
	Dir(Directory<'i>),
}

impl<'i> From<&'i str> for DirChild<'i> {
	fn from(s: &'i str) -> Self {
		let (a, b) = s.split_once(' ').expect("bad input");

		if a == "dir" {
			Self::Dir(Directory::new(b))
		} else {
			let size = a.parse::<usize>().expect("bad file size");
			Self::File(size)
		}
	}
}

#[derive(Debug)]
enum Command<'i> {
	Cd(&'i str),
	Ls(Option<&'i str>),
}

impl<'i> From<&'i str> for Command<'i> {
	fn from(s: &'i str) -> Self {
		match &s[0..2] {
			"ls" => {
				let lines = s.trim().split_once('\n').map(|(_, b)| b);
				Self::Ls(lines)
			}
			"cd" => {
				let (_, to) = s.trim().split_once(' ').expect("missing cd target");
				Self::Cd(to)
			}
			_ => panic!("unknown command"),
		}
	}
}

pub fn solve<'i>(input: &'i str) -> Solution {
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
	#[case(include_str!("../inputs/example.txt"), Solution(95437,24933642))]
	#[case(include_str!("../inputs/personal.txt"), Solution(1581595,0))]
	fn solution(#[case] input: &str, #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
