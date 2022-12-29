#![allow(clippy::all)]
#![allow(clippy::pedantic)]

use std::{mem, os::fd::AsRawFd};

const INPUT_BUF_SIZE: usize = 10_000;
const CALORIES_SIZE: usize = 250;

/// I want to test out some unsafe code, so I'll abuse it in this file.
/// My actual solution is in the part2.rs file.
fn main() {
	unsafe {
		let input_file = std::fs::File::open("./01/input.txt").unwrap();
		let input_buf = libc::malloc(INPUT_BUF_SIZE * mem::size_of::<u8>()) as *mut u8;
		let read_len = libc::read(
			input_file.as_raw_fd(),
			input_buf as *mut libc::c_void,
			INPUT_BUF_SIZE,
		);
		if read_len == -1 {
			panic!("Error reading input file");
		}
		let input_len = read_len;

		let calories = libc::malloc(CALORIES_SIZE * mem::size_of::<u32>()) as *mut u32;
		let mut cal_len = 0;
		let mut cur_calories = 0;
		let mut str = input_buf;
		let str_end = input_buf.offset(input_len);
		loop {
			str = str.offset(1);
			if *str >= b'0' && *str <= b'9' {
				cur_calories *= 10;
				cur_calories += *str as u32 - 48;
			} else if *str == b'\n' && *str.offset(-1) == b'\n' {
				cal_len += 1;
				cur_calories = 0;
			} else if *str == b'\n' {
				*calories.offset(cal_len) += cur_calories;
				cur_calories = 0;
			}

			if str == str_end {
				break;
			}
		}
		libc::free(input_buf as *mut libc::c_void);

		for i in 0..3 {
			let mut max = 0;
			let mut max_idx = 0;
			for j in i..cal_len {
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

		libc::free(calories as *mut libc::c_void);
	}
}
