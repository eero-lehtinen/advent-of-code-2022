#![warn(clippy::pedantic, clippy::nursery)]

use std::collections::HashMap;

enum Op {
	Add,
	Mul,
	Sub,
	Div,
}

struct WaitMonkey {
	op: Op,
	monkeys: [String; 2],
}

fn parse_monkeys(input: &str) -> (HashMap<String, i64>, HashMap<String, WaitMonkey>) {
	let mut monkeys = HashMap::new();
	let mut wait_monkeys = HashMap::new();

	for line in input.lines() {
		let (name, rest) = line.split_once(':').unwrap();

		if let Ok(n) = rest.trim().parse::<i64>() {
			monkeys.insert(name.to_owned(), n);
			continue;
		}

		let mut parts = rest.trim().split(' ');

		let first = parts.next().unwrap();
		let op = parts.next().unwrap();
		let second = parts.next().unwrap();

		let op = match op {
			"+" => Op::Add,
			"*" => Op::Mul,
			"-" => Op::Sub,
			"/" => Op::Div,
			_ => unreachable!(),
		};

		wait_monkeys.insert(
			name.to_owned(),
			WaitMonkey {
				op,
				monkeys: [first.to_owned(), second.to_owned()],
			},
		);
	}

	(monkeys, wait_monkeys)
}

fn main() {
	let input = include_str!("../sample.txt");

	let (mut monkeys, mut wait_monkeys) = parse_monkeys(input);

	while !wait_monkeys.is_empty() {
		wait_monkeys.retain(|name, monkey| {
			if let Some(first) = monkeys.get(&monkey.monkeys[0]) {
				if let Some(second) = monkeys.get(&monkey.monkeys[1]) {
					let result = match monkey.op {
						Op::Add => first + second,
						Op::Mul => first * second,
						Op::Sub => first - second,
						Op::Div => first / second,
					};

					monkeys.insert(name.clone(), result);

					return false;
				}
			}
			true
		});
	}

	println!("Answer: {:?}", monkeys["root"]);
}
