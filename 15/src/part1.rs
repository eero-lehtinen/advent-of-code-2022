#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use parse_display::{Display, FromStr};

#[derive(Debug, PartialEq, Eq, Display, FromStr, Hash, Clone)]
#[display("x={x}, y={y}")]
struct Point {
	x: i32,
	y: i32,
}

#[derive(Debug, PartialEq, Eq, Display, FromStr, Hash, Clone)]
#[display("Sensor at {sensor}: closest beacon is at {beacon}")]
struct Pair {
	sensor: Point,
	beacon: Point,
}

impl Pair {
	const fn manhattan(&self) -> i32 {
		manhattan(&self.sensor, &self.beacon)
	}
}

const fn manhattan(p1: &Point, p2: &Point) -> i32 {
	(p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn main() {
	let input = include_str!("../input.txt");
	let pairs = input
		.lines()
		.map(|l| l.parse().unwrap())
		.collect::<Vec<Pair>>();

	let (min_x, max_x) = (
		pairs
			.iter()
			.map(|p| p.sensor.x - p.manhattan())
			.min()
			.unwrap(),
		pairs
			.iter()
			.map(|p| p.sensor.x + p.manhattan())
			.max()
			.unwrap(),
	);

	let mut not_beacon_count = 0;
	let y = 2_000_000;
	for x in min_x..=max_x {
		let point = Point { x, y };
		let mut closer = false;
		for pair in &pairs {
			if manhattan(&pair.sensor, &point) <= pair.manhattan() && pair.beacon != point {
				closer = true;
				break;
			}
		}
		if closer {
			not_beacon_count += 1;
		}
	}

	println!("Answer: {not_beacon_count}");
}
