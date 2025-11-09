use d11::{solve_part_one, solve_part_two};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string("../input.txt")?;

    let a1 = solve_part_one(&data);

    println!("{a1}");

    let a2 = solve_part_two(&data);

    println!("{a2}");

    Ok(())
}
