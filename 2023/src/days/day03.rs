use super::day::Day;


pub struct Day03 {}

impl Day for Day03 {
    fn day_number(&self) -> &str {
        "03"
    }

    fn part_1(&self) -> String {
        let res: usize = self.load_input()
                                .lines();

        res.to_string()
    }

    fn part_2(&self) -> String {
        let res: u32 = self.load_input()
                                .lines();
        res.to_string()
    }
}
