#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::cast_possible_truncation)]

use parse_display::{Display, FromStr};

#[derive(Debug, PartialEq, Eq, Display, FromStr, Hash, Clone)]
#[display("x={x}, y={y}")]
struct Point {
	x: i64,
	y: i64,
}

#[derive(Debug, PartialEq, Eq, Display, FromStr, Hash, Clone)]
#[display("Sensor at {sensor}: closest beacon is at {beacon}")]
struct Pair {
	sensor: Point,
	beacon: Point,
}

impl Pair {
	const fn manhattan(&self) -> i64 {
		manhattan(&self.sensor, &self.beacon)
	}

	fn perimeter_points(&self) -> Vec<Point> {
		let min_x = self.sensor.x - self.manhattan() - 1;
		let max_x = self.sensor.x + self.manhattan() + 1;
		let min_y = self.sensor.y - self.manhattan() - 1;
		let max_y = self.sensor.y + self.manhattan() + 1;

		let mut res = Vec::new();
		// Left to top
		for (x, y) in (min_x..=self.sensor.x).zip(self.sensor.y..=max_y) {
			res.push(Point { x, y });
		}

		// Top to right
		for (x, y) in (self.sensor.x..=max_x).zip((self.sensor.y..=max_y).rev()) {
			res.push(Point { x, y });
		}

		// Right to bottom
		for (x, y) in ((self.sensor.x..=max_x).rev()).zip((min_y..=self.sensor.y).rev()) {
			res.push(Point { x, y });
		}

		// Bottom to left
		for (x, y) in ((min_x..=self.sensor.x).rev()).zip(min_y..=self.sensor.y) {
			res.push(Point { x, y });
		}

		res
	}
}

const fn manhattan(p1: &Point, p2: &Point) -> i64 {
	(p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

const MAX_POS: i64 = 4_000_000;

fn main() {
	let input = include_str!("../input.txt");
	let pairs = input
		.lines()
		.map(|l| l.parse().unwrap())
		.collect::<Vec<Pair>>();

	// According to instructions there is only one valid point
	// -> It must exist just outside the perimeter of a station
	// -> Search space becomes a lot smaller (some millions vs 4000000^2)
	// -> Brute force becomes possible
	let mut res = Point { x: 0, y: 0 };
	'outer: for pair in &pairs {
		for point in pair.perimeter_points() {
			if point.x > MAX_POS || point.x < 0 || point.y > MAX_POS || point.y < 0 {
				continue;
			}

			let found = pairs
				.iter()
				.all(|pair| manhattan(&pair.sensor, &point) > pair.manhattan());

			if found {
				res = point;
				break 'outer;
			}
		}
	}

	let tuning_frequency = (res.x * 4_000_000) + res.y;

	println!("Answer: {tuning_frequency}");
}
