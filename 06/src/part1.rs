#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use itertools::Itertools;

const WINDOW_SIZE: usize = 4;

fn main() {
	let input = include_str!("../input.txt");
	let input = input.chars().collect::<Vec<_>>();

	let marker = input
		.windows(WINDOW_SIZE)
		.position(|w| w.iter().all_unique())
		.unwrap()
		+ WINDOW_SIZE;

	println!("Answer: {marker}");
}
