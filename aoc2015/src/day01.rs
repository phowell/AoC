use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day01)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|x| if x == '(' { 1 } else { -1 })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    let mut position = 0;
    for (i, floor) in input.iter().enumerate() {
        position += floor;
        if position == -1 {
            return i as i32 + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static EX1: &str = indoc! {"(())"};
    static EX2: &str = indoc! {"()()"};
    static EX3: &str = indoc! {"((("};
    static EX4: &str = indoc! {"(()(()("};
    static EX5: &str = indoc! {"))((((("};
    static EX6: &str = indoc! {"())"};
    static EX7: &str = indoc! {"))("};
    static EX8: &str = indoc! {")))"};
    static EX9: &str = indoc! {")())())"};
    static EX10: &str = indoc! {")"};
    static EX11: &str = indoc! {"()())"};

    #[test]
    fn ex1_part1() {
        assert_eq!(part1(&input_generator(EX1)), 0);
    }

    #[test]
    fn ex2_part1() {
        assert_eq!(part1(&input_generator(EX2)), 0);
    }

    #[test]
    fn ex3_part1() {
        assert_eq!(part1(&input_generator(EX3)), 3);
    }

    #[test]
    fn ex4_part1() {
        assert_eq!(part1(&input_generator(EX4)), 3);
    }

    #[test]
    fn ex5_part1() {
        assert_eq!(part1(&input_generator(EX5)), 3);
    }

    #[test]
    fn ex6_part1() {
        assert_eq!(part1(&input_generator(EX6)), -1);
    }

    #[test]
    fn ex7_part1() {
        assert_eq!(part1(&input_generator(EX7)), -1);
    }

    #[test]
    fn ex8_part1() {
        assert_eq!(part1(&input_generator(EX8)), -3);
    }

    #[test]
    fn ex9_part1() {
        assert_eq!(part1(&input_generator(EX9)), -3);
    }

    #[test]
    fn ex10_part2() {
        assert_eq!(part2(&input_generator(EX10)), 1);
    }

    #[test]
    fn ex11_part2() {
        assert_eq!(part2(&input_generator(EX11)), 5);
    }
}
