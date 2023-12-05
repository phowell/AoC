use itertools::Itertools;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day03)]
pub fn input_generator(input: &str) -> Schematic {
    let dia: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let max_x = dia.len() - 1;
    let max_y = dia[0].len() - 1;
    Schematic { dia, max_x, max_y }
}

pub struct Schematic {
    dia: Vec<Vec<char>>,
    max_x: usize,
    max_y: usize,
}

impl Schematic {
    fn parts(&self) -> Vec<(usize, usize)> {
        let mut parts = Vec::new();
        for (i, x) in self.dia.iter().enumerate() {
            for (j, y) in x.iter().enumerate() {
                if !y.is_numeric() && *y != '.' {
                    parts.push((i, j));
                }
            }
        }
        parts
    }

    fn gears(&self) -> Vec<(usize, usize)> {
        let parts = self.parts();
        parts
            .into_iter()
            .filter(|x| self.dia[x.0][x.1] == '*')
            .collect()
    }

    fn values(&self, pivot: (usize, usize)) -> Vec<usize> {
        let (x, y) = pivot;
        let n = self.neighbours(x, y);
        n.iter()
            .filter(|p| self.dia[p.0][p.1].is_numeric())
            .map(|i| self.first_digit(i.0, i.1))
            .unique()
            .map(|j| self.int_from_ptr(j.0, j.1))
            .collect()
    }

    fn first_digit(&self, x: usize, y: usize) -> (usize, usize) {
        let mut p = y;
        let mut f = false;
        while !f {
            if p == 0 || !self.dia[x][p - 1].is_numeric() {
                f = true;
            } else {
                p -= 1;
            }
        }
        (x, p)
    }
    fn int_from_ptr(&self, x: usize, y: usize) -> usize {
        let mut digits = Vec::<char>::new();
        let mut ptr = y;

        while ptr <= self.max_y && self.dia[x][ptr].is_numeric() {
            digits.push(self.dia[x][ptr]);
            ptr += 1;
        }
        let numstr: String = digits.into_iter().collect();
        numstr.parse::<usize>().unwrap()
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        if (x, y) == (0, 0) {
            vec![(0, 1), (1, 0), (1, 1)]
        } else if (x, y) == (0, self.max_y) {
            vec![(0, self.max_y - 1), (1, self.max_y), (1, self.max_y - 1)]
        } else if (x, y) == (self.max_x, 0) {
            vec![(self.max_x, 1), (self.max_x - 1, 0), (self.max_x - 1, 1)]
        } else if (x, y) == (self.max_x, self.max_y) {
            vec![
                (self.max_x, self.max_y - 1),
                (self.max_x - 1, self.max_y),
                (self.max_x - 1, self.max_y - 1),
            ]
        } else if x == 0 {
            vec![
                (x, y - 1),
                (x, y + 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
            ]
        } else if x == self.max_x {
            vec![
                (x - 1, y - 1),
                (x - 1, y),
                (x - 1, y + 1),
                (x, y - 1),
                (x, y + 1),
            ]
        } else if y == 0 {
            vec![
                (x - 1, y),
                (x + 1, y),
                (x - 1, y + 1),
                (x, y + 1),
                (x + 1, y + 1),
            ]
        } else if y == self.max_y {
            vec![
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
                (x - 1, y),
                (x + 1, y),
            ]
        } else {
            vec![
                (x - 1, y - 1),
                (x - 1, y),
                (x - 1, y + 1),
                (x, y - 1),
                (x, y + 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
            ]
        }
    }
}

#[aoc(day03, part1)]
pub fn part1(input: &Schematic) -> usize {
    let parts = input.parts();
    let mut sum: usize = 0;
    for p in parts {
        sum += input.values(p).iter().sum::<usize>();
    }
    sum
}

#[aoc(day03, part2)]
pub fn part2(input: &Schematic) -> usize {
    let gears = input.gears();
    let mut sum: usize = 0;
    for g in gears {
        if input.values(g).len() == 2 {
            sum += input.values(g).iter().product::<usize>()
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
	"};

    #[test]
    fn example_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 4361);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 467835);
    }
}
