use std::collections::{HashMap, HashSet};

fn main() {
    let input_file = "inputs/day08.txt";
    let contents = std::fs::read_to_string(input_file).unwrap();

    let grid: Vec<Vec<char>> = contents.lines().map(|s| s.chars().collect()).collect();
    let n_rows = grid.len();
    let n_columns = grid[0].len();

    let mut tower_locations: HashMap<char, Vec<Pos>> = HashMap::new();
    for (c, pos) in grid.iter().enumerate().flat_map(|(i, v)| {
        v.iter().enumerate().filter_map(move |(j, c)| match c {
            '.' => None,
            c => Some((*c, Pos(i as i32, j as i32))),
        })
    }) {
        tower_locations
            .entry(c)
            .and_modify(|v| v.push(pos))
            .or_insert(vec![pos]);
    }

    let mut unique_locations = HashSet::new();
    for locs in tower_locations.values() {
        for antinode in combinations_of_2_pos(&locs).flat_map(|(pos1, pos2)| antinodes(pos1, pos2))
        {
            if antinode.within_grid(n_rows, n_columns) && !unique_locations.contains(&antinode) {
                unique_locations.insert(antinode);
            }
        }
    }

    println!(
        "The number of unique locations is {}",
        unique_locations.len()
    );
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Pos(i32, i32);

impl Pos {
    fn within_grid(&self, n_rows: usize, n_columns: usize) -> bool {
        0 <= self.0
            && self.0 < n_rows.try_into().unwrap()
            && 0 <= self.1
            && self.1 < n_columns.try_into().unwrap()
    }
}

fn combinations_of_2_pos(positions: &Vec<Pos>) -> impl Iterator<Item = (Pos, Pos)> + use<'_> {
    positions
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(i, pos1)| std::iter::repeat(pos1).zip(positions.iter().skip(i + 1).copied()))
}

fn antinodes(pos1: Pos, pos2: Pos) -> [Pos; 2] {
    let Pos(a1, b1) = pos1;
    let Pos(a2, b2) = pos2;
    [Pos(2 * a1 - a2, 2 * b1 - b2), Pos(2 * a2 - a1, 2 * b2 - b1)]
}
