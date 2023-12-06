use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day05)]
pub fn input_generator(input: &str) -> Alminac {
    let temp = input
        .split("\n\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let seeds = temp[0].split_whitespace().collect::<Vec<&str>>()[1..]
        .iter()
        .map(|&x| x.parse::<u64>().unwrap())
        .collect();

    let mut alminac = Alminac::new(seeds);

    for mapset in &temp[1..] {
        let data = mapset.trim().split('\n').collect::<Vec<_>>();
        let mapname = data[0].trim().to_string();
        let mut mappers = Vec::<PlantingMapper>::new();
        for mapper in &data[1..] {
            let m = mapper.split_whitespace().collect::<Vec<_>>();
            mappers.push(PlantingMapper::new(
                m[1].parse().unwrap(),
                m[0].parse().unwrap(),
                m[2].parse().unwrap(),
            ));
        }
        alminac.add_mappers(mapname, mappers);
    }
    alminac
}

#[derive(Debug)]
pub struct Alminac {
    seeds: Vec<u64>,
    mappers: Vec<(String, Vec<PlantingMapper>)>,
}

#[derive(Debug)]
pub struct PlantingMapper {
    source: u64,
    dest: u64,
    range: u64,
}

impl Alminac {
    fn new(seeds: Vec<u64>) -> Alminac {
        Alminac {
            seeds,
            mappers: Vec::new(),
        }
    }
    fn add_mappers(&mut self, name: String, mappers: Vec<PlantingMapper>) {
        self.mappers.push((name, mappers));
    }
    fn run(&self, range: bool) -> u64 {
        let mut input: Vec<Box<dyn Iterator<Item = u64>>>;
        let mut seed_count: u64 = 0;
        if range {
            input = Vec::new();
            for pair in self.seeds.chunks(2) {
                println!("{:?}", pair);
                seed_count += pair[1] as u64;
                input.push(Box::new(pair[0]..(pair[0] + pair[1])));
            }
        } else {
            input = Vec::new();
            seed_count = self.seeds.len() as u64;
            input.push(Box::new(self.seeds.clone().into_iter()));
        }
        let mut progress: u64 = 1;
        let mut closest = u64::MAX;
        for seed_range in input {
            println!(">>Border");
            for s in seed_range {
                print!(
                    "\r{}/100",
                    ((progress as f64 / seed_count as f64) as f32 * 100.0) as u64
                );
                let mut mapping = s;
                for mapper in &self.mappers {
                    let mut mapped = false;
                    while !mapped {
                        for m in &mapper.1 {
                            match m.map(mapping) {
                                Some(mval) => {
                                    mapped = true;
                                    mapping = mval;
                                    break;
                                }
                                None => continue,
                            }
                        }
                        if !mapped {
                            mapped = true;
                        }
                    }
                }
                if mapping < closest {
                    println!("***");
                    closest = mapping;
                }
                progress += 1;
            }
        }
        println!();
        closest
    }
}
impl PlantingMapper {
    fn new(source: u64, dest: u64, range: u64) -> PlantingMapper {
        PlantingMapper {
            source,
            dest,
            range,
        }
    }
    fn map(&self, val: u64) -> Option<u64> {
        if self.source <= val && val < (self.source + self.range) {
            Some(self.dest + (val - self.source))
        } else {
            None
        }
    }
}

#[aoc(day05, part1)]
pub fn part1(input: &Alminac) -> u64 {
    //*input.run().iter().min().unwrap()
    input.run(false)
}

#[aoc(day05, part2)]
pub fn part2(input: &Alminac) -> u64 {
    input.run(true)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &str = indoc! {"
        seeds: 79 14 55 13

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
        56 93 4
	"};

    #[test]
    fn example_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 35);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 46);
    }
}
