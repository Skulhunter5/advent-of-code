fn main() {
    let input = std::fs::read_to_string("../inputs/04-input").unwrap();

    println!("{}", solve(&input));
}

fn solve(input: &String) -> String {
    let cards = input
        .lines()
        .map(|line| {
            // wn = winning number; yn = your number
            // "Card {id}: {wn} {wn} {wn} {wn} {wn} | {yn} {yn} {yn} {yn} {yn} {yn} {yn} {yn}"
            // -> rem="{id}: {wn} {wn} {wn} {wn} {wn} | {yn} {yn} {yn} {yn} {yn} {yn} {yn} {yn}"
            let (_, rem) = line.split_once(' ').unwrap();
            // "{id}: {wn} {wn} {wn} {wn} {wn} | {yn} {yn} {yn} {yn} {yn} {yn} {yn} {yn}"
            // -> rem="{wn} {wn} {wn} {wn} {wn} | {yn} {yn} {yn} {yn} {yn} {yn} {yn} {yn}"
            let (_, rem) = rem.split_once(": ").unwrap();
            // {wn} {wn} {wn} {wn} {wn} | {yn} {yn} {yn} {yn} {yn} {yn} {yn} {yn}
            // -> winning_numbers="{wn} {wn} {wn} {wn} {wn}"
            // -> your_numbers="{yn} {yn} {yn} {yn} {yn} {yn} {yn} {yn}"
            let (winning_numbers, your_numbers) = rem.split_once(" | ").unwrap();
            let winning_numbers = winning_numbers
                .split(' ')
                .filter(|s| s.len() > 0)
                .map(|num| num.parse::<u8>().unwrap())
                .collect::<Vec<_>>();
            let your_numbers = your_numbers
                .split(' ')
                .filter(|s| s.len() > 0)
                .map(|num| num.parse::<u8>().unwrap())
                .collect::<Vec<_>>();
            
            winning_numbers.iter().map(|wn| {
                your_numbers.iter().filter(|yn| yn == &wn).count()
            })
            .sum::<usize>()
        })
        .collect::<Vec<_>>();

    let mut counts = vec![1; cards.len()];
    for i in 0..counts.len() {
        (1..=cards[i]).for_each(|offset| {
            counts[i + offset] += counts[i];
        });
    }
    counts.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn example1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string();
        assert_eq!(solve(&input), "30");
    }
}
