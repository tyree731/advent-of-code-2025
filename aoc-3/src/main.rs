mod aoc;
use aoc::{aoc_3_1, aoc_3_2};

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
                "error: aoc-3: unknown --part argument, must be 1 or 2: {}",
                args[1]
            );
        }
    }
    let result = read_stdin_lines_until_eof();
    match result {
        Ok(lines) => {
            if part == 1 {
                println!("aoc-3-1 answer: {}", aoc_3_1(&lines))
            } else if part == 2 {
                println!("aoc-3-2 answer: {}", aoc_3_2(&lines))
            }
        }
        Err(e) => panic!("error: aoc-3: {}", e),
    }
}
