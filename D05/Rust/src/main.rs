#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct Node(i32);

#[derive(Debug)]
struct Edge(Node, Node);

#[derive(Debug, Default)]
struct Graph(Vec<Edge>);

enum EdgeType {
    In,
    Out,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_edge(&mut self, e: Edge) {
        self.0.push(e);
    }

    pub fn find_edge_type(&self, n1: Node, n2: Node) -> Option<EdgeType> {
        self.0.iter().find_map(|Edge(m1, m2)| {
            if n1 == *m1 && n2 == *m2 {
                Some(EdgeType::Out)
            } else if n1 == *m2 && n2 == *m1 {
                Some(EdgeType::In)
            } else {
                None
            }
        })
    }
}

fn parse_pair(input: &str) -> Option<Edge> {
    input
        .split('|')
        .map(str::parse::<i32>)
        .collect::<Result<Vec<_>, _>>()
        .map_or(None, |values| Some(Edge(Node(values[0]), Node(values[1]))))
}

fn parse_line(input: &str) -> Vec<Node> {
    input
        .split(',')
        .filter_map(|x| x.parse::<i32>().map(Node).ok())
        .collect::<Vec<_>>()
}

fn parse(input: &str) -> (Graph, Vec<Vec<Node>>) {
    let mut g = Graph::new();
    let mut t = Vec::new();

    for line in input.lines() {
        if let Some(edge) = parse_pair(line) {
            g.add_edge(edge);
        } else if !line.is_empty() {
            t.push(parse_line(line));
        }
    }

    (g, t)
}

fn sort_update(update: &mut [Node], g: &Graph) {
    update.sort_by(|n1, n2| match g.find_edge_type(*n1, *n2) {
        Some(EdgeType::Out) => std::cmp::Ordering::Less,
        None => std::cmp::Ordering::Equal,
        Some(EdgeType::In) => std::cmp::Ordering::Greater,
    });
}

fn is_ordered(update: &[Node], g: &Graph) -> bool {
    update
        .windows(2)
        .all(|xs| !matches!(g.find_edge_type(xs[0], xs[1]), Some(EdgeType::In)))
}

fn solve_part_one(graph: &Graph, table: &[Vec<Node>]) -> i32 {
    table
        .iter()
        .filter(|update| is_ordered(update, graph))
        .map(|update| update[update.len() / 2].0)
        .sum()
}

fn solve_part_two(graph: &Graph, table: &mut [Vec<Node>]) -> i32 {
    table
        .iter_mut()
        .filter(|update| !is_ordered(update, graph))
        .map(|failed| {
            sort_update(failed, graph);
            failed[failed.len() / 2].0
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string("../input.txt")?;

    let (g, mut t) = parse(&data);

    let p1 = solve_part_one(&g, &t);

    println!("{p1}");

    let p2 = solve_part_two(&g, &mut t);

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

        let (g, mut t) = super::parse(SAMPLE);

        let actual = super::solve_part_two(&g, &mut t);

        assert_eq!(expected, actual);
    }
}
