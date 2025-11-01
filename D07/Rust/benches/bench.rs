use criterion::{criterion_group, criterion_main, Criterion};
use d07::*; // Assuming your functions are public in lib.rs

const SAMPLE: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

fn bench_recursive_vs_iterative_part_one(c: &mut Criterion) {
    let mut group = c.benchmark_group("validation_methods_part_one");

    let test_cases: Vec<_> = SAMPLE.lines().map(parse_line).collect();
    let part_one_ops = &[Instruction::Add, Instruction::Multiply];

    group.bench_function("recursive", |b| {
        b.iter(|| {
            test_cases
                .iter()
                .filter(|(target, values)| {
                    is_valid(
                        std::hint::black_box(values[0]),
                        std::hint::black_box(*target),
                        std::hint::black_box(&values[1..]),
                        std::hint::black_box(part_one_ops),
                    )
                })
                .count()
        })
    });

    group.bench_function("iterative", |b| {
        b.iter(|| {
            test_cases
                .iter()
                .filter(|(target, values)| {
                    is_valid_iterative(
                        std::hint::black_box(values[0]),
                        std::hint::black_box(*target),
                        std::hint::black_box(&values[1..]),
                        std::hint::black_box(part_one_ops),
                    )
                })
                .count()
        })
    });

    group.finish();
}

fn bench_recursive_vs_iterative_part_two(c: &mut Criterion) {
    let mut group = c.benchmark_group("validation_methods_part_two");

    let test_cases: Vec<_> = SAMPLE.lines().map(parse_line).collect();
    let part_two_ops = &[
        Instruction::Add,
        Instruction::Multiply,
        Instruction::Concatenate,
    ];

    group.bench_function("recursive", |b| {
        b.iter(|| {
            test_cases
                .iter()
                .filter(|(target, values)| {
                    is_valid(
                        std::hint::black_box(values[0]),
                        std::hint::black_box(*target),
                        std::hint::black_box(&values[1..]),
                        std::hint::black_box(part_two_ops),
                    )
                })
                .count()
        })
    });

    group.bench_function("iterative", |b| {
        b.iter(|| {
            test_cases
                .iter()
                .filter(|(target, values)| {
                    is_valid_iterative(
                        std::hint::black_box(values[0]),
                        std::hint::black_box(*target),
                        std::hint::black_box(&values[1..]),
                        std::hint::black_box(part_two_ops),
                    )
                })
                .count()
        })
    });

    group.finish();
}

fn bench_concatenation_methods(c: &mut Criterion) {
    let mut group = c.benchmark_group("concatenation");

    let test_pairs = [(123, 456), (1, 2), (0, 0), (999999, 1), (42, 0)];

    group.bench_function("string", |b| {
        b.iter(|| {
            test_pairs
                .iter()
                .map(|&(a, b)| concatenate_str(std::hint::black_box(a), std::hint::black_box(b)))
                .sum::<i64>()
        })
    });

    group.bench_function("integer", |b| {
        b.iter(|| {
            test_pairs
                .iter()
                .map(|&(a, b)| concatenate(std::hint::black_box(a), std::hint::black_box(b)))
                .sum::<i64>()
        })
    });

    group.finish();
}

fn bench_full_solutions_part_one(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_solution_part_one");

    group.bench_function("recursive", |b| {
        b.iter(|| solve_part_one(std::hint::black_box(SAMPLE)))
    });

    group.bench_function("iterative", |b| {
        b.iter(|| solve_part_one_iterative(std::hint::black_box(SAMPLE)))
    });

    group.finish();
}

fn bench_full_solutions_part_two(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_solution_part_two");

    group.bench_function("recursive", |b| {
        b.iter(|| solve_part_two(std::hint::black_box(SAMPLE)))
    });

    group.bench_function("iterative", |b| {
        b.iter(|| solve_part_two_iterative(std::hint::black_box(SAMPLE)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_recursive_vs_iterative_part_one,
    bench_recursive_vs_iterative_part_two,
    bench_concatenation_methods,
    bench_full_solutions_part_one,
    bench_full_solutions_part_two,
);
criterion_main!(benches);
