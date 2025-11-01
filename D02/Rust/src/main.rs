fn process_records(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

fn compare_pair_with_bounds(a: i32, b: i32) -> bool {
    a <= b && (1..=3).contains(&(a - b).abs())
}

fn is_record_safe(record: &[i32]) -> bool {
    record
        .iter()
        .is_sorted_by(|a, b| compare_pair_with_bounds(**a, **b))
        || record
            .iter()
            .is_sorted_by(|a, b| compare_pair_with_bounds(**b, **a))
}

fn is_record_safe_skip(record: &[i32], skip_index: usize) -> bool {
    let filtered: Vec<i32> = record
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| if i == skip_index { None } else { Some(x) })
        .collect();

    is_record_safe(&filtered)
}

fn is_record_safe_anywhere(record: &[i32]) -> bool {
    (0..record.len()).any(|idx| is_record_safe_skip(record, idx))
}

fn solve_part_one(input: &str) -> usize {
    process_records(input)
        .iter()
        .filter(|r| is_record_safe(r))
        .count()
}

fn solve_part_two(input: &str) -> usize {
    process_records(input)
        .iter()
        .filter(|r| is_record_safe(r) || is_record_safe_anywhere(r))
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
    static SAMPLE: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_sample_one() {
        let expected = 2;
        let actual = super::solve_part_one(SAMPLE);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sample_two() {
        let expected = 4;
        let actual = super::solve_part_two(SAMPLE);
        assert_eq!(expected, actual);
    }
}
