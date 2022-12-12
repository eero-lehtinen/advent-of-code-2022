#![allow(clippy::all)]
#![allow(clippy::pedantic)]

use std::{
	alloc::{alloc_zeroed, dealloc, Layout},
	io::Read,
};

/// I want to test out some unsafe code, so I'll abuse it in this file.
/// My actual solution is in the part2.rs file.
fn main() {
	unsafe {
		let input_layout = Layout::array::<u8>(10_000).unwrap();
		let input = alloc_zeroed(input_layout);
		let input_end = input.offset(10_000);

		let mut input_file = std::fs::File::open("input.txt").unwrap();
		input_file
			.read_exact(std::slice::from_raw_parts_mut(input, 10_000))
			.unwrap();

		let calories_layout = Layout::array::<u32>(250).unwrap();
		let calories = alloc_zeroed(calories_layout) as *mut u32;
		let mut cal_idx = 0;
		let mut cur_calories = 0;
		let mut str = input;
		loop {
			str = str.offset(1);
			if *str >= b'0' && *str <= b'9' {
				cur_calories *= 10;
				cur_calories += *str as u32 - 48;
			} else if *str == b'\n' && *str.offset(-1) == b'\n' {
				cal_idx += 1;
				cur_calories = 0;
			} else if *str == b'\n' {
				*calories.offset(cal_idx) += cur_calories;
				cur_calories = 0;
			}

			if str == input_end {
				break;
			}
		}
		dealloc(input, input_layout);

		for i in 0..3 {
			let mut max = 0;
			let mut max_idx = 0;
			for j in i..250 {
				if *calories.offset(j) > max {
					max = *calories.offset(j);
					max_idx = j;
				}
			}
			let temp = *calories.offset(i);
			*calories.offset(i) = *calories.offset(max_idx);
			*calories.offset(max_idx) = temp;
		}

		println!(
			"Answer: {}",
			*calories + *calories.offset(1) + *calories.offset(2)
		);

		dealloc(calories as *mut u8, calories_layout);
	}
}
