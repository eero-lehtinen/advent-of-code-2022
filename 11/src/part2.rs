#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]

use std::{collections::VecDeque, str::FromStr};

#[derive(Debug)]
enum Value {
	Literal(i64),
	Old,
}

#[derive(Debug)]
struct Operation(fn(i64, i64) -> i64, Value);

#[derive(Debug)]
struct DivisibleTest {
	num: i64,
	false_monkey: usize,
	true_monkey: usize,
}

#[derive(Debug)]
struct Monkey {
	items: VecDeque<i64>,
	operation: Operation,
	divisible_test: DivisibleTest,
	inspect_count: u64,
}

impl FromStr for Monkey {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut lines = s.lines();
		lines.next();
		let items_str = lines.next().unwrap().split_once(": ").unwrap().1;
		let items = items_str
			.split(", ")
			.map(|s| s.parse::<i64>().unwrap())
			.collect::<VecDeque<_>>();

		let operation_str = lines.next().unwrap().split_once("= ").unwrap().1;
		let [_, p2, p3] = operation_str.split(' ').collect::<Vec<_>>()[..] else {
			return Err(());
		};
		let op = match p2 {
			"+" => |a, b| a + b,
			"*" => |a, b| a * b,
			_ => unreachable!(),
		};
		let val = match p3 {
			"old" => Value::Old,
			s => Value::Literal(s.parse::<i64>().unwrap()),
		};

		let operation = Operation(op, val);

		let parse_last_num = |s: &str| s.split(' ').last().unwrap().parse::<usize>().unwrap();

		let divisible_test = DivisibleTest {
			num: parse_last_num(lines.next().unwrap()) as i64,
			true_monkey: parse_last_num(lines.next().unwrap()),
			false_monkey: parse_last_num(lines.next().unwrap()),
		};

		Ok(Self {
			items,
			operation,
			divisible_test,
			inspect_count: 0,
		})
	}
}

fn main() {
	let input = include_str!("../input.txt");

	let mut monkeys = input
		.split("\n\n")
		.map(|s| s.parse::<Monkey>().unwrap())
		.collect::<Vec<_>>();

	let common_divisor = monkeys
		.iter()
		.map(|m| m.divisible_test.num)
		.product::<i64>();

	for _ in 0..10_000 {
		for i in 0..monkeys.len() {
			while let Some(item) = monkeys[i].items.pop_front() {
				let operation = &monkeys[i].operation;

				let new_item = match operation {
					Operation(func, Value::Literal(l)) => func(item, *l),
					Operation(func, Value::Old) => func(item, item),
				} % common_divisor;

				let divisible_test = &monkeys[i].divisible_test;
				let idx = if new_item % divisible_test.num == 0 {
					divisible_test.true_monkey
				} else {
					divisible_test.false_monkey
				};

				monkeys[i].inspect_count += 1;

				monkeys[idx].items.push_back(new_item);
			}
		}
	}

	let mut monkey_inspects = monkeys.iter().map(|m| m.inspect_count).collect::<Vec<_>>();
	monkey_inspects.select_nth_unstable_by(1, |a, b| a.cmp(b).reverse());
	let sum = monkey_inspects.iter().take(2).product::<u64>();

	println!("Answer: {sum}");
}
