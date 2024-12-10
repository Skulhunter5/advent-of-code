use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("../inputs/09-input").unwrap();

    let (part1, part2) = solve(&input);
    println!("Part 1: {}", &part1);
    println!("Part 2: {}", &part2);
}

type IDType = usize;
type FileSize = usize;

#[derive(Debug, Copy, Clone)]
struct File {
    pos: usize,
    size: FileSize,
}

#[derive(Debug, Clone)]
struct MemoryMap {
    // part 1
    blocks: Vec<Option<IDType>>,
    // part 2
    mem: Vec<Option<IDType>>,
    files: Vec<File>,
}

#[derive(Debug)]
struct ParseMemoryMapError;

impl FromStr for MemoryMap {
    type Err = ParseMemoryMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = Vec::new();
        let mut mem = Vec::new();
        let mut files = Vec::new();

        let mut pos: usize = 0;

        let mut is_file = true;
        for c in s.chars() {
            if !c.is_ascii_digit() {
                return Err(ParseMemoryMapError);
            }

            let size = c as FileSize - '0' as FileSize;
            if is_file {
                let id = files.len();
                files.push(File { pos, size });
                for _ in 0..size {
                    mem.push(Some(id as IDType));
                    blocks.push(Some(id as IDType));
                }
            } else {
                for _ in 0..size {
                    mem.push(None);
                    blocks.push(None);
                }
            }

            is_file = !is_file;
            pos += size as usize;
        }

        Ok(Self { blocks, mem, files })
    }
}

impl MemoryMap {
    pub fn solve(mut self) -> (usize, usize) {
        self.compact_by_blocks();
        self.compact_by_files();
        (self.checksum1(), self.checksum2())
    }

    fn compact_by_blocks(&mut self) {
        let mut i = 0;
        let mut j = self.blocks.len() - 1;
        'outer: loop {
            // Find next empty block (front to back)
            while i < j && self.blocks[i].is_some() {
                i += 1;
            }
            // Find next file block (back to front)
            loop {
                if j <= i {
                    break 'outer;
                }
                if self.blocks[j].is_some() {
                    break;
                }
                j -= 1;
            };

            self.blocks.swap(i, j);
        }

        self.blocks.resize(i, None);
    }

    fn compact_by_files(&mut self) {
        let mut first_empty = 0;

        'outer: for id in (0..self.files.len()).rev() {
            let (file_pos, file_size) = {
                let file = &self.files[id];
                (file.pos, file.size as usize)
            };

            let mut i = first_empty;
            while self.mem[i].is_some() {
                i += 1;
            }
            first_empty = i;
            let empty_pos = loop {
                while self.mem[i].is_some() {
                    i += 1;
                }

                if i + file_size > file_pos {
                    continue 'outer;
                }

                let empty_pos = i;
                let mut size = 0;
                while self.mem[i].is_none() && size < file_size {
                    i += 1;
                    size += 1;
                }
                if size == file_size {
                    break empty_pos;
                }
            };

            self.files[id].pos = empty_pos;
            for i in file_pos..(file_pos + file_size) {
                self.mem[i] = None;
            }
            for i in empty_pos..(empty_pos + file_size) {
                self.mem[i] = Some(id as IDType);
            }
        }
    }

    fn checksum1(&self) -> usize {
        let mut checksum = 0;
        for (i, block) in self.blocks.iter().enumerate() {
            if let Some(id) = block {
                checksum += i * *id as usize;
            }
        }

        checksum
    }

    fn checksum2(&self) -> usize {
        let mut checksum = 0;
        for (i, block) in self.mem.iter().enumerate() {
            if let Some(id) = block {
                checksum += i * *id as usize;
            }
        }

        checksum
    }
}

fn solve(input: &String) -> (usize, usize) {
    let start = std::time::Instant::now();

    let mmap = MemoryMap::from_str(&input.trim()).expect("invalid input");
    
    let checksums = mmap.solve();

    let time = start.elapsed();
    println!("Time: {:?}", &time);

    checksums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "2333133121414131402".to_string();

        assert_eq!(solve(&input), (1928, 2858));
    }
}
