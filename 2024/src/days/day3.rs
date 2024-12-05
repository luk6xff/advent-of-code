use crate::days::day::Day;
use regex::Regex;

pub struct Day3;

impl Day for Day3 {
    fn day_number(&self) -> &str {
        "3"
    }

    fn part_1(&self) -> String {
        let input = self.load_input();
        self.process_mul(&input).to_string()
    }

    fn part_2(&self) -> String {
        let input = self.load_input();
        self.process_future_mul(&input).to_string()
    }
}


impl Day3 {
    fn process_mul(&self, input: &str) -> i32 {
        // Define the regex pattern to match 'mul(x,y)' with optional negative numbers
        let pattern = r"\.*mul\((-?\d+),(-?\d+)\)";

        // Compile the regex
        let re = Regex::new(pattern).unwrap();

        // Find all matches in the input string
        let mut matches = Vec::new();
        for cap in re.captures_iter(input) {
            // Extract the full match and captured groups
            let full_match = cap.get(0).unwrap().as_str();
            let x: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
            let y: i32 = cap.get(2).unwrap().as_str().parse().unwrap();

            matches.push((full_match.to_string(), x, y));
            //println!("Match: {}, x: {}, y: {}", full_match, x, y);
        }
        let mut result = 0;
        for (_full_match, x, y) in matches {
            result += x * y;
        }
        result
    }

    fn process_future_mul(&self, input: &str) -> i32 {
        // Define the regex pattern to match 'mul(x,y)' with optional negative numbers
        let pattern = r"mul\((-?\d+),(-?\d+)\)|do\(\)|don't\(\)";

        // Compile the regex
        let re = Regex::new(pattern).unwrap();

        // Find all matches in the input string
        let mut allowed = true;
        let mut result = 0;
        for cap in re.captures_iter(input) {
            // Extract the full match and captured groups
            let full_match = cap.get(0).unwrap().as_str();
            if full_match == "do()" {
                allowed = true;
                continue;
            }
            else if full_match == "don't()" {
                allowed = false;
                continue;
            }
            else {
                if allowed {
                    let x: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
                    let y: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
                    println!("Match: {}, x: {}, y: {}", full_match, x, y);
                    result += x * y;
                }
            }
        }
        result
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day3;
        assert_eq!(day.process_mul("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"), 161);
    }

    #[test]
    fn test_part_2() {
        let day = Day3;
        assert_eq!(day.process_future_mul("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"), 48);
    }
}

