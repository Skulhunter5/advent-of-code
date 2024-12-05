#[derive(Debug)]
struct WordSearch {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl WordSearch {
    #[inline]
    fn get(&self, row: usize, col: usize) -> char {
        self.data[row][col]
    }

    fn find_xmas(&self) -> usize {
        (0..(self.height - 2))
            .flat_map(|row| (0..(self.width - 2)).map(move |col| (row, col)))
            .map(|(row, col)| self.check_at(row, col) as usize)
            .sum()
    }

    fn check_at(&self, row: usize, col: usize) -> bool {
        if self.get(row + 1, col + 1) != 'A' {
            return false;
        }

        match self.get(row, col) {
            'M' => match self.get(row, col + 2) {
                'M' => self.get(row + 2, col) == 'S' && self.get(row + 2, col + 2) == 'S', // top
                'S' => self.get(row + 2, col) == 'M' && self.get(row + 2, col + 2) == 'S', // left
                _ => false,
            }
            'S' => match self.get(row, col + 2) {
                'M' => self.get(row + 2, col) == 'S' && self.get(row + 2, col + 2) == 'M', // right
                'S' => self.get(row + 2, col) == 'M' && self.get(row + 2, col + 2) == 'M', // bottom
                _ => false,
            }
            _ => false,
        }
    }
}

impl TryFrom<&String> for WordSearch {
    type Error = &'static str;

    fn try_from(text: &String) -> Result<Self, Self::Error> {
        let data = text
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let height = data.len();
        let width = data[0].len();

        for line in &data {
            if line.len() != width {
                return Err("invalid word search: not a square");
            }
        }

        Ok(Self {
            data,
            height,
            width,
        })
    }
}

fn main() {
    let input = std::fs::read_to_string("../inputs/04-input").unwrap();

    println!("{}", solve(&input));
}

fn solve(input: &String) -> usize {
    let word_search = WordSearch::try_from(input).unwrap();

    word_search.find_xmas()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string();

        assert_eq!(solve(&input), 9);
    }
}
