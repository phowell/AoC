use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|x| x.chars().collect()).collect()
}

fn first_digit(input: &str) -> char {
    input
        .chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<Vec<char>>()[0]
}

fn last_digit(input: &str) -> char {
    input
        .chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<Vec<char>>()
        .pop()
        .unwrap()
}

fn fix_nums(mut input: String) -> String {
    for num in [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero",
    ] {
        input = input.replace(
            num,
            match num {
                "one" => "o1e",
                "two" => "t2o",
                "three" => "t3e",
                "four" => "f4r",
                "five" => "f5e",
                "six" => "s6x",
                "seven" => "s7n",
                "eight" => "e8t",
                "nine" => "n9e",
                "zero" => "z0o",
                _ => unreachable!(),
            },
        )
    }
    input
}

fn chars_to_int(first: char, last: char) -> u32 {
    format!("{}{}", first, last).parse::<u32>().unwrap()
}

#[aoc(day1, part1)]
pub fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|x| chars_to_int(first_digit(x), last_digit(x)))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[String]) -> u32 {
    input
        .iter()
        .map(|x| fix_nums(x.to_string()))
        .map(|x| chars_to_int(first_digit(x.as_str()), last_digit(x.as_str())))
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static EX1: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    static EX2: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        11
    "};

    #[test]
    fn example_part1() {
        assert_eq!(part1(&input_generator(EX1)), 142);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&input_generator(EX2)), 292);
    }
}
