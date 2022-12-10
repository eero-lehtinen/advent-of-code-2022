#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::needless_range_loop)]

use std::fmt::{Display, Formatter};
use std::{collections::HashSet, str::FromStr};

// use std::io::{stdin, stdout, Read, Write};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Dir {
	Left,
	Right,
	Up,
	Down,
}

impl FromStr for Dir {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"L" => Ok(Self::Left),
			"R" => Ok(Self::Right),
			"U" => Ok(Self::Up),
			"D" => Ok(Self::Down),
			_ => Err(()),
		}
	}
}

fn _draw_parts(parts: &[Pos]) {
	let min_x = parts.iter().map(|p| p.x).min().unwrap();
	let max_x = parts.iter().map(|p| p.x).max().unwrap();
	let min_y = parts.iter().map(|p| p.y).min().unwrap();
	let max_y = parts.iter().map(|p| p.y).max().unwrap();

	for y in (min_y..=max_y).rev() {
		for x in min_x..=max_x {
			parts.iter().position(|p| p.x == x && p.y == y).map_or_else(
				|| print!("."),
				|pos| {
					print!("{pos}");
				},
			);
		}
		println!();
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
	x: isize,
	y: isize,
}

impl Display for Pos {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

fn main() {
	let input = include_str!("../input.txt");

	let moves = input
		.lines()
		.map(|l| {
			let (dir, steps) = l.split_once(' ').unwrap();
			(dir.parse::<Dir>().unwrap(), steps.parse::<usize>().unwrap())
		})
		.collect::<Vec<_>>();

	let mut parts = [Pos { x: 0, y: 0 }; 10];

	let mut visited_map = HashSet::new();

	for (dir, steps) in moves {
		for _ in 0..steps {
			let head = &mut parts[0];
			match dir {
				Dir::Left => head.x -= 1,
				Dir::Right => head.x += 1,
				Dir::Up => head.y += 1,
				Dir::Down => head.y -= 1,
			}

			for i in 1..parts.len() {
				let [ref head, ref mut tail] = parts[i - 1..=i] else {
					panic!("this bad");
				};

				let x_offset = head.x - tail.x;
				let y_offset = head.y - tail.y;

				if x_offset.abs() + y_offset.abs() >= 3 {
					tail.x += x_offset.signum();
					tail.y += y_offset.signum();
				} else if x_offset.abs() == 2 {
					tail.x += x_offset.signum();
				} else if y_offset.abs() == 2 {
					tail.y += y_offset.signum();
				}
			}
			visited_map.insert(*parts.last().unwrap());
		}
		// println!("{}", parts.map(|p| p.to_string()).join(", "));
		// draw_parts(&parts);
		// stdin().read_exact(&mut [0]).unwrap();
	}

	println!("Answer: {}", visited_map.len());
}
