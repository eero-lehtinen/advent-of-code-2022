#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::needless_range_loop)]

enum Dir {
	Left,
	Right,
	Up,
	Down,
}

fn shows_in_dir(input: &Vec<Vec<u8>>, pos: (usize, usize), dir: &Dir) -> bool {
	let (mut row, mut col) = pos;
	let height = input[row][col];
	loop {
		if col == 0 || col == input[0].len() - 1 || row == 0 || row == input.len() - 1 {
			return true;
		}

		match dir {
			Dir::Left => col -= 1,
			Dir::Right => col += 1,
			Dir::Up => row -= 1,
			Dir::Down => row += 1,
		}

		if i16::from(input[row][col]) >= i16::from(height) {
			return false;
		}
	}
}

fn main() {
	let input = include_str!("../input.txt");

	let input = input
		.lines()
		.map(|l| l.bytes().collect::<Vec<_>>())
		.collect::<Vec<_>>();

	let mut shown_count = 0;

	for row in 0..input.len() {
		for col in 0..input[0].len() {
			shown_count += i32::from(
				shows_in_dir(&input, (row, col), &Dir::Left)
					|| shows_in_dir(&input, (row, col), &Dir::Right)
					|| shows_in_dir(&input, (row, col), &Dir::Up)
					|| shows_in_dir(&input, (row, col), &Dir::Down),
			);
		}
	}
	println!("Answer: {shown_count}");
}
