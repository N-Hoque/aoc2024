fn solve_part_one(input: &str) -> i32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let values = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<_>>();
        left.push(values[0]);
        right.push(values[1]);
    }

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn solve_part_two(input: &str) -> i32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let values = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<_>>();
        left.push(values[0]);
        right.push(values[1]);
    }

    let mut occurence_map = std::collections::HashMap::new();
    for l in left {
        let counts = right.iter().filter(|x| **x == l).count() as i32;
        occurence_map
            .entry(l)
            .and_modify(|x| *x += counts)
            .or_insert(counts);
    }

    occurence_map.into_iter().map(|(a, b)| a * b).sum()
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
