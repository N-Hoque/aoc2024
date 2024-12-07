use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    position: (usize, usize),
    r#type: CellType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CellType {
    Empty,
    Obstacle,
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    Clockwise,
    CounterClockwise,
}

impl Direction {
    pub fn offset(self) -> (isize, isize) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
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
pub struct Guard {
    position: (usize, usize),
    direction: Direction,
}

impl Guard {
    fn new(start: (usize, usize), direction: Direction) -> Self {
        Self {
            position: start,
            direction,
        }
    }

    pub fn step(&mut self, grid: &[Vec<Cell>]) -> Option<(usize, usize)> {
        let previous_direction = self.direction;

        let height = grid.len() as isize;
        let width = grid[0].len() as isize;

        let (dx, dy) = previous_direction.offset();
        let next_x = self.position.0 as isize + dx;
        let next_y = self.position.1 as isize + dy;

        // Check bounds BEFORE casting to usize
        if next_x < 0 || next_x >= height || next_y < 0 || next_y >= width {
            return None;
        }

        let (x, y) = (next_x as usize, next_y as usize);
        let next_cell = &grid[x][y];

        if next_cell.r#type != CellType::Obstacle {
            // Can move forward
            self.position = (x, y);
            Some(self.position)
        } else {
            // Hit obstacle, turn right
            self.direction = self.direction.rotate(Rotation::Clockwise);
            Some(self.position)
        }
    }
}

pub fn parse_grid(input: &str) -> Vec<Vec<Cell>> {
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

pub fn get_start_position(grid: &[Vec<Cell>]) -> (usize, usize) {
    let mut start_position = (0, 0);
    for idx in 0..grid.len() {
        for jdx in 0..grid[0].len() {
            let cur_cell = grid[idx][jdx];
            if let CellType::Start = cur_cell.r#type {
                start_position = cur_cell.position;
            }
        }
    }
    start_position
}

pub fn causes_loop(
    grid: &[Vec<Cell>],
    start_pos: (usize, usize),
    obstacle_pos: (usize, usize),
) -> bool {
    let mut guard = Guard::new(start_pos, Direction::Up);
    let mut visited_states = HashSet::new();

    loop {
        let current_state = (guard.position, guard.direction);

        if visited_states.contains(&current_state) {
            return true;
        }

        visited_states.insert(current_state);

        // Check if we would step into our temporary obstacle
        let (dx, dy) = guard.direction.offset();
        let next_x = guard.position.0 as isize + dx;
        let next_y = guard.position.1 as isize + dy;

        if next_x < 0
            || next_x >= grid.len() as isize
            || next_y < 0
            || next_y >= grid[0].len() as isize
        {
            return false; // Guard leaves grid
        }

        let next_pos = (next_x as usize, next_y as usize);

        // Check if we hit our temporary obstacle or an existing one
        if next_pos == obstacle_pos || grid[next_pos.0][next_pos.1].r#type == CellType::Obstacle {
            guard.direction = guard.direction.rotate(Rotation::Clockwise);
        } else {
            guard.position = next_pos;
        }
    }
}

pub fn get_guard_path(grid: &[Vec<Cell>], start_pos: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut guard = Guard::new(start_pos, Direction::Up);
    let mut visited_positions = HashSet::new();
    visited_positions.insert(start_pos);

    while let Some(position) = guard.step(grid) {
        visited_positions.insert(position);
    }

    visited_positions
}
