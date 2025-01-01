use std::fs;

use itertools::Itertools;

fn main() {
    let input_file = "inputs/day02.txt";

    let contents = fs::read_to_string(input_file).unwrap();

    let mut n_safe_reports: u32 = 0;

    for line in contents.lines() {
        let diffs = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if is_naive_safe(&diffs) {
            n_safe_reports += 1;
            continue;
        }

        for index in 0..diffs.len() {
            if is_naive_safe(&remove_at_index(&diffs, index)) {
                n_safe_reports += 1;
                break;
            }
        }
    }

    println!("The number of safe reports is {n_safe_reports}");
}

fn is_naive_safe(levels: &[i32]) -> bool {
    let diffs = levels
        .iter()
        .tuple_windows()
        .map(|(a, b)| a - b)
        .collect::<Vec<_>>();
    let incdec = diffs[0] > 0;
    let diff_checker = move |d: i32| d != 0 && (d > 0) == incdec && d.abs() <= 3;
    for diff in diffs {
        if !diff_checker(diff) {
            return false;
        }
    }
    true
}

fn remove_at_index(slice: &[i32], index: usize) -> Vec<i32> {
    let mut out = Vec::with_capacity(slice.len() - 1);
    out.extend_from_slice(&slice[..index]);
    out.extend_from_slice(&slice[index + 1..]);
    out
}
