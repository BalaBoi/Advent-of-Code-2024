use regex::Regex;

fn main() {
    let input_file = "inputs/day03.txt";
    let contents = std::fs::read_to_string(input_file).unwrap();

    let rgx = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut result = 0;

    for caps in rgx.captures_iter(&contents) {
        let a = caps[1].parse::<u32>().unwrap();
        let b = caps[2].parse::<u32>().unwrap();

        result += a * b;
    }

    println!("The result is {result}");
}
