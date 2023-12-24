use super::day::Day;


#[derive(Debug, Default)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {

    fn max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn is_possible(&self) -> bool {
        const MAX_CUBES: [u32; 3] = [12, 13, 14];
        self.red <= MAX_CUBES[0] && self.green <= MAX_CUBES[1] && self.blue <= MAX_CUBES[2]
    }

    fn from_string(line: &str) -> Vec<CubeSet> {
        line.split(';')
            .map(|set| {
                let mut cubes = CubeSet {red: 0, green: 0, blue: 0 };
                for color in set.split(',') {
                    let parts: Vec<&str> = color.trim().split(' ').collect();
                    let count = parts[0].parse::<u32>().unwrap();
                    match parts[1] {
                        "red" => cubes.red = count,
                        "green" => cubes.green = count,
                        "blue" => cubes.blue = count,
                        _ => unreachable!(),
                    }
                }
                cubes
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
        let res: u32 = self.load_input()
                                .lines()
                                .enumerate()
                                .filter()
        res.to_string()
    }

    fn part_2(&self) -> String {
        // let sum: u32 = self.load_input()
        //                         .lines()
        //                         .map(|line| {
            "test".to_string()
    }
}
