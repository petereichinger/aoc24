use regex::Regex;

pub fn part01(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum: u64 = re
        .captures_iter(input)
        .map(|cap| {
            let first = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let second = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();

            first * second
        })
        .sum();
    sum
}
