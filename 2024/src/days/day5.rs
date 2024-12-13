use crate::days::day::Day;
use std::collections::HashMap;

pub struct Day5;

impl Day for Day5 {
    fn day_number(&self) -> &str {
        "5"
    }

    fn part_1(&self) -> String {
        let input = self.load_input();

        // Split input into rules and updates
        let mut rules_input = Vec::new();
        let mut updates_input = Vec::new();
        let mut is_rules = true;
        for line in input.lines() {
            if line.is_empty() {
                is_rules = false;
                continue;
            }
            if is_rules {
                rules_input.push(line);
            } else {
                updates_input.push(line);
            }
        }

        // Parse Ordering Rules
        let ordering_rules = OrderingRules::parse(&rules_input);

        // Parse Updates
        let mut updates = Vec::new();
        for line in &updates_input {
            let pages: Vec<i32> = line
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .filter_map(Result::ok)
                .collect();
            updates.push(pages);
        }

        // Validate Updates and Sum Middle Pages
        let mut sum_of_middle_pages = 0;
        let mut correctly_ordered_updates = Vec::new();

        for update in &updates {
            if ordering_rules.is_correctly_ordered(update) {
                correctly_ordered_updates.push(update.clone());
                let middle = find_middle(update);
                // println!(
                //     "Correctly ordered update: {:?}, Middle page: {}",
                //     update, middle
                // );
                sum_of_middle_pages += middle;
            } else {
                //println!("Incorrectly ordered update: {:?}", update);
            }
        }
        // println!(
        //     "\nSum of middle page numbers from correctly ordered updates: {}",
        //     sum_of_middle_pages
        // );

        sum_of_middle_pages.to_string()
    }

    fn part_2(&self) -> String {
        let input = self.load_input();

        // Split input into rules and updates
        let mut rules_input = Vec::new();
        let mut updates_input = Vec::new();
        let mut is_rules = true;
        for line in input.lines() {
            if line.is_empty() {
                is_rules = false;
                continue;
            }
            if is_rules {
                rules_input.push(line);
            } else {
                updates_input.push(line);
            }
        }

        // Parse Ordering Rules
        let ordering_rules = OrderingRules::parse(&rules_input);

        // Parse Updates
        let mut updates = Vec::new();
        for line in &updates_input {
            let pages: Vec<i32> = line
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .filter_map(Result::ok)
                .collect();
            updates.push(pages);
        }

        // Validate Updates and Sum Middle Pages
        let mut sum_of_middle_pages = 0;

        for update in &updates {
            if !ordering_rules.is_correctly_ordered(update) {
                let mut updated = update.clone();
                fix_order(&ordering_rules, &mut updated);
                let middle = find_middle(&updated);
                sum_of_middle_pages += middle;
            }
        }
        sum_of_middle_pages.to_string()
    }
}




/// Struct to hold ordering rules.
struct OrderingRules {
    rules: Vec<(i32, i32)>,
}

impl OrderingRules {
    /// Parses ordering rules from a list of strings.
    fn parse(rules_str: &[&str]) -> Self {
        let mut rules = Vec::new();
        for line in rules_str {
            let parts: Vec<&str> = line.trim().split('|').collect();
            if parts.len() == 2 {
                if let (Ok(x), Ok(y)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    rules.push((x, y));
                } else {
                    eprintln!("Invalid rule format: {}", line);
                }
            } else {
                eprintln!("Invalid rule format: {}", line);
            }
        }
        OrderingRules { rules }
    }

    /// Checks if an update is correctly ordered based on the rules.
    fn is_correctly_ordered(&self, update: &[i32]) -> bool {
        // Create a map from page number to its index in the update.
        let mut page_indices: HashMap<i32, usize> = HashMap::new();
        for (index, &page) in update.iter().enumerate() {
            page_indices.insert(page, index);
        }

        // For each rule, if both pages are present in the update, check their order.
        for &(x, y) in &self.rules {
            if let (Some(&idx_x), Some(&idx_y)) = (page_indices.get(&x), page_indices.get(&y)) {
                if idx_x >= idx_y {
                    return false; // X should come before Y
                }
            }
        }

        true
    }
}

/// Finds the middle element of a slice.
fn find_middle(update: &[i32]) -> i32 {
    if update.is_empty() {
        panic!("Cannot find middle of an empty update.");
    }
    let middle_index = (update.len() - 1) / 2;
    update[middle_index]
}


/// Fix the order according to the rules
fn fix_order(ordering_rules: &OrderingRules, update: &mut Vec<i32>) {
    // Create a map from page number to its index in the update.
    let mut page_indices: HashMap<i32, usize> = HashMap::new();
    for (index, &page) in update.iter().enumerate() {
        page_indices.insert(page, index);
    }

    // For each rule, if both pages are present in the update, check their order.
    for &(x, y) in &ordering_rules.rules {
        if let (Some(&idx_x), Some(&idx_y)) = (page_indices.get(&x), page_indices.get(&y)) {
            if idx_x >= idx_y {
                // Swap the pages
                let temp = update[idx_x];
                update[idx_x] = update[idx_y];
                update[idx_y] = temp;
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parse_test() {
        use indoc::indoc;

        const INPUT: &str = indoc! {"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
        "};

            // Split input into rules and updates
            let mut rules_input = Vec::new();
            let mut updates_input = Vec::new();
            let mut is_rules = true;
            for line in INPUT.lines() {
                if line.is_empty() {
                    is_rules = false;
                    continue;
                }
                if is_rules {
                    rules_input.push(line);
                } else {
                    updates_input.push(line);
                }
            }
            assert_eq!(rules_input.len(), 21);
            assert_eq!(updates_input.len(), 6);
    }

    #[test]
    fn test_is_correctly_ordered_correct_update1() {
        let rules_input = [
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
        ];
        let ordering_rules = OrderingRules::parse(&rules_input);
        let update = vec![75, 47, 61, 53, 29];
        assert!(ordering_rules.is_correctly_ordered(&update));
    }

    #[test]
    fn test_is_correctly_ordered_correct_update2() {
        let rules_input = [
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
        ];
        let ordering_rules = OrderingRules::parse(&rules_input);
        let update = vec![97, 61, 53, 29, 13];
        assert!(ordering_rules.is_correctly_ordered(&update));
    }

    #[test]
    fn test_is_correctly_ordered_correct_update3() {
        let rules_input = [
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
        ];
        let ordering_rules = OrderingRules::parse(&rules_input);
        let update = vec![75, 29, 13];
        assert!(ordering_rules.is_correctly_ordered(&update));
    }

    #[test]
    fn test_is_correctly_ordered_incorrect_update1() {
        let rules_input = [
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
        ];
        let ordering_rules = OrderingRules::parse(&rules_input);
        let update = vec![75, 97, 47, 61, 53];
        assert!(!ordering_rules.is_correctly_ordered(&update));
    }

    #[test]
    fn test_is_correctly_ordered_incorrect_update2() {
        let rules_input = [
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
        ];
        let ordering_rules = OrderingRules::parse(&rules_input);
        let update = vec![61, 13, 29];
        assert!(!ordering_rules.is_correctly_ordered(&update));
    }

    #[test]
    fn test_find_middle() {
        let update = vec![75, 47, 61, 53, 29];
        assert_eq!(find_middle(&update), 61);

        let update_even = vec![1, 2, 3, 4];
        assert_eq!(find_middle(&update_even), 2); // Lower middle

        let update_single = vec![5];
        assert_eq!(find_middle(&update_single), 5);
    }



    #[test]
    fn test_pt2_fix_order() {
        let rules_input = [
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
        ];

        let ordering_rules = OrderingRules::parse(&rules_input);
        let update = vec![75, 97, 47, 61, 53];
        assert!(!ordering_rules.is_correctly_ordered(&update));

        let mut updated = update.clone();
        fix_order(&ordering_rules, &mut updated);
        assert!(ordering_rules.is_correctly_ordered(&updated));
    }

    #[test]
    fn test_pt2_example() {
        let rules_input = [
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
        ];

        let updates_input = [
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",// Incorrectly ordered
            "61,13,29", // Incorrectly ordered
            "97,13,75,29,47",// Incorrectly ordered
        ];

        // Parse Ordering Rules
        let ordering_rules = OrderingRules::parse(&rules_input);
        // Parse Updates
        let mut updates = Vec::new();
        for line in &updates_input {
            let pages: Vec<i32> = line
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .filter_map(Result::ok)
                .collect();
            updates.push(pages);
        }

        let mut sum_of_middle_pages = 0;
        for u in &updates {
            if !ordering_rules.is_correctly_ordered(u) {
                let mut updated = u.clone();
                fix_order(&ordering_rules, &mut updated);
                let middle = find_middle(&updated);
                // Print updates
                // println!("Original update: {:?}", u);
                // println!("Fixed update: {:?}", updated);
                sum_of_middle_pages += middle;
            }
        }

        assert_eq!(sum_of_middle_pages, 47 + 29 + 47);
    }

}
