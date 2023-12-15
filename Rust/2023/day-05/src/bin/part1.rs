fn main() {
    let input = std::fs::read_to_string("../inputs/05-input").unwrap();

    println!("{}", solve(&input));
}

struct MapRange {
    start: i64,
    end: i64,
    off: i64,
}

impl MapRange {
    fn new(dst: i64, src: i64, length: i64) -> Self {
        Self {
            start: src,
            end: src + length,
            off: dst - src,
        }
    }

    fn from_line(line: &str) -> Self {
        let mut iter = line
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap());
        Self::new(iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }

    fn contains(&self, x: i64) -> bool {
        (self.start..self.end).contains(&x)
    }

    fn convert(&self, x: i64) -> i64 {
        x + self.off
    }
}

fn solve(input: &String) -> String {
    let (seeds, blocks) = input.split_once("\n\n").unwrap();
    let seeds = seeds["seeds: ".len()..]
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let blocks = blocks
        .split("\n\n")
        .map(|block| {
            block
                .split("\n")
                .skip(1)
                .filter(|line| !line.is_empty())
                .map(|line| {
                    MapRange::from_line(line)
                }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    blocks.iter()
        .fold(seeds, |acc, maps| {
            acc.iter()
                .map(|x| {
                    maps.iter()
                        .find(|map| map.contains(*x))
                        .map_or(*x, |map| map.convert(*x))
                })
                .collect::<Vec<_>>()
        })
        .iter()
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4".to_string();

        assert_eq!(solve(&input), "35")
    }
}
