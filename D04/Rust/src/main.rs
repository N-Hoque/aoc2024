const XMAS_SPAN: isize = 3;

static XMAS_EXTENTS: &[Point] = &[
    Point(0, XMAS_SPAN),
    Point(0, -XMAS_SPAN),
    Point(-XMAS_SPAN, 0),
    Point(XMAS_SPAN, 0),
    Point(XMAS_SPAN, XMAS_SPAN),
    Point(XMAS_SPAN, -XMAS_SPAN),
    Point(-XMAS_SPAN, -XMAS_SPAN),
    Point(-XMAS_SPAN, XMAS_SPAN),
];

static XMAS_CROSS_EXTENTS: &[Point] = &[Point(1, 1), Point(1, -1), Point(-1, -1), Point(-1, 1)];

#[derive(Default)]
struct Table {
    cells: Vec<Vec<char>>,
    row_count: usize,
    col_count: usize,
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = String::new();
        for row in &self.cells {
            let mut row_str = String::new();
            for cell in row {
                row_str.push(*cell);
            }
            table += &(row_str + "\n");
        }
        write!(f, "{table}")
    }
}

#[derive(Debug)]
struct Point(isize, isize);

struct Bound {
    start_x: usize,
    end_x: usize,
    start_y: usize,
    end_y: usize,
}

impl std::fmt::Display for Bound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}) -> ({}, {})",
            self.start_x, self.start_y, self.end_x, self.end_y
        )
    }
}

fn process_table(input: &str) -> Table {
    let mut table = Table::default();

    let mut rows = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for ch in line.chars() {
            row.push(ch);
        }
        table.col_count = row.len();
        rows.push(row);
    }
    table.row_count = rows.len();
    table.cells = rows;
    table
}

fn scan_table(table: &Table, x: usize, y: usize) -> Vec<Bound> {
    let mut xmas_points = Vec::new();

    if table.cells[x][y] != 'X' {
        return Vec::new();
    }

    for extent in XMAS_EXTENTS {
        let max_x = (x as isize) + extent.0;
        let max_y = (y as isize) + extent.1;
        if !(max_x >= 0
            && max_x < (table.row_count as isize)
            && max_y >= 0
            && max_y < (table.col_count as isize))
        {
            continue;
        }

        let mut xmas_check = String::new();
        for step in 0..4 {
            let mut cur_x = x as isize;
            if max_x - (x as isize) == XMAS_SPAN {
                cur_x += step;
            } else if max_x - (x as isize) == -XMAS_SPAN {
                cur_x -= step;
            }

            let mut cur_y = y as isize;
            if max_y - (y as isize) == XMAS_SPAN {
                cur_y += step;
            } else if max_y - (y as isize) == -XMAS_SPAN {
                cur_y -= step;
            }
            xmas_check.push(table.cells[cur_x as usize][cur_y as usize]);
        }
        if xmas_check == "XMAS" {
            xmas_points.push(Bound {
                start_x: x,
                start_y: y,
                end_x: max_x as usize,
                end_y: max_y as usize,
            });
        }
    }

    xmas_points
}

fn scan_table_cross(table: &Table, x: usize, y: usize) -> Option<Point> {
    if table.cells[x][y] != 'A' {
        return None;
    }

    for extent in XMAS_CROSS_EXTENTS {
        let max_x = (x as isize) + extent.0;
        let max_y = (y as isize) + extent.1;
        if !(max_x >= 0
            && max_x < (table.row_count as isize)
            && max_y >= 0
            && max_y < (table.col_count as isize))
        {
            return None;
        }
    }

    let tl = table.cells[x - 1][y - 1];
    let tr = table.cells[x - 1][y + 1];
    let bl = table.cells[x + 1][y - 1];
    let br = table.cells[x + 1][y + 1];

    let mut m1 = String::new();
    m1.push(tl);
    m1.push('A');
    m1.push(br);

    let mut m2 = String::new();
    m2.push(tr);
    m2.push('A');
    m2.push(bl);

    if (m1 == "MAS" || m1 == "SAM") && (m2 == "MAS" || m2 == "SAM") {
        return Some(Point(x as isize, y as isize));
    }

    None
}

fn solve_part_one(input: &str) -> usize {
    let table = process_table(input);

    let mut total = 0;

    for (idx, row) in table.cells.iter().enumerate() {
        for (jdx, _) in row.iter().enumerate() {
            total += scan_table(&table, idx, jdx).len();
        }
    }

    total
}

fn solve_part_two(input: &str) -> u32 {
    let table = process_table(input);

    let mut total = 0;

    for (idx, row) in table.cells.iter().enumerate() {
        for (jdx, _) in row.iter().enumerate() {
            if scan_table_cross(&table, idx, jdx).is_some() {
                total += 1;
            }
        }
    }

    total
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
