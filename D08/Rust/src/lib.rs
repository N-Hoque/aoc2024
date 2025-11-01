use std::collections::{HashMap, HashSet};

#[repr(transparent)]
pub struct Antenna(char);
pub struct Grid(Vec<Vec<Option<Antenna>>>);

impl Grid {
    pub fn new(data: Vec<Vec<Option<Antenna>>>) -> Self {
        Self(data)
    }

    pub fn parse(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| if c == '.' { None } else { Some(Antenna(c)) })
                    .collect()
            })
            .collect();
        Self::new(data)
    }

    pub fn rows(&self) -> usize {
        self.0.len()
    }

    pub fn cols(&self) -> usize {
        self.0[0].len()
    }
}

pub struct Map<'g>(&'g Grid, HashMap<char, Vec<(usize, usize)>>);

impl<'g> Map<'g> {
    pub fn scan(grid: &'g Grid) -> Self {
        let mut antenna_map = HashMap::<_, Vec<_>>::new();

        for (row_idx, row) in grid.0.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if let Some(Antenna(freq)) = cell {
                    antenna_map
                        .entry(*freq)
                        .or_default()
                        .push((row_idx, col_idx));
                }
            }
        }

        Self(grid, antenna_map)
    }

    pub fn positions(&self) -> &HashMap<char, Vec<(usize, usize)>> {
        &self.1
    }

    pub fn antinodes(&self) -> Vec<(usize, usize)> {
        let mut antinodes = HashSet::new();

        for positions in self.1.values() {
            // Skip if only one antenna of this frequency
            if positions.len() < 2 {
                continue;
            }

            for i in 0..positions.len() {
                for j in (i + 1)..positions.len() {
                    let (r1, c1) = positions[i];
                    let (r2, c2) = positions[j];

                    let dr = r2 as isize - r1 as isize;
                    let dc = c2 as isize - c1 as isize;

                    let antinode1 = (r1 as isize - dr, c1 as isize - dc);
                    let antinode2 = (r2 as isize + dr, c2 as isize + dc);

                    for (ar, ac) in [antinode1, antinode2] {
                        if self.is_within_bounds(ar, ac) {
                            antinodes.insert((ar as usize, ac as usize));
                        }
                    }
                }
            }
        }

        antinodes.into_iter().collect()
    }

    fn resonant_antinodes(&self) -> Vec<(usize, usize)> {
        let mut antinodes = HashSet::new();

        for positions in self.1.values() {
            // Skip if only one antenna of this frequency
            if positions.len() < 2 {
                continue;
            }

            for i in 0..positions.len() {
                for j in (i + 1)..positions.len() {
                    let (r1, c1) = positions[i];
                    let (r2, c2) = positions[j];

                    let dr = r2 as isize - r1 as isize;
                    let dc = c2 as isize - c1 as isize;

                    // Add all positions along the line in both directions
                    // Starting from the first antenna, go backwards
                    let mut step = 0;
                    loop {
                        let ar = r1 as isize - step * dr;
                        let ac = c1 as isize - step * dc;

                        if self.is_within_bounds(ar, ac) {
                            antinodes.insert((ar as usize, ac as usize));
                            step += 1;
                        } else {
                            break;
                        }
                    }

                    // Starting from the first antenna, go forwards
                    step = 1; // Start at 1 to avoid duplicate at antenna position
                    loop {
                        let ar = r1 as isize + step * dr;
                        let ac = c1 as isize + step * dc;

                        if self.is_within_bounds(ar, ac) {
                            antinodes.insert((ar as usize, ac as usize));
                            step += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        antinodes.into_iter().collect()
    }

    fn is_within_bounds(&self, row: isize, col: isize) -> bool {
        row >= 0 && row < self.0.rows() as isize && col >= 0 && col < self.0.cols() as isize
    }
}

pub fn solve_part_one(input: &str) -> i64 {
    let grid = Grid::parse(input);
    let antenna_map = Map::scan(&grid);
    let antinodes = antenna_map.antinodes();

    antinodes.len() as i64
}

pub fn solve_part_two(input: &str) -> i64 {
    let grid = Grid::parse(input);
    let antenna_map = Map::scan(&grid);
    let antinodes = antenna_map.resonant_antinodes();

    antinodes.len() as i64
}

#[cfg(test)]
mod test {
    static SAMPLE: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn solve_sample_one() {
        let expected = 14;

        let actual = super::solve_part_one(SAMPLE);

        assert_eq!(expected, actual);
    }

    #[test]
    fn solve_sample_two() {
        let expected = 34;

        let actual = super::solve_part_two(SAMPLE);

        assert_eq!(expected, actual);
    }
}
