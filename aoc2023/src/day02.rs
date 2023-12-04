use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day02)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|l| Game::new(l.to_string()))
        .collect::<Vec<Game>>()
}

#[derive(Debug)]
pub struct Game {
    number: u32,
    pulls: Vec<Pull>,
}

impl Game {
    pub fn new(game: String) -> Game {
        let game_raw = game.split_once(':').unwrap();
        let number = game_raw
            .0
            .split_once(' ')
            .unwrap()
            .1
            .parse::<u32>()
            .unwrap();
        let mut pulls = Vec::<Pull>::new();
        for p in game_raw.1.split(';') {
            pulls.push(Pull::new(p));
        }
        Game { number, pulls }
    }
}

#[derive(Debug)]
pub struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}

impl Pull {
    fn new(data: &str) -> Pull {
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;
        let blocks: Vec<&str> = data.split(',').collect();
        for b in blocks {
            match b.trim().split_once(' ').unwrap() {
                (n, "red") => red_count = n.parse::<u32>().unwrap(),
                (n, "green") => green_count = n.parse::<u32>().unwrap(),
                (n, "blue") => blue_count = n.parse::<u32>().unwrap(),
                _ => unreachable!(),
            }
        }
        Pull {
            red: red_count,
            green: green_count,
            blue: blue_count,
        }
    }
}

#[derive(Debug)]
pub enum Block {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[aoc(day02, part1)]
pub fn part1(input: &Vec<Game>) -> u32 {
    //targets
    let tr = 12;
    let tg = 13;
    let tb = 14;

    let mut possible = Vec::<u32>::new();
    for i in input {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for p in &i.pulls {
            if p.red > min_red {
                min_red = p.red;
            }
            if p.green > min_green {
                min_green = p.green;
            }
            if p.blue > min_blue {
                min_blue = p.blue;
            }
        }
        if min_red <= tr && min_green <= tg && min_blue <= tb {
            possible.push(i.number);
        }
    }
    possible.into_iter().sum()
}

#[aoc(day02, part2)]
pub fn part2(input: &Vec<Game>) -> u32 {
    let mut power = Vec::<u32>::new();
    for i in input {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for p in &i.pulls {
            if p.red > min_red {
                min_red = p.red;
            }
            if p.green > min_green {
                min_green = p.green;
            }
            if p.blue > min_blue {
                min_blue = p.blue;
            }
        }
        power.push(min_red * min_green * min_blue);
    }
    power.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
	"};

    #[test]
    fn example_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 8);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 2286);
    }
}
