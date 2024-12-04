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

    fn find<S: AsRef<str>>(&self, word: S) -> usize {
        let word = word.as_ref().chars().collect::<Vec<_>>();
        assert!(word.len() > 0);

        // left, right, down, up
        let r = (0..self.height)
            .map(|row| self.find_line(&word, row, 0, 0, 1))
            .sum::<usize>();
        let l = (0..self.height)
            .map(|row| self.find_line(&word, row, self.width - 1, 0, -1))
            .sum::<usize>();
        let d = (0..self.width)
            .map(|col| self.find_line(&word, 0, col, 1, 0))
            .sum::<usize>();
        let u = (0..self.width)
            .map(|col| self.find_line(&word, self.height - 1, col, -1, 0))
            .sum::<usize>();

        // IDEA: don't even try diagonals that are too short for the word

        // down right
        let dr = ((0..self.height).map(|row| (row, 0)))
            .chain((1..self.width).map(|col| (0, col)))
            .map(|(row, col)| self.find_line(&word, row, col, 1, 1))
            .sum::<usize>();

        // down left
        let dl = ((0..self.height).map(|row| (row, self.width - 1)))
            .chain((0..(self.width - 1)).map(|col| (0, col)))
            .map(|(row, col)| self.find_line(&word, row, col, 1, -1))
            .sum::<usize>();

        // up right
        let ur = ((0..self.height).map(|row| (row, 0)))
            .chain((1..self.width).map(|col| (self.height - 1, col)))
            .map(|(row, col)| self.find_line(&word, row, col, -1, 1))
            .sum::<usize>();

        // up left
        let ul = ((0..self.height).map(|row| (row, self.width - 1)))
            .chain((0..(self.width - 1)).map(|col| (self.height - 1, col)))
            .map(|(row, col)| self.find_line(&word, row, col, -1, -1))
            .sum::<usize>();

        r + l + d + u + dr + dl + ur + ul
    }

    fn find_line(
        &self,
        word: &[char],
        mut row: usize,
        mut col: usize,
        drow: isize,
        dcol: isize,
    ) -> usize {
        let drow = drow as usize;
        let dcol = dcol as usize;

        let mut occurrences = 0;
        let mut i = 0;
        loop {
            // only have to check for greater than bounds because negative numbers
            // would automatically wrap around because of usize
            if row >= self.height || col >= self.width {
                break;
            }

            if self.get(row, col) == word[i] {
                i += 1;
            } else if self.get(row, col) == word[0] {
                // check if the incorrect character is the
                // starting character for the next try
                i = 1;
            } else {
                i = 0;
            }

            if i >= word.len() {
                occurrences += 1;
                i = 0;
            }

            // negative deltas will simply become very high numbers and should wrap around
            // resulting in the correct result anyways
            row += drow;
            col += dcol;
        }

        occurrences
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

    word_search.find("XMAS")
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

        assert_eq!(solve(&input), 18);
    }
}
