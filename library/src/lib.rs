pub mod coordinate;
pub mod dense_grid;
pub fn read_until_empty_line<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
) -> impl Iterator<Item = &'a str> {
    lines.take_while(|line| line.len() > 0)
}

pub fn wait_on_interactive(interactive: bool) {
    if interactive {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
    }
}
