pub fn read_until_empty_line<'a>(
    lines: impl Iterator<Item = &'a str>,
) -> impl Iterator<Item = &'a str> {
    lines.take_while(|line| line.len() > 0)
}
