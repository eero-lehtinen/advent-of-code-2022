#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use parse_display::{Display, FromStr};

#[derive(Debug, PartialEq, Eq, Display, FromStr, Clone, Hash)]
#[display("{x},{y},{z}")]
struct Coord {
	x: usize,
	y: usize,
	z: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum Cube {
	Lava,
	#[default]
	Empty,
	Water,
}

fn main() {
	let input = include_str!("../input.txt");
	let coords = input
		.lines()
		.map(|l| l.parse().unwrap())
		.collect::<Vec<Coord>>();

	let max_x = coords.iter().map(|c| c.x).max().unwrap();
	let max_y = coords.iter().map(|c| c.y).max().unwrap();
	let max_z = coords.iter().map(|c| c.z).max().unwrap();

	let mut space = ndarray::Array3::default((max_x + 1, max_y + 1, max_z + 1));

	for c in &coords {
		space[[c.x, c.y, c.z]] = Cube::Lava;
	}

	// Flood fill
	let mut stack = vec![(1, 1, 1)];

	while let Some((x, y, z)) = stack.pop() {
		let cube = &mut space[[x, y, z]];
		if *cube != Cube::Empty {
			continue;
		}

		*cube = Cube::Water;
		if x > 0 {
			stack.push((x - 1, y, z));
		}
		if x < max_x {
			stack.push((x + 1, y, z));
		}
		if y > 0 {
			stack.push((x, y - 1, z));
		}
		if y < max_y {
			stack.push((x, y + 1, z));
		}
		if z > 0 {
			stack.push((x, y, z - 1));
		}
		if z < max_z {
			stack.push((x, y, z + 1));
		}
	}

	let mut unconnected_sides = 0;

	for Coord { x, y, z } in &coords {
		unconnected_sides += i32::from(*x == max_x || space[[x + 1, *y, *z]] == Cube::Water)
			+ i32::from(*x == 0 || space[[x - 1, *y, *z]] == Cube::Water)
			+ i32::from(*y == max_y || space[[*x, y + 1, *z]] == Cube::Water)
			+ i32::from(*y == 0 || space[[*x, y - 1, *z]] == Cube::Water)
			+ i32::from(*z == max_z || space[[*x, *y, z + 1]] == Cube::Water)
			+ i32::from(*z == 0 || space[[*x, *y, z - 1]] == Cube::Water);
	}

	println!("Answer: {unconnected_sides}");
}
