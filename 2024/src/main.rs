mod days;
mod utils;

struct DaySolutions {
    days: Vec<Box<dyn days::day::Day>>,
}

impl DaySolutions {
    fn add(&mut self, day_solution: Box<dyn days::day::Day>) {
        self.days.push(day_solution)
    }

    fn run(&self) {
        for day in self.days.iter() {
            let _timer = utils::timer::Timer::new();
            println!("");
            println!("Day:{} -> part_1 = {}", day.day_number(), day.part_1());
            println!("Day:{} -> part_2 = {}", day.day_number(), day.part_2());
        }
    }
}

fn main() {
    run();
}

fn run() {
    let mut solutions = DaySolutions{days:Vec::new()};
    solutions.add(Box::new(days::day1::Day1{}));
    solutions.add(Box::new(days::day2::Day2{}));
    solutions.add(Box::new(days::day3::Day3{}));
    // Run all the solutions
    solutions.run();
}



// # New way of working
// mod days;
// mod utils;

// use days::get_all_days;
// use utils::timer::Timer;

// struct DaySolutions {
//     days: Vec<Box<dyn days::day::Day>>,
// }

// impl DaySolutions {
//     fn new() -> Self {
//         Self {
//             days: get_all_days(),
//         }
//     }

//     fn run(&self) {
//         for day in &self.days {
//             let _timer = Timer::new();
//             println!("\nDay {} -> Part 1: {}", day.day_number(), day.part_1());
//             println!("Day {} -> Part 2: {}", day.day_number(), day.part_2());
//         }
//     }
// }

// fn main() {
//     let solutions = DaySolutions::new();
//     solutions.run();
// }
