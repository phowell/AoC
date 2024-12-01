use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day07)]
pub fn input_generator(input: &str) -> Vec<&str> {
    let lines = input.lines().collect();
    let times = lines[0].split_whitespace();
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

#[aoc(day07, part1)]
pub fn part1(input: &Vec<&str>) -> u32 {
    todo!();
}

#[aoc(day07, part2)]
pub fn part2(input: &Vec<&str>) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
	"};

    #[test]
    fn example_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 15);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 12);
    }
}
