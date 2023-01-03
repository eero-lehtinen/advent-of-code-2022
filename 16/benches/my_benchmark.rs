#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_panics_doc)]
use criterion::{criterion_group, criterion_main, Criterion};

use bitvec::prelude::*;
use lazy_regex::regex_captures;
use ndarray::prelude::*;
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

fn get_pressure_adj_matrix_number_visited(flow_rates: &[i32], adj_matrix: &Array<i32, Ix2>) -> i32 {
	// DFS
	let mut best_pressure = 0;
	// visited, current, elapsed, pressure
	let mut stack = vec![(0_u16, 0, 0, 0)];
	while let Some((visited, current, elapsed, pressure)) = stack.pop() {
		for i in 0..adj_matrix.nrows() {
			if visited & (1 << i) != 0 {
				continue;
			}

			let mut new_visited = visited;
			new_visited |= 1 << i;
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

fn get_pressure_adj_matrix_bitarray_visited(
	flow_rates: &[i32],
	adj_matrix: &Array<i32, Ix2>,
) -> i32 {
	// DFS
	let mut best_pressure = 0;
	// visited, current, elapsed, pressure
	let mut stack = vec![(0_u16.into_bitarray::<Lsb0>(), 0, 0, 0)];
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

fn get_pressure_adj_matrix_array_visited(flow_rates: &[i32], adj_matrix: &Array<i32, Ix2>) -> i32 {
	// DFS
	let mut best_pressure = 0;
	// visited, current, elapsed, pressure
	let mut stack = vec![([false; 16], 0, 0, 0)];
	while let Some((visited, current, elapsed, pressure)) = stack.pop() {
		for i in 0..adj_matrix.nrows() {
			if visited[i] {
				continue;
			}

			let mut new_visited = visited;
			new_visited[i] = true;
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

fn get_pressure_adj_matrix_hash_set_visited(
	flow_rates: &[i32],
	adj_matrix: &Array<i32, Ix2>,
) -> i32 {
	// DFS
	let mut best_pressure = 0;
	// visited, current, elapsed, pressure
	let mut stack = vec![(HashSet::new(), 0, 0, 0)];
	while let Some((visited, current, elapsed, pressure)) = stack.pop() {
		for i in 0..adj_matrix.nrows() {
			if visited.contains(&i) {
				continue;
			}

			let mut new_visited = visited.clone();
			new_visited.insert(i);
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

fn get_pressure_adj_matrix_vec_visited(flow_rates: &[i32], adj_matrix: &Array<i32, Ix2>) -> i32 {
	// DFS
	let mut best_pressure = 0;
	// visited, current, elapsed, pressure
	let mut stack = vec![(vec![], 0, 0, 0)];
	while let Some((visited, current, elapsed, pressure)) = stack.pop() {
		for i in 0..adj_matrix.nrows() {
			if visited.contains(&i) {
				continue;
			}

			let mut new_visited = visited.clone();
			new_visited.push(i);
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

fn get_pressure_hashmap(
	flow_rates: &[i32],
	distances: &HashMap<(usize, usize), i32>,
	visited: u16,
) -> i32 {
	// DFS
	let mut best_pressure = 0;
	// visited, current, elapsed, pressure
	let mut stack = vec![(visited.into_bitarray::<Lsb0>(), 0, 0, 0)];
	while let Some((visited, current, elapsed, pressure)) = stack.pop() {
		for i in 0..flow_rates.len() {
			if visited[i] {
				continue;
			}

			let mut new_visited = visited;
			new_visited.set(i, true);
			let new_elapsed = elapsed + distances[&(current, i)] + 1;

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

fn get_pressure_string_hashmap(
	valves: &[String],
	scans: &HashMap<String, Scan>,
	distances: &HashMap<(String, String), i32>,
) -> i32 {
	// DFS
	let mut best_pressure = 0;
	// visited, current, elapsed, pressure
	let mut stack = vec![(HashSet::new(), "AA", 0, 0)];
	while let Some((visited, current, elapsed, pressure)) = stack.pop() {
		for next in valves {
			if visited.contains(next) {
				continue;
			}

			let mut new_visited = visited.clone();
			new_visited.insert(next.clone());
			let new_elapsed = elapsed + distances[&(current.to_string(), next.clone())] + 1;

			best_pressure = best_pressure.max(pressure);

			if new_elapsed >= TIME {
				continue;
			}

			stack.push((
				new_visited,
				next,
				new_elapsed,
				pressure + scans[next].flow_rate * (TIME - new_elapsed),
			));
		}
	}
	best_pressure
}

pub fn criterion_benchmark(c: &mut Criterion) {
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

	let mut distances = HashMap::new();
	for i in 0..valves.len() {
		for j in 0..valves.len() {
			distances.insert((i, j), adj_matrix[[i, j]]);
		}
	}

	let mut string_key_distances = HashMap::new();
	for i in 0..valves.len() {
		for j in 0..valves.len() {
			string_key_distances.insert((valves[i].clone(), valves[j].clone()), adj_matrix[[i, j]]);
		}
	}

	c.bench_function("adjacency matrix number visited", |b| {
		b.iter(|| get_pressure_adj_matrix_number_visited(&flow_rates, &adj_matrix));
	});

	c.bench_function("adjacency matrix bitarray visited", |b| {
		b.iter(|| get_pressure_adj_matrix_bitarray_visited(&flow_rates, &adj_matrix));
	});

	c.bench_function("adjacency matrix array visited", |b| {
		b.iter(|| get_pressure_adj_matrix_array_visited(&flow_rates, &adj_matrix));
	});

	c.bench_function("adjacency matrix hash set visited", |b| {
		b.iter(|| get_pressure_adj_matrix_hash_set_visited(&flow_rates, &adj_matrix));
	});

	c.bench_function("adjacency matrix vec visited", |b| {
		b.iter(|| get_pressure_adj_matrix_vec_visited(&flow_rates, &adj_matrix));
	});

	c.bench_function("usize key hashmap bitarray visited", |b| {
		b.iter(|| get_pressure_hashmap(&flow_rates, &distances, 0));
	});

	c.bench_function("string key hashmap hash set visited", |b| {
		b.iter(|| get_pressure_string_hashmap(&valves, &scans, &string_key_distances));
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
