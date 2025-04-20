pub fn part01(input: &str) -> u64 {
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

    a.sort();
    b.sort();

    let sum: u64 = a
        .iter()
        .zip(b)
        .map(|(&a, b)| if a > b { a - b } else { b - a })
        .sum();

    sum
}
