fn main() {
	let input = include_str!("../input.txt");

	let largest_sum = input
		.split("\n\n")
		.map(|part| part.split('\n').flat_map(str::parse::<i64>).sum())
		.max()
		.unwrap_or(0);

	println!("Answer: {largest_sum}");
}
