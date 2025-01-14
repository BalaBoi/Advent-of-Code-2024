use Block::*;

fn main() {
    let contents = std::fs::read_to_string("inputs/day09.txt").unwrap();

    let mut file_index = 0;
    let mut disk_map = contents
        .trim()
        .char_indices()
        .map(|(index, c)| {
            let count = c.to_digit(10).unwrap() as usize;
            if index % 2 == 0 {
                let out = Blocks::File(file_index, count);
                file_index += 1;
                out
            } else {
                Blocks::Empty(count)
            }
        })
        .collect::<Vec<_>>();

    let mut blocks_index: usize = disk_map.len() - 1;

    while blocks_index > 0 {
        if let Blocks::File(id, size) = disk_map[blocks_index] {
            if let Some((empty_blocks_index, empty_blocks_size)) =
                find_empty_blocks(size, blocks_index, &disk_map)
            {
                disk_map.remove(blocks_index);
                disk_map.insert(blocks_index, Blocks::Empty(size));
                if empty_blocks_size > size {
                    disk_map[empty_blocks_index] = Blocks::Empty(empty_blocks_size - size);
                    blocks_index += 1;
                } else {
                    disk_map.remove(empty_blocks_index);
                }
                disk_map.insert(empty_blocks_index, Blocks::File(id, size));
            }
        }
        blocks_index -= 1;
    }

    let disk = disk_map
        .iter()
        .flat_map(|b| match b {
            &Blocks::Empty(size) => std::iter::repeat_n(Empty, size),
            &Blocks::File(id, size) => std::iter::repeat_n(File(id), size),
        })
        .collect::<Vec<_>>();

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
enum Blocks {
    Empty(usize),       //contains the size
    File(usize, usize), //id followed by the size
}

#[derive(Clone, Copy, Debug)]
enum Block {
    Empty,
    File(usize),
}

fn find_empty_blocks(
    min_size: usize,
    max_index: usize,
    disk_map: &Vec<Blocks>,
) -> Option<(usize, usize)> {
    for (index, blocks) in disk_map.iter().enumerate().take(max_index) {
        match blocks {
            &Blocks::Empty(size) if size >= min_size => return Some((index, size)),
            _ => continue,
        }
    }
    None
}
