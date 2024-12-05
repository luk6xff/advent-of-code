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

        "Part 2".to_string()
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
    }
}

