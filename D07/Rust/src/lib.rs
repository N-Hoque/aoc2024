use std::collections::VecDeque;

pub fn solve_part_one(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .fold(0, |acc, (target, values)| {
            acc + if is_valid(
                values[0],
                target,
                &values[1..],
                &[Instruction::Add, Instruction::Multiply],
            ) {
                target
            } else {
                0
            }
        })
}

pub fn solve_part_two(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .fold(0, |acc, (target, values)| {
            acc + if is_valid(
                values[0],
                target,
                &values[1..],
                &[
                    Instruction::Add,
                    Instruction::Multiply,
                    Instruction::Concatenate,
                ],
            ) {
                target
            } else {
                0
            }
        })
}

pub fn solve_part_one_iterative(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .fold(0, |acc, (target, values)| {
            acc + if is_valid_iterative(
                values[0],
                target,
                &values[1..],
                &[Instruction::Add, Instruction::Multiply],
            ) {
                target
            } else {
                0
            }
        })
}

pub fn solve_part_two_iterative(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .fold(0, |acc, (target, values)| {
            acc + if is_valid_iterative(
                values[0],
                target,
                &values[1..],
                &[
                    Instruction::Add,
                    Instruction::Multiply,
                    Instruction::Concatenate,
                ],
            ) {
                target
            } else {
                0
            }
        })
}

pub enum Instruction {
    Add,
    Multiply,
    Concatenate,
}

pub fn parse_line(input: &str) -> (i64, Vec<i64>) {
    let values = input.split(": ").collect::<Vec<_>>();
    (
        values[0].parse().unwrap(),
        values[1]
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect(),
    )
}

pub fn concatenate_str(first: i64, second: i64) -> i64 {
    first
        .to_string()
        .chars()
        .chain(second.to_string().chars())
        .collect::<String>()
        .parse()
        .unwrap()
}

pub fn concatenate(first: i64, second: i64) -> i64 {
    first * 10i64.pow(if second == 0 { 1 } else { second.ilog10() + 1 }) + second
}

pub fn is_valid(first: i64, target: i64, values: &[i64], instructions: &[Instruction]) -> bool {
    if values.is_empty() {
        return first == target;
    }

    let second = values[0];

    instructions
        .iter()
        .map(|i| match i {
            Instruction::Add => first + second,
            Instruction::Multiply => first * second,
            Instruction::Concatenate => concatenate(first, second),
        })
        .any(|res| is_valid(res, target, &values[1..], instructions))
}

pub fn is_valid_iterative(
    first: i64,
    target: i64,
    values: &[i64],
    instructions: &[Instruction],
) -> bool {
    if values.is_empty() {
        return first == target;
    }

    let mut queue = VecDeque::from([(first, 0)]);

    while let Some((current, index)) = queue.pop_front() {
        if index >= values.len() {
            if current == target {
                return true;
            }
            continue;
        }

        let next_value = values[index];

        queue.extend(
            instructions
                .iter()
                .map(|instruction| {
                    let result = match instruction {
                        Instruction::Add => current + next_value,
                        Instruction::Multiply => current * next_value,
                        Instruction::Concatenate => concatenate(current, next_value),
                    };
                    (result, index + 1)
                })
                .filter(|(result, idx)| *result <= target || *idx == values.len()),
        );
    }

    false
}
