use std::str::FromStr;

struct Range {
	pub start: i64,
	pub end: i64,
}

impl FromStr for Range {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (left, right) = s.split_once('-').unwrap();
		let start = left.parse().unwrap();
		let end = right.parse().unwrap();
		Ok(Range { start, end })
	}
}

fn main() {
	let input = include_str!("../input.txt");

	let ranges = input
		.lines()
		.map(|line| line.split_once(',').unwrap())
		.map(|(s1, s2)| (s1.parse::<Range>().unwrap(), s2.parse::<Range>().unwrap()))
		.collect::<Vec<_>>();

	let fully_contains_ranges_count = ranges
		.iter()
		.map(|(r1, r2)| {
			i64::from(
				r1.start <= r2.start && r1.end >= r2.end
					|| r2.start <= r1.start && r2.end >= r1.end,
			)
		})
		.sum::<i64>();

	println!("Answer: {fully_contains_ranges_count}");
}
