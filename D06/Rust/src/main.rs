use d06::{causes_loop, get_guard_path, get_start_position, parse_grid, Cell};

fn solve_part_one(grid: &[Vec<Cell>]) -> usize {
    let start_position = get_start_position(grid);
    let original_path = get_guard_path(grid, start_position);
    original_path.len()
}

fn solve_part_two(grid: &[Vec<Cell>]) -> usize {
    let start_position = get_start_position(grid);
    let original_path = get_guard_path(grid, start_position);

    original_path
        .iter()
        .filter(|&&position| position != start_position)
        .filter(|&&position| causes_loop(grid, start_position, position))
        .count()
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
        let expected = 6;

        let grid = super::parse_grid(SAMPLE);

        let actual = super::solve_part_two(&grid);

        assert_eq!(expected, actual);
    }
}
