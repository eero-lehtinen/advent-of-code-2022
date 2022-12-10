#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::needless_range_loop)]

use std::{collections::HashSet, str::FromStr};

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

fn _draw_visited_map(map: &HashSet<Pos>) {
	let min_x = map.iter().map(|p| p.x).min().unwrap();
	let max_x = map.iter().map(|p| p.x).max().unwrap();
	let min_y = map.iter().map(|p| p.y).min().unwrap();
	let max_y = map.iter().map(|p| p.y).max().unwrap();

	for y in min_y..=max_y {
		for x in min_x..=max_x {
			if map.contains(&Pos { x, y }) {
				print!("#");
			} else {
				print!(".");
			}
		}
		println!();
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
	x: isize,
	y: isize,
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

	let mut head = Pos { x: 0, y: 0 };
	let mut tail = Pos { x: 0, y: 0 };

	let mut visited_map = HashSet::new();

	for (dir, steps) in moves {
		for _ in 0..steps {
			match dir {
				Dir::Left => head.x -= 1,
				Dir::Right => head.x += 1,
				Dir::Up => head.y += 1,
				Dir::Down => head.y -= 1,
			}

			let x_offset = head.x - tail.x;
			let y_offset = head.y - tail.y;

			if x_offset.abs() > 1 || y_offset.abs() > 1 {
				let Pos { x, y } = head;
				tail = match dir {
					Dir::Left => Pos { x: x + 1, y },
					Dir::Right => Pos { x: x - 1, y },
					Dir::Up => Pos { x, y: y - 1 },
					Dir::Down => Pos { x, y: y + 1 },
				}
			} else if x_offset.abs() > 1 {
				tail.x += x_offset.signum();
			} else if y_offset.abs() > 1 {
				tail.y += y_offset.signum();
			}

			visited_map.insert(tail);

			// println!("\nHead: {head:?}, Tail: {tail:?}");
			// draw_visited_map(&visited_map);
		}
	}

	println!("Answer: {}", visited_map.len());
}
