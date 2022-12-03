fn find_common_item((s1, s2): (&str, &str)) -> char {
	for c1 in s1.chars() {
		for c2 in s2.chars() {
			if c1 == c2 {
				return c1;
			}
		}
	}
	panic!("No common item found");
}

fn char_to_num(c: char) -> i64 {
	// Convert char to number, a=1 ... z=26, A=27 ... Z=52
	match c {
		'a'..='z' => c as i64 - 96,
		'A'..='Z' => c as i64 - 38,
		_ => panic!("Invalid char"),
	}
}

fn main() {
	let input = include_str!("../input.txt");

	let sum = input
		.lines()
		.map(|line| line.split_at(line.len() / 2))
		.map(find_common_item)
		.map(char_to_num)
		.sum::<i64>();

	println!("Answer: {sum}");
}
