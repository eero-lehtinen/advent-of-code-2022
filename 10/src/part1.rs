#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::match_wildcard_for_single_variants)]

use std::str::FromStr;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Op {
	AddX(i64),
	NoOp,
}

impl FromStr for Op {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts = s.split(' ').collect::<Vec<_>>();
		match &parts[..] {
			["noop"] => Ok(Self::NoOp),
			["addx", num] => Ok(Self::AddX(num.parse::<i64>().unwrap())),
			_ => Err(()),
		}
	}
}

fn main() {
	let input = include_str!("../input.txt");

	let mut commands = input.lines().map(|l| l.parse::<Op>().unwrap());

	let mut cycle = 0;
	let mut accumulator = 0;
	let mut acc_cycles = 20;
	let mut register_x = 1;

	loop {
		let Some(op) = commands.next() else {
			break;
		};

		let mut sub_cycle = 0;
		loop {
			cycle += 1;
			sub_cycle += 1;
			acc_cycles += 1;

			if acc_cycles == 40 {
				accumulator += cycle / 20 * 20 * register_x;
				acc_cycles -= 40;
			}

			match op {
				Op::AddX(num) if sub_cycle == 2 => {
					register_x += num;
					break;
				}
				Op::NoOp => break,
				_ => (),
			}
		}
	}

	println!("Answer: {accumulator}");
}
