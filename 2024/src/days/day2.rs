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
        let mut solution: i32 = 0;
        for line in input.lines() {
            match self.validate_reports_part2(line) {
                State::Safe => {
                    solution += 1;
                }
                State::Unsafe => (),
            }
        }
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
            }

            if (current - last).abs() < 1 || (current - last).abs() > 3 {
                state = State::Unsafe;
                break;
            }

            last = current;
        }
        state
    }


    fn validate_reports_part2(&self, report: &str) -> State {
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

        let mut skipped: bool = false;

        for item in iter {
            let current = match item.parse::<i32>() {
                Ok(num) => num,
                Err(_) => return State::Unsafe,
            };

            if current > last {
                if order == Order::Decreasing {
                    if skipped {
                        state = State::Unsafe;
                        break;
                    } else {
                        skipped = true;
                    }
                }
                order = Order::Increasing;
            } else if current < last {
                if order == Order::Increasing {
                    if skipped {
                        state = State::Unsafe;
                        break;
                    } else {
                        skipped = true;
                    }
                }
                order = Order::Decreasing;
            } else {
                if skipped {
                    state = State::Unsafe;
                    break;
                } else {
                    skipped = true;
                }
                order = Order::Equal;
            }

            if (current - last).abs() < 1 || (current - last).abs() > 3 {
                if skipped {
                    state = State::Unsafe;
                    break;
                } else {
                    skipped = true;
                }
            }

            last = current;
        }
        state
    }
}
