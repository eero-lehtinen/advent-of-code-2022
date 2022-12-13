#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::collections::{HashSet, VecDeque};

fn main() {
	let input = include_str!("../input.txt");

	let mut start_candidates = vec![];
	let mut end = (0, 0);

	let heightmap = input
		.lines()
		.enumerate()
		.map(|(y, l)| {
			l.bytes()
				.enumerate()
				.map(|(x, b)| {
					let h = match b {
						b'a'..=b'z' => b - b'a',
						b'S' => 0,
						b'E' => {
							end = (x, y);
							25
						}
						_ => unreachable!(),
					};
					if h == 0 {
						start_candidates.push((x, y));
					};
					h
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	// BFS

	let mut queue = VecDeque::new();
	queue.extend(start_candidates.iter().map(|&s| (s, 0)));

	let mut visited = HashSet::new();

	let mut result = i32::MAX;

	while let Some(((x, y), steps)) = queue.pop_front() {
		if (x, y) == end && steps < result {
			result = steps;
		}

		if visited.contains(&(x, y)) {
			continue;
		}

		visited.insert((x, y));

		let h = heightmap[y][x];

		if y > 0 && heightmap[y - 1][x] <= h + 1 {
			queue.push_back(((x, y - 1), steps + 1));
		}

		if y < heightmap.len() - 1 && heightmap[y + 1][x] <= h + 1 {
			queue.push_back(((x, y + 1), steps + 1));
		}

		if x > 0 && heightmap[y][x - 1] <= h + 1 {
			queue.push_back(((x - 1, y), steps + 1));
		}

		if x < heightmap[y].len() - 1 && heightmap[y][x + 1] <= h + 1 {
			queue.push_back(((x + 1, y), steps + 1));
		}
	}

	println!("Answer: {result}");
}
