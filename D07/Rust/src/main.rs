enum Instruction {
    Add,
    Multiply,
    Concatenate,
}

impl From<&Instruction> for fn(i64, i64) -> i64 {
    fn from(value: &Instruction) -> Self {
        match value {
            Instruction::Add => add,
            Instruction::Multiply => mul,
            Instruction::Concatenate => cct,
        }
    }
}

const fn add(x: i64, y: i64) -> i64 {
    x + y
}

const fn mul(x: i64, y: i64) -> i64 {
    x * y
}

fn cct(x: i64, y: i64) -> i64 {
    x * 10i64.pow(if y == 0 {
        1
    } else {
        ((y as f64).log10().floor() + 1.0) as u32
    }) + y
}

fn parse_line(input: &str) -> (i64, Vec<i64>) {
    let values = input.split(": ").collect::<Vec<_>>();
    (
        values[0].parse().unwrap(),
        values[1]
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect(),
    )
}

fn is_valid(first: i64, target: i64, values: &[i64], instructions: &[Instruction]) -> bool {
    if values.is_empty() {
        return first == target;
    }

    instructions
        .iter()
        .map(<fn(i64, i64) -> i64>::from)
        .map(|f| f(first, values[0]))
        .any(|res| is_valid(res, target, &values[1..], instructions))
}

fn solve_part_one(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .filter(|(target, values)| {
            is_valid(
                values[0],
                *target,
                &values[1..],
                &[Instruction::Add, Instruction::Multiply],
            )
        })
        .map(|(target, _)| target)
        .sum()
}

fn solve_part_two(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .filter(|(target, values)| {
            is_valid(
                values[0],
                *target,
                &values[1..],
                &[
                    Instruction::Add,
                    Instruction::Multiply,
                    Instruction::Concatenate,
                ],
            )
        })
        .map(|(target, _)| target)
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string("../input.txt")?;

    let a1 = solve_part_one(&data);

    println!("{a1}");

    let a2 = solve_part_two(&data);

    println!("{a2}");

    Ok(())
}

#[cfg(test)]
mod test {
    static SAMPLE: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn solve_sample_one() {
        let expected = 3749;

        let actual = super::solve_part_one(SAMPLE);

        assert_eq!(expected, actual);
    }

    #[test]
    fn solve_sample_two() {
        let expected = 11387;

        let actual = super::solve_part_two(SAMPLE);

        assert_eq!(expected, actual);
    }
}
