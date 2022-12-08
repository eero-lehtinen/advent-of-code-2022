#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::needless_range_loop)]

enum Dir {
	Left,
	Right,
	Up,
	Down,
}

fn show_dist(input: &Vec<Vec<u8>>, pos: (usize, usize), dir: &Dir) -> usize {
	let (mut row, mut col) = pos;
	let height = input[row][col];
	loop {
		if col == 0 || col == input[0].len() - 1 || row == 0 || row == input.len() - 1 {
			break;
		}

		match dir {
			Dir::Left => col -= 1,
			Dir::Right => col += 1,
			Dir::Up => row -= 1,
			Dir::Down => row += 1,
		}

		if i16::from(input[row][col]) >= i16::from(height) {
			break;
		}
	}

	match dir {
		Dir::Left => pos.1 - col,
		Dir::Right => col - pos.1,
		Dir::Up => pos.0 - row,
		Dir::Down => row - pos.0,
	}
}

fn main() {
	let input = include_str!("../input.txt");

	let input = input
		.lines()
		.map(|l| l.bytes().collect::<Vec<_>>())
		.collect::<Vec<_>>();

	let mut highest_score = 0;

	for row in 0..input.len() {
		for col in 0..input[0].len() {
			let score = show_dist(&input, (row, col), &Dir::Left)
				* show_dist(&input, (row, col), &Dir::Right)
				* show_dist(&input, (row, col), &Dir::Up)
				* show_dist(&input, (row, col), &Dir::Down);

			if score > highest_score {
				highest_score = score;
			}
		}
	}
	println!("Answer: {highest_score}");
}
