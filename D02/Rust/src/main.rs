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

fn is_record_safe(record: &[i32]) -> bool {
    let mut first = true;
    let mut ascending = false;
    for pair in record.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        let d = (a - b).abs();
        if !(1..=3).contains(&d) {
            return false;
        }
        if first {
            first = false;
            ascending = a > b;
        } else if a > b && !ascending || a < b && ascending {
            return false;
        }
    }
    true
}

fn solve_part_one(input: &str) -> usize {
    process_records(input)
        .iter()
        .filter(|r| is_record_safe(r))
        .count()
}

fn solve_part_two(input: &str) -> i32 {
    let records = process_records(input);

    let mut safe_records = 0;

    for record in records {
        if is_record_safe(&record) {
            safe_records += 1;
        } else {
            let num_elements = record.len();
            for i in 0..num_elements {
                let mut record = record.clone();
                record.remove(i);
                if is_record_safe(&record) {
                    safe_records += 1;
                    break;
                }
            }
        }
    }

    safe_records
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
