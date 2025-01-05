fn main() {
    let input_file = "inputs/day06.txt";
    let contents = std::fs::read_to_string(input_file).unwrap();
    //     let contents = r##"....#.....
    // .........#
    // ..........
    // ..#.......
    // .......#..
    // ..........
    // .#..^.....
    // ........#.
    // #.........
    // ......#..."##;
    let mut map = Map {
        grid: Vec::new(),
        n_rows: 0,
        n_columns: 0,
    };
    let mut guard_pos = None;
    for (row, line) in contents.lines().enumerate() {
        map.grid.push(
            line.chars()
                .enumerate()
                .map(|(column, c)| match c {
                    '.' => Square::Empty { visited: false },
                    '#' => Square::Obstacle,
                    '^' => {
                        guard_pos = Some(Pos(row as i32, column as i32));
                        Square::Empty { visited: true }
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>(),
        );
    }

    map.n_rows = map.grid.len();
    map.n_columns = map.grid[0].len();

    let mut n_visited = 1;

    move_guard(guard_pos.unwrap(), &mut map, &mut n_visited);

    println!("Number of visited positions are {}", n_visited);
}

enum Square {
    Obstacle,
    Empty { visited: bool },
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug)]
struct Pos(i32, i32);

impl Pos {
    fn next(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Pos(self.0 - 1, self.1),
            Direction::Right => Pos(self.0, self.1 + 1),
            Direction::Down => Pos(self.0 + 1, self.1),
            Direction::Left => Pos(self.0, self.1 - 1),
        }
    }
}

struct Map {
    grid: Vec<Vec<Square>>,
    n_rows: usize,
    n_columns: usize,
}

impl Map {
    fn is_within_map(&self, pos: Pos) -> bool {
        0 <= pos.0
            && pos.0 < self.n_rows.try_into().unwrap()
            && 0 <= pos.1
            && pos.1 < self.n_columns.try_into().unwrap()
    }

    fn visit(&mut self, pos: Pos, n_visited: &mut usize) {
        if let Square::Empty { visited } = self.grid[pos.0 as usize][pos.1 as usize] {
            if !visited {
                self.grid[pos.0 as usize][pos.1 as usize] = Square::Empty { visited: true };
                *n_visited += 1;
            }
        } else {
            unreachable!();
        }
    }

    fn is_obstacle(&self, pos: Pos) -> bool {
        if let Square::Obstacle = self.grid[pos.0 as usize][pos.1 as usize] {
            true
        } else {
            false
        }
    }
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn move_guard(guard_pos: Pos, map: &mut Map, n_visited: &mut usize) {
    let mut pos_opt = Some((guard_pos, Direction::Up));
    while let Some((pos, dir)) = pos_opt {
        // dbg!(&pos_opt);
        let new_pos = pos.next(dir);
        if map.is_within_map(new_pos) {
            if map.is_obstacle(new_pos) {
                pos_opt = Some((pos, dir.turn_right()));
            } else {
                map.visit(new_pos, n_visited);
                pos_opt = Some((new_pos, dir));
            }
        } else {
            pos_opt = None;
        }
    }
}
