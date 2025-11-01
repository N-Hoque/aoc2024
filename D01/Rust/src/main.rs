use itertools::Itertools;

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .filter_map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect_tuple()
        })
        .unzip()
}

fn solve_part_one(input: &str) -> i32 {
    let (mut left, mut right) = parse(input);

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .fold(0, |acc, (a, b)| acc + (a - b).abs())
}

fn solve_part_two(input: &str) -> usize {
    let (left, right) = parse(input);

    let right_counts = right.into_iter().counts();

    left.into_iter().fold(0, |acc, l| {
        acc + (l as usize) * right_counts.get(&l).unwrap_or(&0)
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string("../input.txt")?;

    let a1 = solve_part_one(&data);

    println!("{}", a1);

    let a2 = solve_part_two(&data);

    println!("{}", a2);

    Ok(())
}

#[cfg(test)]
mod test {
    static SAMPLE: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn solve_sample_one() {
        let expected = 11;
        let actual = super::solve_part_one(SAMPLE);

        assert_eq!(expected, actual)
    }

    #[test]
    fn solve_sample_two() {
        let expected = 31;
        let actual = super::solve_part_two(SAMPLE);

        assert_eq!(expected, actual)
    }
}
