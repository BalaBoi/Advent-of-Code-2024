use std::{fs, ops::Mul};

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

    let mut right_index = 0;
    let mut similarity_score = 0;
    for left_num in &left_list {
        while right_index < n_lines && *left_num > right_list[right_index] { right_index +=1 }
        let mut count = 0;
        while right_index < n_lines && *left_num == right_list[right_index] { 
            count += 1;
            right_index += 1;
        }
        similarity_score += left_num.mul(count);
        if right_index == n_lines {
            break;
        }
    }

    println!("The similarity score is {similarity_score}")
}