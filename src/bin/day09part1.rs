use Block::*;

fn main() {
    let contents = std::fs::read_to_string("inputs/day09.txt").unwrap();

    let mut file_index = 0;
    let mut disk = contents
        .trim()
        .char_indices()
        .flat_map(move |(index, c)| {
            let count = c.to_digit(10).unwrap() as usize;
            if index % 2 == 0 {
                let iter = std::iter::repeat_n(Block::File(file_index), count);
                file_index += 1;
                iter
            } else {
                std::iter::repeat_n(Block::Empty, count)
            }
        })
        .collect::<Vec<_>>();

    let mut a = 0;
    let mut b = disk.len() - 1;

    while a < b {
        match disk[a] {
            File(_) => {
                a += 1;
            }
            Empty => {
                disk.swap(a, b);
                b = nearest_file_block(b, &disk);
            }
        }
    }

    let checksum: usize = disk
        .iter()
        .enumerate()
        .map(|(i, b)| match b {
            &Empty => 0,
            &File(id) => id * i,
        })
        .sum();

    println!("The checksum is {}", checksum);
}

#[derive(Clone, Copy, Debug)]
enum Block {
    Empty,
    File(usize),
}

fn nearest_file_block(mut b: usize, disk: &Vec<Block>) -> usize {
    loop {
        if let File(_) = disk[b] {
            return b;
        }
        b -= 1;
    }
}
