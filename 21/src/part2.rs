#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::too_many_lines)]

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
	Add,
	Mul,
	Sub,
	Div,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WaitMonkey {
	op: Op,
	monkeys: [String; 2],
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
	Num(i64),
	Solve(WaitMonkey),
}

fn parse_monkeys(input: &str) -> (HashMap<String, Value>, HashMap<String, WaitMonkey>) {
	let mut monkeys = HashMap::new();
	let mut wait_monkeys = HashMap::new();

	for line in input.lines() {
		let (name, rest) = line.split_once(':').unwrap();

		if let Ok(n) = rest.trim().parse::<i64>() {
			monkeys.insert(
				name.to_owned(),
				if name == "humn" {
					Value::Solve(WaitMonkey {
						op: Op::Add,
						monkeys: [String::new(), String::new()],
					})
				} else {
					Value::Num(n)
				},
			);
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
	let input = include_str!("../input.txt");

	let (mut monkeys, mut wait_monkeys) = parse_monkeys(input);

	let total_monkeys = monkeys.len() + wait_monkeys.len();

	while monkeys.len() < total_monkeys {
		wait_monkeys.retain(|name, monkey| {
			if let Some(first) = monkeys.get(&monkey.monkeys[0]) {
				if let Some(second) = monkeys.get(&monkey.monkeys[1]) {
					let (Value::Num(n1), Value::Num(n2)) = (first, second) else {
						// If this path includes the human, store it for solving later
						monkeys.insert(name.clone(), Value::Solve(monkey.clone()));
						return false;
					};

					let result = match monkey.op {
						Op::Add => n1 + n2,
						Op::Mul => n1 * n2,
						Op::Sub => n1 - n2,
						Op::Div => n1 / n2,
					};

					monkeys.insert(name.clone(), Value::Num(result));

					return false;
				}
			}
			true
		});
	}

	// Backtrack to solve from root to humn
	let root = monkeys.get("root").unwrap().clone();

	// Find the number we need to solve for
	let mut current = match &root {
		Value::Solve(monkey) => {
			match (
				monkeys.get(&monkey.monkeys[0]).unwrap(),
				monkeys.get(&monkey.monkeys[1]).unwrap(),
			) {
				(Value::Num(n), Value::Solve(solve_monkey)) => (
					monkey.monkeys[1].clone(),
					Value::Solve(solve_monkey.clone()),
					*n,
				),
				(Value::Solve(solve_monkey), Value::Num(n)) => (
					monkey.monkeys[0].clone(),
					Value::Solve(solve_monkey.clone()),
					*n,
				),
				_ => unreachable!(),
			}
		}
		Value::Num(_) => unreachable!(),
	};

	// Solve whole path from root to humn
	while current.0 != "humn" {
		let (_, value, current_answer) = current;
		current = match &value {
			Value::Solve(monkey) => {
				match (
					monkeys.get(&monkey.monkeys[0]).unwrap(),
					monkeys.get(&monkey.monkeys[1]).unwrap(),
				) {
					(Value::Num(n), Value::Solve(solve_monkey)) => (
						monkey.monkeys[1].clone(),
						Value::Solve(solve_monkey.clone()),
						match monkey.op {
							Op::Add => current_answer - n,
							Op::Mul => current_answer / n,
							Op::Sub => n - current_answer,
							Op::Div => n / current_answer,
						},
					),
					(Value::Solve(solve_monkey), Value::Num(n)) => (
						monkey.monkeys[0].clone(),
						Value::Solve(solve_monkey.clone()),
						match monkey.op {
							Op::Add => current_answer - n,
							Op::Mul => current_answer / n,
							Op::Sub => current_answer + n,
							Op::Div => current_answer * n,
						},
					),
					_ => unreachable!(),
				}
			}
			Value::Num(_) => unreachable!(),
		};
	}

	let human_yell = current.2;

	println!("Answer: {human_yell:?}");
}
