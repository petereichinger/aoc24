use library::dense_grid;

pub fn part01(input: &str) -> u64 {
    let grid = dense_grid::DenseGrid::from_string(input);
    println!("{}", grid);
    0
}
