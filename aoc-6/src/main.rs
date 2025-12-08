mod aoc;
use aoc::{aoc_6_1, aoc_6_2};

use common::read_stdin_lines_until_eof;
use std::env;

fn main() {
    let mut part = 1;
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 && args[1] == "--part" {
        if args[2] == "2" {
            part = 2;
        } else if args[2] == "1" {
            part = 1;
        } else {
            panic!(
                "error: aoc-6: unknown --part argument, must be 1 or 2: {}",
                args[1]
            );
        }
    }
    let result = read_stdin_lines_until_eof();
    match result {
        Ok(contents) => {
            if part == 1 {
                println!("aoc-6: part 1 solution: {}", aoc_6_1(&contents));
            } else if part == 2 {
                println!("aoc-6: part 2 solution: {}", aoc_6_2(&contents));
            }
        }
        Err(e) => panic!("error: aoc-6: {}", e),
    }
}
