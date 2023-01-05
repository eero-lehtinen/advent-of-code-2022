#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

fn parse_rock(input: &str) -> (usize, Vec<u8>) {
	let rock = input
		.lines()
		.map(|line| {
			let mut arr = 0_u8;
			for (i, b) in line.bytes().enumerate() {
				arr = (arr & !(1 << i)) | (u8::from(b == b'#') << i);
			}
			arr
		})
		.rev()
		.collect::<Vec<u8>>();

	let rock_width = input.lines().next().unwrap().len();
	(rock_width, rock)
}

struct Map {
	map: Vec<u8>,
	rocks: Vec<(usize, (usize, usize))>,
}

impl Map {
	fn new() -> Self {
		Self {
			map: vec![u8::MAX; 1],
			rocks: Vec::new(),
		}
	}

	fn add_rock(&mut self, rock_idx: usize, rock: &[u8], pos: (usize, usize)) {
		for (i, &r) in rock.iter().enumerate() {
			self.map[pos.1 + i] |= r << pos.0;
		}
		self.rocks.push((rock_idx, pos));
	}

	fn rock_overlaps(&self, rock: &[u8], pos: (usize, usize)) -> bool {
		rock.iter()
			.enumerate()
			.any(|(i, &r)| self.map[pos.1 + i] & r << pos.0 != 0)
	}

	fn highest_rock_pos(&self) -> usize {
		for (i, &m) in self.map.iter().enumerate().rev() {
			if m != 0 {
				return i;
			}
		}
		unreachable!();
	}

	/// Returns starting index and height of repeating rocks
	fn find_repeating_part(&self) -> Option<(usize, usize)> {
		// The rocks are first dropped on the floor, so the start may not be included in the repeating part.
		// This is why we search starting from any index 0..rocks.len()/2 (probably inefficient but works).
		'outer: for start in (1..self.rocks.len() / 2).rev() {
			let (bottom, top) = self.rocks[start..].split_at((self.rocks.len() - start) / 2);

			// Get relative positions of rocks
			let bottom_y_min = bottom.first().unwrap().1 .1 as isize;
			let bottom = bottom
				.iter()
				.map(|(i, (x, y))| (i, (x, *y as isize - bottom_y_min)));
			let top_y_min = top.first().unwrap().1 .1 as isize;
			let top = top
				.iter()
				.map(|(i, (x, y))| (i, (x, *y as isize - top_y_min)));

			for (b, t) in bottom.zip(top) {
				if b != t {
					continue 'outer;
				}
			}

			return Some((start, (top_y_min - bottom_y_min) as usize * 2));
		}

		None
	}

	fn ensure_size(&mut self, len: usize) {
		if self.map.len() < len {
			self.map.resize(len, 0_u8);
		}
	}
}

const MAP_WIDTH: usize = 7;
const ROCK_COUNT_TARGET: usize = 1_000_000_000_000;

fn main() {
	let input = include_str!("../input.txt");
	let rocks = include_str!("../rocks.txt")
		.split("\n\n")
		.map(parse_rock)
		.collect::<Vec<_>>();

	let mut jets = input.bytes().cycle();

	// Bottom coordinate is floor
	let mut map = Map::new();
	let mut rock_counter = 0;
	let mut map_y_offset = 0;

	for (rock_idx, (rock_width, rock)) in rocks.iter().enumerate().cycle() {
		let highest_rock_pos = map.highest_rock_pos();
		let mut rock_pos: (usize, usize) = (2, highest_rock_pos + 4);

		map.ensure_size(rock_pos.1 + rock.len());

		loop {
			// Horizontal movement because of jet
			let jet = jets.next().unwrap();

			let rock_pos_jet = match jet {
				b'<' => (rock_pos.0.saturating_sub(1), rock_pos.1),
				b'>' => ((rock_pos.0 + 1).min(MAP_WIDTH - rock_width), rock_pos.1),
				_ => unreachable!(),
			};

			if !map.rock_overlaps(rock, rock_pos_jet) {
				rock_pos = rock_pos_jet;
			}

			let rock_pos_grav = (rock_pos.0, rock_pos.1 - 1);

			if map.rock_overlaps(rock, rock_pos_grav) {
				map.add_rock(rock_idx, rock, rock_pos);
				break;
			}
			rock_pos = rock_pos_grav;
		}

		rock_counter += 1;
		if rock_counter >= ROCK_COUNT_TARGET {
			break;
		}

		if map_y_offset == 0 {
			if let Some((repeat_start, repeat_height)) = map.find_repeating_part() {
				println!("Repeat found: y={repeat_start}, height={repeat_height}");
				let repeat_len = rock_counter - repeat_start;
				let apply_repeats_count = (ROCK_COUNT_TARGET - rock_counter) / repeat_len;
				rock_counter += apply_repeats_count * repeat_len;
				map_y_offset = apply_repeats_count * repeat_height;
			}
		}
	}

	let tower_height = map_y_offset + map.highest_rock_pos();

	println!("Answer: {tower_height}");
}
