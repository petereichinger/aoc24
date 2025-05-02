use library::{coordinate::Coord, dense_grid};

const DIRECTIONS: [Coord; 8] = [
    Coord { x: 1, y: 0 },
    Coord { x: -1, y: 0 },
    Coord { x: 0, y: 1 },
    Coord { x: 0, y: -1 },
    Coord { x: 1, y: 1 },
    Coord { x: 1, y: -1 },
    Coord { x: -1, y: 1 },
    Coord { x: -1, y: -1 },
];
pub fn part01(input: &str) -> u64 {
    let grid = dense_grid::DenseGrid::from_string(input);

    let mut count = 0;
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let coord = Coord::new(x as isize, y as isize);

            for dir in DIRECTIONS {
                let x_coord = coord;
                let m_coord = coord + dir;
                let a_coord = m_coord + dir;
                let s_coord = a_coord + dir;
                if let (Some('X'), Some('M'), Some('A'), Some('S')) = (
                    grid.get(&x_coord),
                    grid.get(&m_coord),
                    grid.get(&a_coord),
                    grid.get(&s_coord),
                ) {
                    count += 1;
                }
            }
        }
    }
    count
}
