use super::day::Day;

pub struct Day01 {}

impl Day for Day01 {
    fn day_number(&self) -> &str {
        "01"
    }

    fn part_1(&self) -> String {
        let res: u32 = self.load_input()
                                .lines()
                                .map(|line| {
                                    let it = line.chars();
                                    let digits: Vec<char> = it.filter(|c| c.is_digit(10)).collect();
                                    let first = digits.first().cloned().unwrap_or_default();
                                    let second = digits.last().cloned().unwrap_or(first);
                                    let concat_str: String = vec![first, second].iter().collect();
                                    concat_str.parse::<u32>().unwrap_or_default()
                                })
                                .sum::<u32>();
        res.to_string()
    }

    fn part_2(&self) -> String {
        let sum: u32 = self.load_input()
                                .lines()
                                .map(|line| {
                                    let mut it = (0..line.len()).filter_map(|idx| {
                                        match &line[idx..] {
                                            line if line.starts_with("one") => Some(1),
                                            line if line.starts_with("two") => Some(2),
                                            line if line.starts_with("three") => Some(3),
                                            line if line.starts_with("four") => Some(4),
                                            line if line.starts_with("five") => Some(5),
                                            line if line.starts_with("six") => Some(6),
                                            line if line.starts_with("seven") => Some(7),
                                            line if line.starts_with("eight") => Some(8),
                                            line if line.starts_with("nine") => Some(9),
                                            line => {
                                                line.chars().next().unwrap().to_digit(10)
                                            }
                                        }
                                    });
                                    let first = it.next().expect("Should be a number");
                                    match it.last() {
                                        Some(num) => first * 10 + num,
                                        None => first * 10 + first,
                                    }
                                })
                                .sum::<u32>();
        sum.to_string()
    }
}
