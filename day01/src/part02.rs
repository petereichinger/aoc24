use std::collections::HashMap;

pub fn part02(input: &str) -> u64 {
    let (mut a, mut b): (Vec<u64>, Vec<u64>) = input
        .lines()
        .map(|line| {
            let mut splits = line.split_whitespace();
            let [first, second] = [splits.next().unwrap(), splits.next().unwrap()];

            (
                first.parse::<u64>().unwrap(),
                second.parse::<u64>().unwrap(),
            )
        })
        .unzip();

    let mut occurrences: HashMap<u64, u64> = HashMap::new();

    b.iter().for_each(|&item| {
        let entry = occurrences.entry(item).or_insert(0);
        *entry = *entry + 1;
    });

    let occurrences = occurrences;

    let sum = a
        .iter()
        .map(|entry| {
            let occ = occurrences.get(entry).unwrap_or(&0);
            occ * entry
        })
        .sum();

    sum
}

