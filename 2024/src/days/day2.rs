use crate::days::day::Day;

pub struct Day2;


impl Day for Day2 {
    fn day_number(&self) -> &str {
        "2"
    }

    fn part_1(&self) -> String {
        let input = self.load_input();
        let mut solution: i32 = 0;
        for line in input.lines() {
            match self.validate_reports(line) {
                State::Safe => {
                    solution += 1;
                }
                State::Unsafe => (),
            }
        }
        solution.to_string()

    }

    fn part_2(&self) -> String {
        let input = self.load_input();
        let mut reports = Vec::new();
        for line in input.lines() {
            let int_line: Result<Vec<i32>, _> = line
                .split_whitespace()
                .map(|item| item.parse::<i32>())
                .collect();

            match int_line {
                Ok(numbers) => reports.push(numbers),
                Err(e) => {
                    println!(
                        "Error parsing line. Skipping this line. Error: {}", e
                    )
                }
            }
        }

        let solution: usize = count_reports(&reports, true);
        // println!(
        //     "Part 2: The number of safe reports is: {}",
        //     solution
        // );
        solution.to_string()
    }
}


#[derive(Debug, PartialEq)]
enum State {
    Safe,
    Unsafe,
}

#[derive(Debug, PartialEq)]
enum Order {
    Decreasing,
    Increasing,
    Equal,
}

impl Day2 {
    fn validate_reports(&self, report: &str) -> State {
        let mut state = State::Safe;
        let mut order = Order::Equal;
        let mut iter = report.split_whitespace();
        let mut last= match iter.next() {
            Some(value) => match value.parse::<i32>() {
                Ok(num) => num,
                Err(_) => return State::Unsafe,
            },
            None => return State::Unsafe,
        };

        for item in iter {
            let current = match item.parse::<i32>() {
                Ok(num) => num,
                Err(_) => return State::Unsafe,
            };

            if current > last {
                if order == Order::Decreasing {
                    state = State::Unsafe;
                    break;
                }
                order = Order::Increasing;
            } else if current < last {
                if order == Order::Increasing {
                    state = State::Unsafe;
                    break;
                }
                order = Order::Decreasing;
            } else {
                state = State::Unsafe;
                break;
            };

            let diff = current.abs_diff(last);
            if diff < 1 || diff > 3 {
                state = State::Unsafe;
                break;
            }

            last = current;
        }
        state
    }

}



/// Checks if a given report is safe.
/// A report is safe if all step differences are either:
/// - All increasing with step sizes between 1 and 3.
/// - All decreasing with step sizes between -3 and -1.
fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        // A single element or empty report is considered safe.
        return true;
    }

    // Calculate step differences.
    let steps: Vec<i32> = report.windows(2).map(|w| w[1] - w[0]).collect();

    // Check if all steps are increasing within 1 to 3.
    let increasing = steps.iter().all(|&step| step > 0 && step <= 3);

    // Check if all steps are decreasing within -3 to -1.
    let decreasing = steps.iter().all(|&step| step < 0 && step >= -3);

    increasing || decreasing
}

/// Checks if removing any single element from the report results in a safe report.
fn safe_removal(report: &[i32]) -> bool {
    for i in 0..report.len() {
        let mut modified_report = Vec::with_capacity(report.len() - 1);
        for (idx, &value) in report.iter().enumerate() {
            if idx != i {
                modified_report.push(value);
            }
        }
        if is_safe(&modified_report) {
            return true;
        }
    }
    false
}

/// Counts the number of safe reports.
/// If `removal` is true, also counts reports that become safe after a single removal.
fn count_reports(reports: &[Vec<i32>], removal: bool) -> usize {
    let mut safe_counter = 0;
    for report in reports {
        if is_safe(report) {
            // Counter for part 1.
            safe_counter += 1;
        } else if removal && safe_removal(report) {
            // Counter for part 2.
            safe_counter += 1;
        }
    }
    safe_counter
}


