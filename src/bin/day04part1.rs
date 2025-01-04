use regex::Regex;

fn main() {
    let input_file = "inputs/day04.txt";
    let content = std::fs::read_to_string(input_file).unwrap();

    let matrix = read_string_to_matrix(&content);
    println!(
        "The number of rows is {} and the number of columns is {}",
        matrix.len(),
        matrix[0].len()
    );
    //It is a square matrix
    let n = matrix.len();

    let horizontal_strings: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    let mut vertical_strings = Vec::new();

    for j in 0..n {
        let mut s = String::with_capacity(n);
        for i in 0..n {
            s.push(matrix[i][j]);
        }
        vertical_strings.push(s);
    }

    let mut primary_diagonal_strings = Vec::new();

    for start_col in 0..n {
        let mut s = String::with_capacity(n); //max size of a diagonal is n, let's avoid extra allocations
        let mut i = 0;
        let mut j = start_col;
        while i < n && j < n {
            s.push(matrix[i][j]);
            i += 1;
            j += 1;
        }
        primary_diagonal_strings.push(s);
    }

    for start_row in 1..n {
        let mut s = String::with_capacity(n);
        let mut i = start_row;
        let mut j = 0;
        while i < n && j < n {
            s.push(matrix[i][j]);
            i += 1;
            j += 1;
        }
        primary_diagonal_strings.push(s);
    }

    let mut secondary_diagonal_strings = Vec::new();

    for start_col in 0..n {
        let mut s = String::with_capacity(n);
        let mut i = 0;
        let mut j = start_col as i32;
        while i < n && j >= 0 {
            s.push(matrix[i][j as usize]);
            i += 1;
            j -= 1;
        }
        secondary_diagonal_strings.push(s);
    }

    for start_row in 1..n {
        let mut s = String::with_capacity(n);
        let mut i = start_row;
        let mut j = (n - 1) as i32;
        while i < n && j >= 0 {
            s.push(matrix[i][j as usize]);
            i += 1;
            j -= 1;
        }
        secondary_diagonal_strings.push(s);
    }

    let result = count_xmas(&horizontal_strings)
        + count_xmas(&vertical_strings)
        + count_xmas(&primary_diagonal_strings)
        + count_xmas(&secondary_diagonal_strings);

    println!("The result is {result}");
}

fn read_string_to_matrix(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|s| s.chars().collect()).collect()
}

fn count_xmas(vec: &Vec<String>) -> usize {
    let rgx1 = Regex::new(r"XMAS").unwrap();
    let rgx2 = Regex::new(r"SAMX").unwrap();

    vec.iter()
        .map(|s| rgx1.find_iter(s).count() + rgx2.find_iter(s).count())
        .sum()
}
