use library::{coordinate::Coord, dense_grid};

const DIRECTIONS: [(Coord, Coord); 4] = [
    (Coord { x: -1, y: -1 }, Coord { x: -1, y: 1 }),
    (Coord { x: -1, y: -1 }, Coord { x: 1, y: -1 }),
    (Coord { x: 1, y: -1 }, Coord { x: 1, y: 1 }),
    (Coord { x: 1, y: 1 }, Coord { x: -1, y: 1 }),
];
pub fn part02(input: &str) -> u64 {
    let grid = dense_grid::DenseGrid::from_string(input);

    let mut count = 0;
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let coord = Coord::new(x as isize, y as isize);

            for (dir_a, dir_b) in DIRECTIONS {
                let m_a = coord + dir_a;
                let m_b = coord + dir_b;
                let s_a = coord + -1 * dir_a;
                let s_b = coord + -1 * dir_b;
                if let (Some('A'), Some('M'), Some('M'), Some('S'), Some('S')) = (
                    grid.get(&coord),
                    grid.get(&m_a),
                    grid.get(&m_b),
                    grid.get(&s_a),
                    grid.get(&s_b),
                ) {
                    count += 1;
                }
            }
        }
    }
    count
}
