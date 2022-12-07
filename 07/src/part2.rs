#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Dir {
	size: usize,
	files: HashSet<String>,
	children: HashSet<String>,
}

fn main() {
	let input = include_str!("../input.txt");
	let lines = input.lines().collect::<Vec<_>>();

	let mut dirs = HashMap::new();
	dirs.insert(
		vec!["/".to_string()],
		Dir {
			size: 0,
			children: HashSet::new(),
			files: HashSet::new(),
		},
	);

	let mut pwd = vec![];

	for line in lines {
		let parts = line.split_whitespace().collect::<Vec<_>>();
		match parts[..] {
			["$", "cd", dir] => match dir {
				".." => {
					pwd.pop();
				}
				"/" => {
					pwd = vec!["/".to_string()];
				}
				dir => {
					dirs.get_mut(&pwd).unwrap().children.insert(dir.to_string());

					pwd.push(dir.to_string());

					dirs.entry(pwd.clone()).or_insert_with(|| Dir {
						size: 0,
						children: HashSet::new(),
						files: HashSet::new(),
					});
				}
			},
			["dir", _] | ["$", "ls"] => {}
			[size, file] => {
				if !dirs[&pwd.clone()].files.contains(file) {
					dirs.entry(pwd.clone()).and_modify(|d| {
						d.files.insert(file.to_string());
						d.size += size.parse::<usize>().unwrap();
					});
				}
			}
			_ => panic!("Unknown command: {line}"),
		}
	}

	let mut queue = dirs.keys().cloned().collect::<Vec<_>>();
	queue.sort_unstable_by_key(std::vec::Vec::len);

	while let Some(path) = queue.pop() {
		if let Some(parent) = path.get(0..path.len() - 1) {
			let size = dirs[&path].size;
			dirs.entry(parent.to_vec()).and_modify(|d| {
				d.size += size;
			});
		}
	}

	let taken_space = dirs[&vec!["/".to_string()]].size;
	let total_space = 70_000_000;
	let free_space = total_space - taken_space;
	let space_needed = 30_000_000;
	let space_to_free = space_needed - free_space;

	let mut sizes = dirs.values().map(|d| d.size).collect::<Vec<_>>();
	sizes.sort_unstable();
	let ans = sizes.iter().find(|s| **s >= space_to_free).unwrap();

	println!("Answer: {ans}");
}
