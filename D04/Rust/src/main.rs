static XMAS: &str = "XMAS";

const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (-1, 0),
    (1, 0),
    (1, 1),
    (1, -1),
    (-1, -1),
    (-1, 1),
];

#[derive(Default)]
struct Table {
    cells: Vec<Vec<char>>,
    row_count: usize,
    col_count: usize,
}

impl Table {
    pub fn new(input: &str) -> Self {
        let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        Table {
            row_count: cells.len(),
            col_count: cells.first().map_or(0, |row| row.len()),
            cells,
        }
    }

    fn coordinate_map(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.row_count).flat_map(|x| (0..self.col_count).map(move |y| (x, y)))
    }
}

fn scan_table(table: &Table, x: usize, y: usize) -> usize {
    if table.cells[x][y] != 'X' {
        0
    } else {
        DIRECTIONS
            .iter()
            .filter(|(dx, dy)| {
                XMAS.chars().enumerate().all(|(step, ch)| {
                    let nx = x as isize + dx * step as isize;
                    let ny = y as isize + dy * step as isize;

                    nx >= 0
                        && nx < table.row_count as isize
                        && ny >= 0
                        && ny < table.col_count as isize
                        && table.cells[nx as usize][ny as usize] == ch
                })
            })
            .count()
    }
}

fn scan_table_cross(table: &Table, x: usize, y: usize) -> bool {
    if table.cells[x][y] != 'A'
        || x == 0
        || y == 0
        || x >= table.row_count - 1
        || y >= table.col_count - 1
    {
        return false;
    }

    let corners = [
        table.cells[x - 1][y - 1], // tl
        table.cells[x - 1][y + 1], // tr
        table.cells[x + 1][y - 1], // bl
        table.cells[x + 1][y + 1], // br
    ];

    let diag1 = format!("{}A{}", corners[0], corners[3]);
    let diag2 = format!("{}A{}", corners[1], corners[2]);

    (diag1 == "MAS" || diag1 == "SAM") && (diag2 == "MAS" || diag2 == "SAM")
}

fn solve_part_one(input: &str) -> usize {
    let table = Table::new(input);

    table
        .coordinate_map()
        .fold(0, |acc, (x, y)| acc + scan_table(&table, x, y))
}

fn solve_part_two(input: &str) -> usize {
    let table = Table::new(input);

    table
        .coordinate_map()
        .filter(|&(x, y)| scan_table_cross(&table, x, y))
        .count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string("../input.txt")?;

    let p1 = solve_part_one(&data);

    println!("{p1}");

    let p2 = solve_part_two(&data);

    println!("{p2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    static SMALL: &str = r"..X...
.SAMX.
.A..A.
XMAS.S
.X....";

    static SAMPLE: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_solve_small_one() {
        let expected = 4;

        let actual = super::solve_part_one(SMALL);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_solve_small_two() {
        let expected = 0;

        let actual = super::solve_part_two(SMALL);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_solve_one() {
        let expected = 18;

        let actual = super::solve_part_one(SAMPLE);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_solve_two() {
        let expected = 9;

        let actual = super::solve_part_two(SAMPLE);

        assert_eq!(expected, actual);
    }
}
