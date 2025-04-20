use library::read_until_empty_line;

pub fn part01(input: &str) -> u64 {
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
            let decreasing = report.iter().all(|&level| level < 0 && level >= -3);

            decreasing || report.iter().all(|&level| level > 0 && level <= 3)
        })
        .count();
    valid_reports as u64
}
