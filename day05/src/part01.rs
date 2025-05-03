use std::collections::{HashMap, HashSet};

use library::read_until_empty_line;

pub fn part01(input: &str) -> u64 {
    let mut precedences: HashMap<u64, HashSet<u64>> = HashMap::new();
    let mut lines = input.lines();
    read_until_empty_line(&mut lines)
        .filter_map(|line| line.split_once("|"))
        .for_each(|(a, b)| {
            let before = a.parse::<u64>().unwrap();
            let after = b.parse::<u64>().unwrap();

            let entry = precedences.entry(before).or_insert(HashSet::new());
            entry.insert(after);
        });

    let rules: Vec<_> = read_until_empty_line(&mut lines)
        .map(|line| {
            line.split(',')
                .map(|entry| entry.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let valid_rules_sum: u64 = rules
        .iter()
        .filter(|rule| {
            for (index, page) in rule.iter().enumerate() {
                let following_pages = &rule[index + 1..];

                for fp in following_pages {
                    let prec_entry = precedences.get(fp);
                    if let Some(rule) = prec_entry.map(|pe| pe.contains(&page)) {
                        if rule {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .filter_map(|rule| rule.get(rule.len() / 2))
        .sum();

    // println!("{:?}nnnn{:?}", precedences, rules);
    valid_rules_sum
}
