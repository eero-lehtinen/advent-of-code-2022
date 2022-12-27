#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use parse_display::{Display, FromStr};

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Display, FromStr, Hash, Clone)]
#[display("{x},{y}")]
struct Point {
	x: i32,
	y: i32,
}

fn main() {
	let input = include_str!("../input.txt");
	let paths = input
		.lines()
		.map(|l| {
			l.split(" -> ")
				.map(|p| p.parse().unwrap())
				.collect::<Vec<Point>>()
		})
		.collect::<Vec<_>>();

	let mut map = HashSet::new();
	for path in paths {
		for i in 0..path.len() - 1 {
			let pos1 = &path[i];
			let pos2 = &path[i + 1];

			let (x1, x2) = (pos1.x.min(pos2.x), pos1.x.max(pos2.x));
			let (y1, y2) = (pos1.y.min(pos2.y), pos1.y.max(pos2.y));

			for x in x1..=x2 {
				for y in y1..=y2 {
					map.insert(Point { x, y });
				}
			}
		}
	}

	let sand_drop = Point { x: 500, y: 0 };

	let mut sand_count = 0;

	'outer: loop {
		let mut cur_pos = sand_drop.clone();
		loop {
			// Try down
			cur_pos.y += 1;
			if cur_pos.y > 1000 {
				break 'outer;
			}
			if !map.contains(&cur_pos) {
				continue;
			}

			// Try left
			cur_pos.x -= 1;
			if !map.contains(&cur_pos) {
				continue;
			}
			// Try right
			cur_pos.x += 2;
			if !map.contains(&cur_pos) {
				continue;
			}
			// Go to rest in last empty position
			cur_pos.x -= 1;
			cur_pos.y -= 1;
			map.insert(cur_pos.clone());
			break;
		}
		sand_count += 1;
	}

	println!("Answer: {sand_count}");
}
