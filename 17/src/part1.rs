#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

fn parse_rock(input: &str) -> (usize, Vec<u8>) {
	let rock = input
		.lines()
		.map(|line| {
			let mut arr = 0_u8;
			for (i, b) in line.bytes().enumerate() {
				arr.set(i, b == b'#');
			}
			arr
		})
		.rev()
		.collect::<Vec<u8>>();

	let rock_width = input.lines().next().unwrap().len();
	(rock_width, rock)
}
trait BitOps {
	fn set(&mut self, pos: usize, val: bool);
	fn get(&self, pos: usize) -> bool;
}

impl BitOps for u8 {
	fn set(&mut self, pos: usize, val: bool) {
		*self = (*self & !(1 << pos)) | (Self::from(val) << pos);
	}

	fn get(&self, pos: usize) -> bool {
		(self >> pos) & 1 == 1
	}
}

const MAP_WIDTH: usize = 7;
const ROCK_COUNT: usize = 2022;

fn rock_overlaps(rock: &[u8], map: &[u8], pos: (usize, usize)) -> bool {
	rock.iter()
		.enumerate()
		.any(|(i, &r)| map[pos.1 + i] & r << pos.0 != 0)
}

fn add_rock_to_map(rock: &[u8], map: &mut [u8], pos: (usize, usize)) {
	for (i, &r) in rock.iter().enumerate() {
		map[pos.1 + i] |= r << pos.0;
	}
}

fn highest_rock_pos(map: &[u8]) -> usize {
	for (i, &m) in map.iter().enumerate().rev() {
		if m != 0 {
			return i;
		}
	}
	unreachable!();
}

fn main() {
	let input = include_str!("../input.txt");
	let rocks = include_str!("../rocks.txt")
		.split("\n\n")
		.map(parse_rock)
		.collect::<Vec<_>>();

	let mut jets = input.as_bytes().iter().cycle();

	// Bottom y coordinate is floor
	let mut map = vec![u8::MAX; 1];

	for (rock_width, rock) in rocks.iter().cycle().take(ROCK_COUNT) {
		let highest_rock_pos = highest_rock_pos(&map);
		let mut rock_pos: (usize, usize) = (2, highest_rock_pos + 4);

		if rock_pos.1 + rock.len() > map.len() {
			map.resize(rock_pos.1 + rock.len(), 0_u8);
		}

		loop {
			// Horizontal movement because of jet
			let jet = jets.next().unwrap();
			let rock_pos_jet = match jet {
				b'<' => (rock_pos.0.saturating_sub(1), rock_pos.1),
				b'>' => ((rock_pos.0 + 1).min(MAP_WIDTH - rock_width), rock_pos.1),
				_ => unreachable!(),
			};

			if !rock_overlaps(rock, &map, rock_pos_jet) {
				rock_pos = rock_pos_jet;
			}

			let rock_pos_grav = (rock_pos.0, rock_pos.1 - 1);

			if rock_overlaps(rock, &map, rock_pos_grav) {
				add_rock_to_map(rock, &mut map, rock_pos);
				break;
			}
			rock_pos = rock_pos_grav;
		}
	}

	let tower_height = highest_rock_pos(&map);

	println!("Answer: {tower_height}");
}
