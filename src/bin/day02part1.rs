use std::fs;

use itertools::Itertools;

fn main() {
    let input_file ="inputs/day02.txt";

    let contents = fs::read_to_string(input_file).unwrap();

    let mut n_safe_reports: u32 = 0;

    for line in contents.lines() {
        let mut diffs = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .tuple_windows()
            .map(|(a,b)| a - b)
            .peekable();

        let incdec = *diffs.peek().unwrap() > 0; //true implies strictly decreasing, false implies (not strictly) increasing
        
        let diff_checker = move |d: i32| {
            d != 0 && (d > 0) == incdec && d.abs() <= 3
        };

        let mut safe = true;
        for diff in diffs {
            if !diff_checker(diff) {
                safe = false;
                break;
            }
        } 
        if safe {
            n_safe_reports += 1;
        }
    }

    println!("The number of safe reports is {n_safe_reports}");
}