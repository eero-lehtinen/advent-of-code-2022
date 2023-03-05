#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
	clippy::cast_possible_truncation,
	clippy::cast_sign_loss,
	clippy::cast_possible_wrap
)]

fn main() {
	let mut numbers: Vec<(usize, i32)> = include_str!("../input.txt")
		.lines()
		.map(|s| s.parse::<i32>().unwrap())
		.enumerate()
		.collect();

	for original_index in 0..numbers.len() {
		let index = numbers.iter().position(|x| x.0 == original_index).unwrap();
		let value = numbers[index].1;
		let new_i = (index as i32 + value).rem_euclid(numbers.len() as i32 - 1);
		let tmp = numbers.remove(index);
		numbers.insert(new_i as usize, tmp);
	}

	let zero_i = numbers.iter().position(|&x| x.1 == 0).unwrap();

	let coordinates = (
		numbers[(zero_i + 1000) % numbers.len()].1,
		numbers[(zero_i + 2000) % numbers.len()].1,
		numbers[(zero_i + 3000) % numbers.len()].1,
	);

	println!(
		"Answer: {:?}",
		coordinates.0 + coordinates.1 + coordinates.2
	);
}
