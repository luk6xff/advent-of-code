use crate::days::day::Day;

pub struct Day1;

impl Day for Day1 {
    fn day_number(&self) -> &str {
        "1"
    }

    fn part_1(&self) -> String {
        let input = self.load_input();
        let solution = input
            .lines()
            .map(|line| line.split_whitespace().map(|x, y| x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()).collect::<Vec<i32>>())
            .to_string();
        "Part 1 Solution".to_string()
    }

    fn part_2(&self) -> String {
        let input = self.load_input();
        // TODO: Implement your solution for Part 2
        "Part 2 Solution".to_string()
    }
}
