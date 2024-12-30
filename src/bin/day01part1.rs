use std::fs;

fn main() {
    let input_file = "inputs/day01.txt";

    let content = fs::read_to_string(input_file).unwrap();
    let n_lines = content.lines().count();
    let mut left_list = Vec::with_capacity(n_lines);
    let mut right_list = Vec::with_capacity(n_lines);

    for line in content.lines() {
        let mut parts = line.split_whitespace();

        let num1 = parts.next().unwrap().parse::<i32>().unwrap();
        let num2 = parts.next().unwrap().parse::<i32>().unwrap();
        left_list.push(num1);
        right_list.push(num2);
    }

    left_list.sort();
    right_list.sort();

    let mut distance = 0;

    for (num1, num2) in left_list.iter().zip(right_list.iter()) {
        distance += num1.abs_diff(*num2);
    }

    println!("The distance between the two lists is {distance}");
}
