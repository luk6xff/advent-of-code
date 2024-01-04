use super::day::Day;


#[derive(Debug)]
enum Value {
    Empty,
    Symbol(char),
    Number(u32),
}

pub struct Day03 {}

impl Day for Day03 {
    fn day_number(&self) -> &str {
        "03"
    }

    fn part_1(&self) -> String {
        let res: Vec<(usize, usize, Value)> = self.load_input()
                            .lines()
                            .enumerate()
                            .flat_map(|(y, line)| {
                                line.chars()
                                    .enumerate()
                                    .map( move |(x, c)| {
                                        match c {
                                            '.' => (x, y, Value::Empty),
                                            character => if character.is_ascii_digit() {
                                                (x, y, Value::Number(character.to_digit(10).expect("Invalid digit"),))
                                            } else {
                                                (x, y, Value::Symbol(character))
                                            },
                                            symbol => (x, y, Value::Symbol(symbol)),
                                        }
                                    })
                            }).collect::<Vec<(usize, usize, Value)>>();
        for (x, y, v) in res {
            println!("{} {} {:?}", x, y, v);
        }
        " ".to_string()
    }

    fn part_2(&self) -> String {
        // let res: u32 = self.load_input()
        //                         .lines();
        " ".to_string()
    }
}
