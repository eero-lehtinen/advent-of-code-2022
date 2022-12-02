#[derive(PartialEq)]
enum Choice {
	Rock,
	Paper,
	Scissors,
}

enum RPSResult {
	Win,
	Lose,
	Draw,
}

fn calc_score((a, b): (Choice, Choice)) -> i64 {
	let choice_score = |c: &Choice| match c {
		Choice::Rock => 1,
		Choice::Paper => 2,
		Choice::Scissors => 3,
	};

	let result_score = |r: &RPSResult| match r {
		RPSResult::Lose => 0,
		RPSResult::Draw => 3,
		RPSResult::Win => 6,
	};

	let res = if a == b {
		RPSResult::Draw
	} else {
		match (&a, &b) {
			(Choice::Scissors, Choice::Rock)
			| (Choice::Rock, Choice::Paper)
			| (Choice::Paper, Choice::Scissors) => RPSResult::Win,
			_ => RPSResult::Lose,
		}
	};

	result_score(&res) + choice_score(&b)
}

fn map_choice(c: &str) -> Choice {
	match c {
		"A" | "X" => Choice::Rock,
		"B" | "Y" => Choice::Paper,
		"C" | "Z" => Choice::Scissors,
		_ => panic!("Invalid choice"),
	}
}

fn main() {
	let input = include_str!("../input.txt");

	let score = input
		.lines()
		.map(|line| line.split_once(' ').unwrap())
		.map(|(a, b)| (map_choice(a), map_choice(b)))
		.map(calc_score)
		.sum::<i64>();

	println!("Answer: {score}");
}
