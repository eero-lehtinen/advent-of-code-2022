#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::collections::HashSet;

use parse_display::{Display, FromStr};

#[derive(Debug, PartialEq, Eq, Display, FromStr, Clone, Hash)]
#[display("{x},{y},{z}")]
struct Coord {
	x: usize,
	y: usize,
	z: usize,
}

fn main() {
	let input = include_str!("../input.txt");
	let coords = input
		.lines()
		.map(|l| l.parse().unwrap())
		.collect::<HashSet<Coord>>();

	let mut unconnected_sides = 0;

	for c in &coords {
		unconnected_sides += i32::from(!coords.contains(&Coord { x: c.x + 1, ..*c }))
			+ i32::from(!coords.contains(&Coord { x: c.x - 1, ..*c }))
			+ i32::from(!coords.contains(&Coord { y: c.y + 1, ..*c }))
			+ i32::from(!coords.contains(&Coord { y: c.y - 1, ..*c }))
			+ i32::from(!coords.contains(&Coord { z: c.z + 1, ..*c }))
			+ i32::from(!coords.contains(&Coord { z: c.z - 1, ..*c }));
	}

	println!("Answer: {unconnected_sides}");
}
