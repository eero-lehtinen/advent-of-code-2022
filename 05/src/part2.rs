use std::str::FromStr;

struct Instruction {
	count: usize,
	from: usize,
	to: usize,
}

impl FromStr for Instruction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts = s.split_whitespace().collect::<Vec<_>>();
		let count = parts[1].parse().unwrap();
		let from = parts[3].parse::<usize>().unwrap() - 1;
		let to = parts[5].parse::<usize>().unwrap() - 1;
		Ok(Self { count, from, to })
	}
}

fn main() {
	let input = include_str!("../input.txt");

	let (stacks, instructions) = input.split_once("\n\n").unwrap();

	let mut stacks = {
		let lines = stacks
			.lines()
			.map(str::as_bytes)
			.rev()
			.skip(1)
			.collect::<Vec<_>>();

		let mut stacks = Vec::new();
		for i in (1..lines.first().unwrap().len() - 1).step_by(4) {
			let mut stack = Vec::new();
			for line in &lines {
				if line[i] == b' ' {
					break;
				}
				stack.push(line[i] as char);
			}
			stacks.push(stack);
		}
		stacks
	};

	let instructions = instructions
		.lines()
		.map(|line| line.parse().unwrap())
		.collect::<Vec<Instruction>>();

	for inst in instructions {
		let from_len = stacks[inst.from].len();
		let mut mv = stacks[inst.from]
			.drain(from_len - inst.count..)
			.collect::<Vec<_>>();
		stacks[inst.to].append(&mut mv);
	}

	let top_crates = stacks
		.iter()
		.map(|stack| stack.last().unwrap())
		.collect::<String>();

	println!("Answer: {top_crates}");
}
