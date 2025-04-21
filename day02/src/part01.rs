use library::read_until_empty_line;

pub fn part01(input: &str) -> u64 {
    let reports = read_until_empty_line(input.lines());

    let reports = reports.map(|report| {
        report
            .split_whitespace()
            .map(|level| level.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    });

    fn check_decr_fault(mut report: impl Iterator<Item = i64>) -> Option<usize> {
        report.position(|level| !(level < 0 && level >= -3))
    }

    fn check_incr_fault(mut report: impl Iterator<Item = i64>) -> Option<usize> {
        report.position(|level| !(level > 0 && level <= 3))
    }

    fn check_fault(report: impl Iterator<Item = i64> + Clone) -> Option<usize> {
        let decr_fault = check_decr_fault(report.clone());

        match decr_fault {
            Some(_) => check_incr_fault(report),
            None => None,
        }
    }

    let valid_reports = reports
        .filter(|report| {
            check_fault(
                report
                    .windows(2)
                    .map(|window| window.get(0).unwrap() - window.get(1).unwrap()),
            )
            .is_none()
        })
        .count();
    valid_reports as u64
}
