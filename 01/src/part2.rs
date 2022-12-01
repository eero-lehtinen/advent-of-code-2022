fn main() {
	let input = include_str!("../input.txt");

	let mut sums: Vec<i64> = input
		.split("\n\n")
		.map(|part| part.split('\n').flat_map(str::parse::<i64>).sum())
		.collect();

	sums.select_nth_unstable_by(2, |a, b| a.cmp(b).reverse());

	let top3_sum: i64 = sums.iter().take(3).sum();

	println!("Answer: {top3_sum}");
}
