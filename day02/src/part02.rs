use library::read_until_empty_line;

pub fn part02(input: &str) -> u64 {
    let reports = read_until_empty_line(input.lines());

    let reports = reports
        .map(|report| {
            report
                .split_whitespace()
                .map(|level| level.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|report| {
            report
                .windows(2)
                .map(|window| window.get(0).unwrap() - window.get(1).unwrap())
                .collect::<Vec<_>>()
        });

    let valid_reports = reports
        .filter(|report| {
            let invalid_decreases = report
                .iter()
                .filter(|&&level| level >= 0 && level < -3)
                .count();

            let invalid_increases = report
                .iter()
                .filter(|&&level| level <= 0 && level > 3)
                .count();

            let valid = invalid_decreases <= 1 || invalid_increases <= 1;
            println!("{} {:?}", valid, report);
            valid
        })
        .count();
    valid_reports as u64
}
