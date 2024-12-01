use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day01)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|w| w.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day01, part1)]
pub fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let mut score = 0;
    let mut left = Vec::<u32>::new();
    let mut right = Vec::<u32>::new();
    for nums in input {
        left.push(nums[0]);
        right.push(nums[1]);
    }
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .for_each(|(&l, &r)| score += l.abs_diff(r));
    score
}

#[aoc(day01, part2)]
pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let mut score = 0;
    let mut left = Vec::<u32>::new();
    let mut right = Vec::<u32>::new();
    for nums in input {
        left.push(nums[0]);
        right.push(nums[1]);
    }
    for l in left {
        for r in &right {
            if &l == r {
                score += l
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &str = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
	"};

    #[test]
    fn example_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 11);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 31);
    }
}
