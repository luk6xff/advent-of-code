use super::day::Day;


#[derive(Debug, Default)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {

    fn is_possible(&self) -> bool {
        const MAX_CUBES: [u32; 3] = [12, 13, 14];
        self.red <= MAX_CUBES[0] && self.green <= MAX_CUBES[1] && self.blue <= MAX_CUBES[2]
    }

    fn max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power_of_cube(&self) -> u32 {
        self.red * self.green * self.blue
    }

    fn from_string(line: &str) -> Vec<CubeSet> {
        let (_, line) = line.split_at(line.find(':').unwrap() + 1);
        line.split(';')
            .map(|set| {
                let mut cube = CubeSet {red: 0, green: 0, blue: 0 };
                for color in set.split(',') {
                    let parts: Vec<&str> = color.trim().split(' ').collect();
                    let count = parts[0].parse::<u32>().unwrap();
                    match parts[1] {
                        "red" => cube.red = count,
                        "green" => cube.green = count,
                        "blue" => cube.blue = count,
                        _ => unreachable!(),
                    }
                }
                cube
            })
            .collect()

    }
}

pub struct Day02 {}

impl Day for Day02 {
    fn day_number(&self) -> &str {
        "02"
    }

    fn part_1(&self) -> String {
        let res: usize = self.load_input()
                                .lines()
                                .enumerate()
                                .map(|(idx, line)| { let is_not_possible = CubeSet::from_string(line).into_iter().any(|cubesat| !cubesat.is_possible()); if is_not_possible == true { 0 } else { idx+1 }})
                                .sum();
        res.to_string()
    }

    fn part_2(&self) -> String {
        let res: u32 = self.load_input()
                                .lines()
                                .map(|line| CubeSet::from_string(line))
                                .map(|game| {
                                    game.into_iter().fold(CubeSet::default(), |max_cubes, cubeset| max_cubes.max(&cubeset)).power_of_cube()
                                })
                                .sum();
        res.to_string()
    }
}
