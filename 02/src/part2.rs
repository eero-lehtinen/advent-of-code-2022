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

fn calc_score((opp_choice, res): (Choice, RPSResult)) -> i64 {
	type R = RPSResult;
	type C = Choice;

	let choice_score = |c: &C| match c {
		C::Rock => 1,
		C::Paper => 2,
		C::Scissors => 3,
	};

	let result_score = |r: &RPSResult| match r {
		R::Lose => 0,
		R::Draw => 3,
		R::Win => 6,
	};

	let my_choice = match (&res, &opp_choice) {
		(R::Win, C::Rock) | (R::Lose, C::Scissors) => C::Paper,
		(R::Win, C::Scissors) | (R::Lose, C::Paper) => C::Rock,
		(R::Win, C::Paper) | (R::Lose, C::Rock) => C::Scissors,
		_ => opp_choice,
	};

	result_score(&res) + choice_score(&my_choice)
}

fn map_choice(c: &str) -> Choice {
	match c {
		"A" => Choice::Rock,
		"B" => Choice::Paper,
		"C" => Choice::Scissors,
		_ => panic!("Invalid choice"),
	}
}

fn map_result(c: &str) -> RPSResult {
	match c {
		"X" => RPSResult::Lose,
		"Y" => RPSResult::Draw,
		"Z" => RPSResult::Win,
		_ => panic!("Invalid choice"),
	}
}

fn main() {
	let input = include_str!("../input.txt");

	let score = input
		.lines()
		.map(|line| line.split_once(' ').unwrap())
		.map(|(opp, res)| (map_choice(opp), map_result(res)))
		.map(calc_score)
		.sum::<i64>();

	println!("Answer: {score}");
}
