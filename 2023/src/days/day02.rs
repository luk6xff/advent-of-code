use super::day::Day;

pub struct Day02 {}

impl Day for Day02 {
    fn day_number(&self) -> &str {
        "02"
    }

    fn part_1(&self) -> String {
        let res: u32 = self.load_input()
                                .lines()
                                .map(|line| {
                                })
                                .sum::<u32>();
        res.to_string()
    }

    fn part_2(&self) -> String {
        let sum: u32 = self.load_input()
                                .lines()
                                .map(|line| {

    }
}
