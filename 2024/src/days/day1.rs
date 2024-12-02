use std::collections::HashMap;

use crate::days::day::Day;

pub struct Day1;

impl Day for Day1 {
    fn day_number(&self) -> &str {
        "1"
    }

    fn part_1(&self) -> String {
        let input = self.load_input();
        let mut data_x: Vec<i32> = Vec::new();
        let mut data_y: Vec<i32> = Vec::new();

        for line in input.lines() {
            let mut iter = line.split_whitespace();
            data_x.push(iter.next().unwrap().parse::<i32>().unwrap());
            data_y.push(iter.next().unwrap().parse::<i32>().unwrap());
        }

        // Sort the data by the first column
        data_x.sort_by(|a, b| a.partial_cmp(b).unwrap());
        data_y.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Compute the solution
        let mut solution: i32 = 0;
        for (index, x) in data_x.iter().enumerate() {
            solution += (x - data_y[index]).abs();
        }

        solution.to_string()
    }

    fn part_2(&self) -> String {
        let input = self.load_input();
        let mut data_x: Vec<i32> = Vec::new();
        let mut data_y_map: HashMap<i32, i32> = HashMap::new();

        for line in input.lines() {
            let mut iter = line.split_whitespace();
            data_x.push(iter.next().unwrap().parse::<i32>().unwrap());
            let y = iter.next().unwrap().parse::<i32>().unwrap();
            data_y_map.insert(y, data_y_map.get(&y).unwrap_or(&0) + 1);
        }

        // Compute the solution
        let mut solution: i32 = 0;
        for x in data_x {
            solution += x * data_y_map.get(&x).unwrap_or(&0);
        }

        solution.to_string()
    }
}
