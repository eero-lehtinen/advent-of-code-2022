#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Item {
	Num(i32),
	List(Vec<Item>),
}

impl FromStr for Item {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.is_empty() {
			return Err(());
		}
		Ok(if s.starts_with('[') {
			let mut part = String::new();
			let mut parts = Vec::new();
			let mut depth = 0;
			for c in s[1..s.len() - 1].chars() {
				if c == '[' {
					depth += 1;
				} else if c == ']' {
					depth -= 1;
				}
				if c == ',' && depth == 0 && !part.is_empty() {
					parts.push(part.parse().unwrap());
					part.clear();
				} else {
					part.push(c);
				}
			}
			if !part.is_empty() {
				parts.push(part.parse().unwrap());
			}

			Self::List(parts)
		} else {
			Self::Num(s.parse().unwrap())
		})
	}
}

impl PartialOrd for Item {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Item {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match (self, other) {
			(Self::Num(a), Self::Num(b)) => a.cmp(b),
			(Self::List(a), Self::List(b)) => {
				for (a, b) in a.iter().zip(b.iter()) {
					match a.cmp(b) {
						std::cmp::Ordering::Equal => continue,
						ord => return ord,
					}
				}
				a.len().cmp(&b.len())
			}
			(a @ Self::Num(_), b @ Self::List(_)) => Self::List(vec![a.clone()]).cmp(b),
			(a @ Self::List(_), b @ Self::Num(_)) => a.cmp(&Self::List(vec![b.clone()])),
		}
	}
}

fn main() {
	let input = include_str!("../input.txt");
	let mut items = input.lines().filter_map(|line| line.parse::<Item>().ok());

	let mut pairs = Vec::new();
	while let (Some(a), Some(b)) = (items.next(), items.next()) {
		pairs.push((a, b));
	}

	let result = pairs
		.iter()
		.enumerate()
		.filter_map(|(i, (a, b))| if a <= b { Some(i + 1) } else { None })
		.sum::<usize>();

	println!("Answer: {result}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_1() {
		assert_eq!(("1").parse::<Item>().unwrap(), Item::Num(1));
	}

	#[test]
	fn parse_2() {
		assert_eq!(
			("[1]").parse::<Item>().unwrap(),
			Item::List(vec![Item::Num(1)])
		);
	}

	#[test]
	fn parse_3() {
		assert_eq!(
			("[1,2]").parse::<Item>().unwrap(),
			Item::List(vec![Item::Num(1), Item::Num(2)])
		);
	}

	#[test]
	fn parse_4() {
		assert_eq!(
			("[1,[2]]").parse::<Item>().unwrap(),
			Item::List(vec![Item::Num(1), Item::List(vec![Item::Num(2)])])
		);
	}

	#[test]
	fn parse_5() {
		assert_eq!(
			("[[1],2]").parse::<Item>().unwrap(),
			Item::List(vec![Item::List(vec![Item::Num(1)]), Item::Num(2)])
		);
	}

	#[test]
	fn parse_6() {
		assert_eq!(("[]").parse::<Item>().unwrap(), Item::List(vec![]));
	}

	#[test]
	fn parse_7() {
		assert_eq!(
			("[1,[[2]]]").parse::<Item>().unwrap(),
			Item::List(vec![
				Item::Num(1),
				Item::List(vec![Item::List(vec![Item::Num(2)])])
			])
		);
	}
}
