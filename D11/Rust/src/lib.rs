use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rule {
    Replace,
    Split,
    Scale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Stone(u64);

impl Stone {
    pub fn new(weight: u64) -> Self {
        Self(weight)
    }

    pub fn apply_rule(&self) -> Vec<Stone> {
        match self.find_applicable_rule() {
            Rule::Replace => vec![Stone::new(1)],
            Rule::Split => {
                let divisor = 10_u64.pow(if self.0 == 0 { 1 } else { self.0.ilog10() + 1 } / 2);
                vec![
                    // First half: Top n/2 digits
                    Stone::new(self.0 / divisor),
                    // Second half: Bottom n/2 digits
                    Stone::new(self.0 % divisor),
                ]
            }
            Rule::Scale => vec![Stone::new(self.0 * 2024)],
        }
    }

    fn find_applicable_rule(&self) -> Rule {
        if self.0 == 0 {
            Rule::Replace
        } else if (self.0.ilog10() + 1).is_multiple_of(2) {
            Rule::Split
        } else {
            Rule::Scale
        }
    }
}

pub fn parse(input: &str) -> Vec<Stone> {
    input
        .split_whitespace()
        .filter_map(|x| x.parse::<u64>().map(Stone).ok())
        .collect()
}

pub fn blink(stones: &[Stone], num_blinks: usize) -> HashMap<Stone, usize> {
    let mut counts = HashMap::new();
    for stone in stones {
        counts.entry(*stone).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut updates: Vec<(Stone, usize)> = Vec::new();

    for _ in 0..num_blinks {
        updates.clear();

        for (&stone, &count) in &counts {
            // Collect the transformations
            for new_stone in stone.apply_rule() {
                updates.push((new_stone, count));
            }
        }

        counts.clear();
        for (stone, count) in &updates {
            counts
                .entry(*stone)
                .and_modify(|c| *c += count)
                .or_insert(*count);
        }
    }

    counts
}

pub fn solve_part_one(input: &str) -> usize {
    let numbers = parse(input);
    let sum = blink(&numbers, 25).values().sum();
    debug_assert_eq!(sum, 224529);
    sum
}

pub fn solve_part_two(input: &str) -> usize {
    let numbers = parse(input);
    let sum = blink(&numbers, 75).values().sum();
    debug_assert_eq!(sum, 266820198587914);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_1: &str = "0 1 10 99 999";
    static SAMPLE_2: &str = "125 17";
    static SAMPLE_3: &str = "1 1";

    #[test]
    fn test_parse() {
        let parsed = parse(SAMPLE_1);
        assert_eq!(
            parsed,
            vec![
                Stone::new(0),
                Stone::new(1),
                Stone::new(10),
                Stone::new(99),
                Stone::new(999)
            ]
        );
    }

    #[test]
    fn test_parse_2() {
        let parsed = parse(SAMPLE_2);
        assert_eq!(parsed, vec![Stone::new(125), Stone::new(17)]);
    }

    #[test]
    fn test_apply_rule_replace() {
        let stone = Stone::new(0);
        assert_eq!(stone.find_applicable_rule(), Rule::Replace);
        let result = stone.apply_rule();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, 1);
    }

    #[test]
    fn test_apply_rule_split_even() {
        let stone = Stone::new(1234);
        assert_eq!(stone.find_applicable_rule(), Rule::Split);
        let result = stone.apply_rule();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, 12);
        assert_eq!(result[1].0, 34);
    }

    #[test]
    fn test_apply_rule_multiply() {
        let stone = Stone::new(2);
        let result = stone.apply_rule();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, 4048);
    }

    #[test]
    fn test_apply_rule_split_odd() {
        let stone = Stone::new(123);
        assert_eq!(stone.find_applicable_rule(), Rule::Scale);
        let result = stone.apply_rule();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, 123 * 2024);
    }

    #[test]
    fn test_blink_once_sample_1() {
        let stones = parse(SAMPLE_1);
        let result = blink(&stones, 1);
        assert_eq!(result.len(), 5);
        assert_eq!(
            result,
            HashMap::from_iter([
                (Stone::new(1), 2),
                (Stone::new(2024), 1),
                (Stone::new(0), 1),
                (Stone::new(9), 2),
                (Stone::new(2021976), 1),
            ])
        );
    }

    #[test]
    fn test_blink_once_sample_2() {
        let stones = parse(SAMPLE_2);
        let result = blink(&stones, 1);
        assert_eq!(result.len(), 3);
        assert_eq!(
            result,
            HashMap::from_iter([
                (Stone::new(125 * 2024), 1), // from 125 -> Scale
                (Stone::new(1), 1),          // from 17 -> Split
                (Stone::new(7), 1),          // from 17 -> Split
            ])
        );
    }

    #[test]
    fn test_blink_once_sample_3() {
        let stones = parse(SAMPLE_3);
        let result = blink(&stones, 1);
        assert_eq!(result.len(), 1);
        assert_eq!(
            result,
            HashMap::from_iter([
                (Stone::new(2024), 2),       // from 1 -> Scale
            ])
        );
    }

    #[test]
    fn test_blink_part_one() {
        let stones = parse(SAMPLE_2);
        let result = blink(&stones, 25);
        assert_eq!(result.values().sum::<usize>(), 55312);
    }
}
