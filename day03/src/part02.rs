use regex::Regex;

pub fn part02(input: &str) -> u64 {
    let mut filtered_input = String::new();

    let mut enabled = true;
    let mut current_input = input;
    let mut current_search = "don't()";

    while let Some(next_flip_index) = current_input.find(current_search) {
        println!("{} {}", enabled, &current_input[0..next_flip_index]);
        if enabled {
            filtered_input.push_str(&current_input[0..next_flip_index]);
        }

        current_input = &current_input[(next_flip_index + current_search.len())..];
        enabled = !enabled;
        if enabled {
            current_search = "don't()";
        } else {
            current_search = "do()";
        }
    }

    if enabled {
        println!("{}", current_input);
        filtered_input.push_str(current_input);
    }

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // println!("{}", filtered_input);
    let sum: u64 = re
        .captures_iter(&filtered_input)
        .map(|cap| {
            let first = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let second = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();

            first * second
        })
        .sum();
    sum
}
