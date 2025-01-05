use std::collections::HashSet;

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
                    '.' => Square::Empty {
                        visited: false,
                        visited_directions: None,
                    },
                    '#' => Square::Obstacle,
                    '^' => {
                        guard_pos = Some(Pos(row as i32, column as i32));
                        Square::Empty {
                            visited: true,
                            visited_directions: Some(HashSet::from([Direction::Up])),
                        }
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>(),
        );
    }

    map.n_rows = map.grid.len();
    map.n_columns = map.grid[0].len();

    let guard_pos = guard_pos.unwrap();
    let mut visited_vec = Vec::new();

    move_guard(guard_pos, &mut map.clone(), &mut visited_vec);

    let mut result = 0;
    for visited_pos in visited_vec {
        if obstacle_makes_loop(visited_pos, guard_pos, &map) {
            result += 1;
        }
    }

    println!("The result is {}", result);
}

#[derive(Clone)]
enum Square {
    Obstacle,
    Empty {
        visited: bool,
        visited_directions: Option<HashSet<Direction>>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone)]
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

    fn visit(&mut self, pos: Pos, dir: Direction, visited_vec: &mut Vec<Pos>) {
        if let Square::Empty {
            ref mut visited,
            ref mut visited_directions,
        } = self.grid[pos.0 as usize][pos.1 as usize]
        {
            if !*visited {
                *visited = true;
                *visited_directions = Some(HashSet::from([dir]));
                visited_vec.push(pos);
            } else if !visited_directions.as_ref().unwrap().contains(&dir) {
                visited_directions.as_mut().unwrap().insert(dir);
            }
        } else {
            unreachable!();
        }
    }

    fn visit_without_vec(&mut self, pos: Pos, dir: Direction) {
        if let Square::Empty {
            ref mut visited,
            ref mut visited_directions,
        } = self.grid[pos.0 as usize][pos.1 as usize]
        {
            if !*visited {
                *visited = true;
                *visited_directions = Some(HashSet::from([dir]));
            } else if !visited_directions.as_ref().unwrap().contains(&dir) {
                visited_directions.as_mut().unwrap().insert(dir);
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

    fn make_obstacle(&mut self, pos: Pos) {
        self.grid[pos.0 as usize][pos.1 as usize] = Square::Obstacle;
    }

    fn is_loop(&self, pos: Pos, dir: Direction) -> bool {
        if let Square::Empty {
            visited,
            ref visited_directions,
        } = self.grid[pos.0 as usize][pos.1 as usize]
        {
            if visited && visited_directions.as_ref().unwrap().contains(&dir) {
                true
            } else {
                false
            }
        } else {
            unreachable!()
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

fn move_guard(guard_pos: Pos, map: &mut Map, visited_vec: &mut Vec<Pos>) {
    let mut pos_opt = Some((guard_pos, Direction::Up));
    while let Some((pos, dir)) = pos_opt {
        // dbg!(&pos_opt);
        let new_pos = pos.next(dir);
        if map.is_within_map(new_pos) {
            if map.is_obstacle(new_pos) {
                pos_opt = Some((pos, dir.turn_right()));
            } else {
                map.visit(new_pos, dir, visited_vec);
                pos_opt = Some((new_pos, dir));
            }
        } else {
            pos_opt = None;
        }
    }
}

fn obstacle_makes_loop(obstacle_pos: Pos, guard_pos: Pos, map: &Map) -> bool {
    let mut map = map.clone();
    map.make_obstacle(obstacle_pos);
    let (mut pos, mut dir) = (guard_pos, Direction::Up);
    loop {
        let pos_step = pos.next(dir);
        if map.is_within_map(pos_step) {
            (pos, dir) = if map.is_obstacle(pos_step) {
                (pos, dir.turn_right())
            } else {
                (pos_step, dir)
            };
            if map.is_loop(pos, dir) {
                return true;
            }
            map.visit_without_vec(pos, dir);
        } else {
            break;
        }
    }
    false
}
