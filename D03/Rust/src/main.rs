#![warn(clippy::pedantic, clippy::nursery)]

use d03::{
    lexer::Lexer,
    parser::{Expression, Parser},
};

fn solve_part_one(input: &str) -> u32 {
    let mut lexer = Lexer::new(input);
    let parser = Parser::new(&mut lexer, false);

    parser.fold(0, |acc, expr| match expr {
        Expression::Multiply(first, second) => acc + (first * second),
        Expression::Nop => acc,
    })
}

fn solve_part_two(input: &str) -> u32 {
    let mut lexer = Lexer::new(input);
    let parser = Parser::new(&mut lexer, true);

    parser.fold(0, |acc, expr| match expr {
        Expression::Multiply(first, second) => acc + (first * second),
        Expression::Nop => acc,
    })
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
    #[test]
    fn test_sample_one() {
        const SAMPLE: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let expected = 161;
        let actual = super::solve_part_one(SAMPLE);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sample_two() {
        const SAMPLE: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let expected = 48;
        let actual = super::solve_part_two(SAMPLE);
        assert_eq!(expected, actual);
    }
}
