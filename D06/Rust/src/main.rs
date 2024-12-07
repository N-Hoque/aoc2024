#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cell {
    position: (usize, usize),
    r#type: CellType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CellType {
    Empty,
    Obstacle,
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Orientation {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rotation {
    Clockwise,
    CounterClockwise,
}

impl Orientation {
    pub fn offset(self) -> (isize, isize) {
        match self {
            Orientation::Left => (0, -1),
            Orientation::Right => (0, 1),
            Orientation::Up => (-1, 0),
            Orientation::Down => (1, 0),
        }
    }

    pub fn rotate(self, direction: Rotation) -> Self {
        match (self, direction) {
            (Self::Left, Rotation::Clockwise) => Self::Up,
            (Self::Left, Rotation::CounterClockwise) => Self::Down,
            (Self::Up, Rotation::Clockwise) => Self::Right,
            (Self::Up, Rotation::CounterClockwise) => Self::Left,
            (Self::Right, Rotation::Clockwise) => Self::Down,
            (Self::Right, Rotation::CounterClockwise) => Self::Up,
            (Self::Down, Rotation::Clockwise) => Self::Left,
            (Self::Down, Rotation::CounterClockwise) => Self::Right,
        }
    }
}

#[derive(Debug)]
struct Guard {
    position: (usize, usize),
    orientation: Orientation,
}

fn parse_grid(input: &str) -> Vec<Vec<Cell>> {
    let mut grid = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let cells = line
            .chars()
            .enumerate()
            .map(|(jdx, c)| Cell {
                position: (idx, jdx),
                r#type: match c {
                    '^' => CellType::Start,
                    '#' => CellType::Obstacle,
                    _ => CellType::Empty,
                },
            })
            .collect();
        grid.push(cells);
    }

    grid
}

fn solve_part_one(grid: &[Vec<Cell>]) -> usize {
    let mut start_position = (0, 0);
    for idx in 0..grid.len() {
        for jdx in 0..grid[0].len() {
            let cur_cell = grid[idx][jdx];
            if let CellType::Start = cur_cell.r#type {
                start_position = cur_cell.position;
            }
        }
    }

    let mut guard = Guard {
        position: start_position,
        orientation: Orientation::Up,
    };

    let mut visited_cells = Vec::new();

    loop {
        let position = guard.position;
        if !visited_cells.contains(&position) {
            visited_cells.push(position);
        }
        let offset = guard.orientation.offset();
        let mut new_x = ((guard.position.0 as isize) + offset.0) as usize;
        let mut new_y = ((guard.position.1 as isize) + offset.1) as usize;

        match grid.get(new_x) {
            None => break,
            Some(row) => match row.get(new_y) {
                None => break,
                Some(cell) => {
                    if cell.r#type == CellType::Obstacle {
                        guard.orientation = guard.orientation.rotate(Rotation::Clockwise);
                        let offset = guard.orientation.offset();
                        new_x = ((guard.position.0 as isize) + offset.0) as usize;
                        new_y = ((guard.position.1 as isize) + offset.1) as usize;
                    }
                    guard.position = (new_x, new_y);
                }
            },
        }
    }

    visited_cells.len()
}

fn solve_part_two(grid: &[Vec<Cell>]) -> i32 {
    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string("../input.txt")?;

    let grid = parse_grid(&data);

    let p1 = solve_part_one(&grid);

    println!("{p1}");

    let p2 = solve_part_two(&grid);

    println!("{p2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    static SAMPLE: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_solve_one() {
        let expected = 41;

        let grid = super::parse_grid(SAMPLE);

        let actual = super::solve_part_one(&grid);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_solve_two() {
        let expected = 123;

        let grid = super::parse_grid(SAMPLE);

        let actual = super::solve_part_two(&grid);

        assert_eq!(expected, actual);
    }
}
