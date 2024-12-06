use crate::days::day::Day;
use std::fmt;

pub struct Day4;

impl Day for Day4 {
    fn day_number(&self) -> &str {
        "4"
    }

    fn part_1(&self) -> String {
        let input = self.load_input();
        let input_formatted = input.lines().map(|s| s).collect::<Vec<&str>>();
        let found_words = self.find_word(&input_formatted, Day4::WORD);
        found_words.len().to_string()
    }

    fn part_2(&self) -> String {
        let input = self.load_input();
        let input_formatted = input.lines().map(|s| s).collect::<Vec<&str>>();
        let found_xmas = self.find_x_mas(&input_formatted);
        found_xmas.to_string()
    }
}


struct FoundWord {
    word: String,
    start_pos: (usize, usize),
    direction: String,
}

impl fmt::Display for FoundWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at ({}, {}) in direction {}", self.word, self.start_pos.0, self.start_pos.1, self.direction)
    }
}

impl Day4 {

    const WORD: &'static str = "XMAS";

    fn find_word(&self, grid: &[&str], word: &str) -> Vec<FoundWord> {
        let mut found_words = Vec::new();
        for (y, row) in grid.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if ch == word.chars().next().unwrap() {
                    for direction in ["N", "NE", "E", "SE", "S", "SW", "W", "NW"].iter() {
                        if Self::check_direction(grid, word, (x, y), direction) {
                            found_words.push(FoundWord {
                                word: word.to_string(),
                                start_pos: (x, y),
                                direction: direction.to_string(),
                            });
                        }
                    }
                }
            }
        }
        found_words
    }

    fn find_x_mas(&self, grid: &[&str]) -> usize {
        let mut found_x_mases = 0;
        for (y, row) in grid.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if ch == 'A' {
                    const COMBINATIONS: [[(char, (i32, i32)); 4]; 4] = [
                                [('M',(-1, -1)), ('S', (1, -1)), ('S', (1, 1)), ('M',(-1, 1))],
                                [('S',(-1, -1)), ('S', (1, -1)), ('M', (1, 1)), ('M',(-1, 1))],
                                [('S',(-1, -1)), ('M', (1, -1)), ('M', (1, 1)), ('S',(-1, 1))],
                                [('M',(-1, -1)), ('M', (1, -1)), ('S', (1, 1)), ('S',(-1, 1))],
                            ];
                    for combination in COMBINATIONS.iter() {
                        let mut found = true;
                        for data in combination.iter() {
                            let (dx, dy) = data.1;
                            let x = (x as i32 + dx) as usize;
                            let y = (y as i32 + dy) as usize;
                            if x >= grid[0].len() || y >= grid.len() {
                                found = false;
                                break;
                            }

                            if grid[y].chars().nth(x).unwrap() != data.0 {
                                found = false;
                                break;
                            }
                        }

                        if found {
                            found_x_mases += 1;
                        }
                    }
                }
            }
        }
        found_x_mases
    }

    fn check_direction(grid: &[&str], word: &str, start_pos: (usize, usize), direction: &str) -> bool {
        let (dx, dy) = match direction {
            "N" => (0, -1),
            "NE" => (1, -1),
            "E" => (1, 0),
            "SE" => (1, 1),
            "S" => (0, 1),
            "SW" => (-1, 1),
            "W" => (-1, 0),
            "NW" => (-1, -1),
            _ => panic!("Invalid direction"),
        };
        let mut x = start_pos.0;
        let mut y = start_pos.1;
        for ch in word.chars() {
            if x >= grid[0].len() || y >= grid.len() {
                return false;
            }
            if grid[y].chars().nth(x).unwrap() != ch {
                return false;
            }
            x = (x as i32 + dx) as usize;
            y = (y as i32 + dy) as usize;
        }
        true
    }

    fn print_grid(&self, grid: &[&str]) {
        println!("Grid:");
        for row in grid {
            println!("{}", row);
        }
        println!("");
    }

    fn print_found_words(&self, words: &[FoundWord]) {
        if words.is_empty() {
            println!("No occurrences of '{}' found", Self::WORD);
        } else {
            println!("Occurrences of '{}':", Self::WORD);
            for word in words {
                println!("{}", word);
            }
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day4;
        const GRID_SIZE: usize = 10;
        const GRID: [&str; GRID_SIZE] = [
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];

        day.print_grid(&GRID);
        let found_words = day.find_word(&GRID, Day4::WORD);
        assert_eq!(found_words.len(), 18);
    }

    #[test]
    fn test_part_2() {
        let day = Day4;
        const GRID_SIZE: usize = 10;
        const GRID: [&str; GRID_SIZE] = [
            ".M.S......",
            "..A..MSMS.",
            ".M.S.MAA..",
            "..A.ASMSM.",
            ".M.S.M....",
            "..........",
            "S.S.S.S.S.",
            ".A.A.A.A..",
            "M.M.M.M.M.",
            "..........",
        ];
        let found_x_mases = day.find_x_mas(&GRID);
        assert_eq!(found_x_mases, 9);
    }

    #[test]
    fn test_part_2_1() {
        let day = Day4;
        const GRID_SIZE: usize = 3;
        const GRID: [&str; GRID_SIZE] = [
            "M.S",
            ".A.",
            "M.S",
        ];
        let found_x_mases = day.find_x_mas(&GRID);
        assert_eq!(found_x_mases, 1);
    }

    #[test]
    fn test_part_2_2() {
        let day = Day4;
        const GRID_SIZE: usize = 3;
        const GRID: [&str; GRID_SIZE] = [
            "M.S.M.",
            ".A.A..",
            "M.S.M.",
        ];
        let found_x_mases = day.find_x_mas(&GRID);
        assert_eq!(found_x_mases, 2);
    }


}

