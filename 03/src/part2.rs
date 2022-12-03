fn find_common_item(s1: &str, s2: &str, s3: &str) -> char {
	for c1 in s1.chars() {
		for c2 in s2.chars() {
			for c3 in s3.chars() {
				if c1 == c2 && c2 == c3 {
					return c1;
				}
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

	let lines = input.lines().collect::<Vec<_>>();
	let sum = lines
		.chunks(3)
		.map(|chunk| find_common_item(chunk[0], chunk[1], chunk[2]))
		.map(char_to_num)
		.sum::<i64>();

	println!("Answer: {sum}");
}
