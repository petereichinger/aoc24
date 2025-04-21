use std::env;

mod common;
mod part01;
mod part02;

const EXAMPLE: &str = include_str!("example.txt");
const EXAMPLE02: &str = include_str!("example02.txt");
const INPUT: &str = include_str!("input.txt");

fn wait_on_interactive(interactive: bool) {
    if interactive {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
    }
}
fn main() {
    let interactive = env::args().skip(1).nth(0).unwrap_or("".to_owned()) == "-i";

    use part01::part01;
    use part02::part02;

    let example01 = part01(EXAMPLE);
    println!("ex 01 {}", example01);
    wait_on_interactive(interactive);

    let input01 = part01(INPUT);
    println!("in 01 {}", input01);
    wait_on_interactive(interactive);

    let example02 = part02(EXAMPLE02);
    println!("ex 02 {}", example02);
    wait_on_interactive(interactive);

    let input02 = part02(INPUT);
    println!("in02 {}", input02);
}
