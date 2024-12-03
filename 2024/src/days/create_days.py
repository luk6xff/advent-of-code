#!/usr/bin/env python3

import os

# Define the number of days
TOTAL_DAYS = 25

# Define the directory where day modules will be created
DAYS_DIR = os.path.dirname(__file__)

# Ensure the days directory exists
os.makedirs(DAYS_DIR, exist_ok=True)

# Template for each day module
TEMPLATE = '''use crate::days::day::Day;

pub struct Day{day_number};

impl Day for Day{day_number} {{
    fn day_number(&self) -> &str {{
        "{day_number}"
    }}

    fn part_1(&self) -> String {{
        let input = self.load_input();

        "Part 1".to_string()
    }}

    fn part_2(&self) -> String {{
        let input = self.load_input();

        "Part 2".to_string()
    }}
}}
'''

def create_day_file(day_num):
    """
    Creates a Rust module file for a given day number with a predefined template.
    """
    filename = f'day{day_num}.rs'
    filepath = os.path.join(DAYS_DIR, filename)

    # Check if the file already exists
    if os.path.exists(filepath):
        print(f'[SKIP] {filename} already exists.')
        return

    # Generate the file content using the template
    content = TEMPLATE.format(day_number=day_num)

    # Write the content to the file
    with open(filepath, 'w') as file:
        file.write(content)

    print(f'[CREATE] {filename} created successfully.')

def main():
    """
    Main function to create all day files.
    """
    print('Starting creation of day modules...\n')
    for day in range(1, TOTAL_DAYS + 1):
        create_day_file(day)
    print('\nAll day modules have been processed.')

if __name__ == '__main__':
    main()
