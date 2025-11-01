use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct Node(i32);

#[derive(Debug)]
struct Edge(Node, Node);

#[derive(Debug, Default)]
struct Graph(Vec<Edge>);

impl Graph {
    fn ordering(&self, n1: Node, n2: Node) -> Ordering {
        if self.0.iter().any(|Edge(from, to)| *from == n1 && *to == n2) {
            Ordering::Less
        } else if self.0.iter().any(|Edge(from, to)| *from == n2 && *to == n1) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

fn parse(input: &str) -> (Graph, Vec<Vec<Node>>) {
    let mut edges = Vec::new();
    let mut updates = Vec::new();

    for line in input.lines() {
        if line.contains('|') {
            if let Some((left, right)) = line.split_once('|') {
                if let (Ok(a), Ok(b)) = (left.parse::<i32>(), right.parse::<i32>()) {
                    edges.push(Edge(Node(a), Node(b)));
                }
            }
        } else if !line.is_empty() {
            let update = line
                .split(',')
                .filter_map(|x| x.parse::<i32>().map(Node).ok())
                .collect();
            updates.push(update);
        }
    }

    (Graph(edges), updates)
}

fn is_ordered(update: &[Node], graph: &Graph) -> bool {
    update.is_sorted_by(|a, b| graph.ordering(*a, *b) != Ordering::Greater)
}

fn solve_part_one(graph: &Graph, updates: &[Vec<Node>]) -> i32 {
    updates.iter().fold(0, |acc, update| {
        acc + if is_ordered(update, graph) {
            update.get(update.len() / 2).unwrap().0
        } else {
            0
        }
    })
}

fn solve_part_two(graph: &Graph, updates: &[Vec<Node>]) -> i32 {
    updates.iter().fold(0, |acc, update| {
        acc + if !is_ordered(update, graph) {
            update
                .iter()
                .sorted_by(|&&a, &&b| graph.ordering(a, b))
                .nth(update.len() / 2)
                .unwrap()
                .0
        } else {
            0
        }
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string("../input.txt")?;

    let (g, t) = parse(&data);

    let p1 = solve_part_one(&g, &t);

    println!("{p1}");

    let p2 = solve_part_two(&g, &t);

    println!("{p2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    static SAMPLE: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_solve_one() {
        let expected = 143;

        let (g, t) = super::parse(SAMPLE);

        let actual = super::solve_part_one(&g, &t);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_solve_two() {
        let expected = 123;

        let (g, t) = super::parse(SAMPLE);

        let actual = super::solve_part_two(&g, &t);

        assert_eq!(expected, actual);
    }
}
