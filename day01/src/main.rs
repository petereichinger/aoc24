mod common;
mod part01;
mod part02;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn main() {
    use part01::part01;
    use part02::part02;
    let (example01, input01) = (part01(EXAMPLE), part01(INPUT));
    let (example02, input02) = (part02(EXAMPLE), part02(INPUT));

    println!("{example01}, {input01}");
    println!("{example02}, {input02}");
}
