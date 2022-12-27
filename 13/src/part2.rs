#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Item {
	Num(i32),
	List(Vec<Item>),
}

// Convenience macros
macro_rules! list {
	($($item:expr),*) => {
		Item::List(vec![$($item),*])
	};
}

macro_rules! num {
	($num:expr) => {
		Item::Num($num)
	};
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
	let mut items = input
		.lines()
		.filter_map(|line| line.parse::<Item>().ok())
		.collect::<Vec<_>>();

	let divider1 = list![list![num!(2)]];
	items.push(divider1.clone());
	let divider2 = list![list![num!(6)]];
	items.push(divider2.clone());

	items.sort_unstable();

	let divider1_pos = items.binary_search(&divider1).unwrap() + 1;
	let divider2_pos = items.binary_search(&divider2).unwrap() + 1;

	println!("Answer: {}", divider2_pos * divider1_pos);
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
		assert_eq!(("[1]").parse::<Item>().unwrap(), list![num!(1)]);
	}

	#[test]
	fn parse_3() {
		assert_eq!(("[1,2]").parse::<Item>().unwrap(), list![num!(1), num!(2)]);
	}

	#[test]
	fn parse_4() {
		assert_eq!(
			("[1,[2]]").parse::<Item>().unwrap(),
			list![num!(1), list![num!(2)]]
		);
	}

	#[test]
	fn parse_5() {
		assert_eq!(
			("[[1],2]").parse::<Item>().unwrap(),
			list![list![num!(1)], num!(2)]
		);
	}

	#[test]
	fn parse_6() {
		assert_eq!(("[]").parse::<Item>().unwrap(), list![]);
	}

	#[test]
	fn parse_7() {
		assert_eq!(
			("[1,[[2]]]").parse::<Item>().unwrap(),
			list![num!(1), list![list![num!(2)]]]
		);
	}
}
