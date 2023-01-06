#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::too_many_lines, clippy::cast_possible_truncation)]

use rayon::prelude::*;
use std::str::FromStr;

use lazy_regex::regex_captures;

#[derive(Debug)]
struct Blueprint {
	ore_robot_cost: Resources,
	clay_robot_cost: Resources,
	obsidian_robot_cost: Resources,
	geode_robot_cost: Resources,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Resources {
	ore: u16,
	clay: u16,
	obsidian: u16,
	geodes: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Robots {
	ore: u16,
	clay: u16,
	obsidian: u16,
	geodes: u16,
}

impl Robots {
	const fn new() -> Self {
		Self {
			ore: 1,
			clay: 0,
			obsidian: 0,
			geodes: 0,
		}
	}
}

impl FromStr for Blueprint {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (_, or_r_cost, c_r_cost, ob_r_cost_1, ob_r_cost_2, g_r_cost_1, g_r_cost_2) = regex_captures!(
			"^Blueprint (?:\\d+): Each ore robot costs (\\d+) ore. Each clay robot costs (\\d+) ore. Each obsidian robot costs (\\d+) ore and (\\d+) clay. Each geode robot costs (\\d+) ore and (\\d+) obsidian.$",
			s
		)
		.unwrap();

		Ok(Self {
			ore_robot_cost: Resources {
				ore: or_r_cost.parse().unwrap(),
				..Resources::default()
			},
			clay_robot_cost: Resources {
				ore: c_r_cost.parse().unwrap(),
				..Resources::default()
			},
			obsidian_robot_cost: Resources {
				ore: ob_r_cost_1.parse().unwrap(),
				clay: ob_r_cost_2.parse().unwrap(),
				..Resources::default()
			},
			geode_robot_cost: Resources {
				ore: g_r_cost_1.parse().unwrap(),
				obsidian: g_r_cost_2.parse().unwrap(),
				..Resources::default()
			},
		})
	}
}

const TIME: usize = 32;

fn main() {
	let input = include_str!("../input.txt");
	let blueprints = input
		.lines()
		.take(3)
		.map(|line| line.parse::<Blueprint>().unwrap())
		.collect::<Vec<_>>();

	let result = blueprints
		.into_par_iter()
		.map(|blueprint| {
			let mut most_geodes = 0;

			let mut iters: u64 = 0;

			// Optimization, only 1 robot can be built at a time, so we can't need more robots gathering a resource than the max cost of any robot
			let max_ore_cost = u16::max(
				u16::max(blueprint.ore_robot_cost.ore, blueprint.clay_robot_cost.ore),
				u16::max(
					blueprint.obsidian_robot_cost.ore,
					blueprint.geode_robot_cost.clay,
				),
			);
			let max_clay_cost = blueprint.obsidian_robot_cost.clay;
			let max_obsidian_cost = blueprint.geode_robot_cost.obsidian;

			// DFS
			// elapsed, robots, resources
			let mut stack = vec![(0, Robots::new(), Resources::default())];

			while let Some((elapsed, robots, resources)) = stack.pop() {
				for i in 0..=4 {
					iters += 1;

					let (mut new_resources, new_robots) = match i {
						0 => (resources.clone(), robots.clone()),
						1 => {
							if resources.ore < blueprint.ore_robot_cost.ore
								|| robots.ore >= max_ore_cost
							{
								continue;
							}
							(
								Resources {
									ore: resources.ore - blueprint.ore_robot_cost.ore,
									..resources
								},
								Robots {
									ore: robots.ore + 1,
									..robots
								},
							)
						}
						2 => {
							if resources.ore < blueprint.clay_robot_cost.ore
								|| robots.clay >= max_clay_cost
							{
								continue;
							}
							(
								Resources {
									ore: resources.ore - blueprint.clay_robot_cost.ore,
									..resources
								},
								Robots {
									clay: robots.clay + 1,
									..robots
								},
							)
						}
						3 => {
							if resources.ore < blueprint.obsidian_robot_cost.ore
								|| resources.clay < blueprint.obsidian_robot_cost.clay
								|| robots.obsidian >= max_obsidian_cost
							{
								continue;
							}
							(
								Resources {
									ore: resources.ore - blueprint.obsidian_robot_cost.ore,
									clay: resources.clay - blueprint.obsidian_robot_cost.clay,
									..resources
								},
								Robots {
									obsidian: robots.obsidian + 1,
									..robots
								},
							)
						}
						4 => {
							if resources.ore < blueprint.geode_robot_cost.ore
								|| resources.obsidian < blueprint.geode_robot_cost.obsidian
							{
								continue;
							}
							(
								Resources {
									ore: resources.ore - blueprint.geode_robot_cost.ore,
									obsidian: resources.obsidian
										- blueprint.geode_robot_cost.obsidian,
									..resources
								},
								Robots {
									geodes: robots.geodes + 1,
									..robots
								},
							)
						}
						_ => unreachable!(),
					};

					new_resources.ore += robots.ore;
					new_resources.clay += robots.clay;
					new_resources.obsidian += robots.obsidian;
					new_resources.geodes += robots.geodes;

					let new_elapsed = elapsed + 1;

					if new_elapsed >= TIME {
						most_geodes = most_geodes.max(new_resources.geodes);
						continue;
					}

					// Optimization, stop very bad states, where there is no way to beat the current best
					// even if we create a new geode robot every minute
					let time_left = (TIME - new_elapsed) as u16;
					let ideal_geodes = new_resources.geodes
						+ time_left * new_robots.geodes
						+ (time_left - 1) * (time_left) / 2;

					if ideal_geodes < most_geodes {
						continue;
					}

					stack.push((new_elapsed, new_robots, new_resources));
				}
			}
			println!("iterations: {iters}, most geodes: {most_geodes}");
			u64::from(most_geodes)
		})
		.product::<u64>();

	println!("Answer: {result}");
}
