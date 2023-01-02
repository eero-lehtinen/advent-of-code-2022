#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use bitvec::prelude::*;
use lazy_regex::regex_captures;
use ndarray::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use std::str::FromStr;
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Scan {
	valve: String,
	flow_rate: i32,
	tunnels: Vec<String>,
}

impl FromStr for Scan {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (_, valve, flow_rate, tunnels) = regex_captures!(
			"^Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? (.*)$",
			s
		)
		.unwrap();
		Ok(Self {
			valve: valve.to_string(),
			flow_rate: flow_rate.parse().unwrap(),
			tunnels: tunnels
				.split(", ")
				.map(str::to_string)
				.collect::<Vec<String>>(),
		})
	}
}

const TIME: i32 = 26;

fn find_distance(scans: &HashMap<String, Scan>, start: String, end: &String) -> i32 {
	let mut queue = VecDeque::new();
	queue.push_back((start, 0));
	let mut visited = HashSet::new();

	while let Some((valve, distance)) = queue.pop_front() {
		let scan = &scans[&valve];
		if valve == *end {
			return distance;
		}
		visited.insert(valve.clone());
		for tunnel in &scan.tunnels {
			if !visited.contains(tunnel) {
				queue.push_back((tunnel.clone(), distance + 1));
			}
		}
	}

	panic!("No path found");
}

fn find_best_pressure(flow_rates: &[i32], adj_matrix: &Array<i32, Ix2>, visited: u16) -> i32 {
	// DFS
	let mut best_pressure = 0;
	// visited, current, elapsed, pressure
	let mut stack = vec![(visited.into_bitarray::<Lsb0>(), 0, 0, 0)];
	while let Some((visited, current, elapsed, pressure)) = stack.pop() {
		for i in 0..adj_matrix.nrows() {
			if visited[i] {
				continue;
			}

			let mut new_visited = visited;
			new_visited.set(i, true);
			let new_elapsed = elapsed + adj_matrix[[current, i]] + 1;

			best_pressure = best_pressure.max(pressure);

			if new_elapsed >= TIME {
				continue;
			}

			stack.push((
				new_visited,
				i,
				new_elapsed,
				pressure + flow_rates[i] * (TIME - new_elapsed),
			));
		}
	}
	best_pressure
}

fn main() {
	let input = include_str!("../input.txt");
	let scans = input
		.lines()
		.map(|l| l.parse::<Scan>().unwrap())
		.map(|s| (s.valve.clone(), s))
		.collect::<HashMap<String, Scan>>();

	let mut valves = scans
		.keys()
		.filter(|k| scans[*k].flow_rate > 0 || k == &"AA")
		.cloned()
		.collect::<Vec<String>>();

	valves.sort_unstable();

	let flow_rates = valves
		.iter()
		.map(|v| scans[v].flow_rate)
		.collect::<Vec<i32>>();

	let mut adj_matrix = Array2::zeros((valves.len(), valves.len()));

	for i in 0..valves.len() {
		for j in 0..valves.len() {
			adj_matrix[[i, j]] = find_distance(&scans, valves[i].clone(), &valves[j]);
		}
	}

	let best_pressure = (0..u16::MAX / 2)
		.into_par_iter()
		.step_by(2)
		.map(|visited| {
			find_best_pressure(&flow_rates, &adj_matrix, visited)
				+ find_best_pressure(&flow_rates, &adj_matrix, !visited & !1_u16)
		})
		.max()
		.unwrap();

	println!("Answer: {best_pressure}");
}
