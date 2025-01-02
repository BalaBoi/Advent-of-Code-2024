fn main() {
    let input_file = "inputs/day04.txt";
    let content = std::fs::read_to_string(input_file).unwrap();
    
    let matrix = read_string_to_matrix(&content);
    let n = matrix.len();

    let mut result = 0;
    for i in 1..n-1 {
        for j in 1..n-1 {
            if is_index_x_mas(&matrix, i, j) {
                result += 1;
            }
        }
    }

    println!("The result is {result}");
}

fn read_string_to_matrix(s: &str) -> Vec<Vec<char>> {
    s
        .lines()
        .map(|s| s.chars().collect())
        .collect()
}

fn is_index_x_mas(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if matrix[i][j] != 'A' {
        return false;
    }
    let diag1 = (matrix[i-1][j-1], matrix[i+1][j+1]);
    if diag1 != ('S', 'M') && diag1 != ('M', 'S') {
        return false;
    }
    let diag2 = (matrix[i-1][j+1], matrix[i+1][j-1]);
    if diag2 != ('S', 'M') && diag2 != ('M', 'S') {
        return false;
    }
    true
}
