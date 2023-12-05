use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day04)]
pub fn input_generator(input: &str) -> Vec<Ticket> {
    input
        .lines()
        .map(|x| x.split_once(':').unwrap())
        .map(|y| (y.0, y.1.split_once('|').unwrap()))
        .map(|z| Ticket::new(z.0, (z.1).0, (z.1).1))
        .collect()
}

pub struct Ticket {
    card: usize,
    wins: Vec<usize>,
    nums: Vec<usize>,
}

impl Ticket {
    fn new(g: &str, w: &str, n: &str) -> Ticket {
        Ticket {
            card: g.split_whitespace().collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap(),
            wins: w
                .split_whitespace()
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect(),
            nums: n
                .split_whitespace()
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect(),
        }
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for n in &self.nums {
            if self.wins.contains(n) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        score
    }

    fn winnings(&self) -> usize {
        let mut wins = 0;
        for n in &self.nums {
            if self.wins.contains(n) {
                wins += 1;
            }
        }
        wins
    }
}
#[aoc(day04, part1)]
pub fn part1(input: &[Ticket]) -> usize {
    input.iter().map(|m| m.score()).sum()
}

#[aoc(day04, part2)]
pub fn part2(input: &Vec<Ticket>) -> usize {
    let mut counts = input
        .iter()
        .map(|t| (t.card, 1))
        .collect::<HashMap<usize, usize>>();

    for i in 1..=input.len() {
        let w = input
            .iter()
            .filter(|t| t.card == i)
            .collect::<Vec<&Ticket>>()[0]
            .winnings();
        if w > 0 {
            for j in 1..=w {
                counts.insert(&i + &j, counts[&(&i + &j)] + counts[&i]);
            }
        }
    }
    counts.values().sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
	"};

    #[test]
    fn example_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 13);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 30);
    }
}
