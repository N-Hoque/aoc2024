use std::collections::{BTreeMap, HashMap, VecDeque};

type Position = (usize, usize);

pub struct TopographicalMap {
    cells: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub struct TrailNode {
    position: Position,
    height: u8,
    children: Vec<TrailNode>,
}

impl TrailNode {
    fn new(position: Position, height: u8) -> Self {
        TrailNode {
            position,
            height,
            children: Vec::new(),
        }
    }

    fn collect_peaks(&self) -> Vec<Position> {
        if self.height == 9 {
            vec![self.position]
        } else {
            self.children
                .iter()
                .flat_map(|child| child.collect_peaks())
                .collect()
        }
    }

    // Count unique endpoints (part 1)
    pub fn count_unique_peaks(&self) -> usize {
        if self.height == 9 {
            return 1;
        }
        self.children
            .iter()
            .flat_map(|child| child.collect_peaks())
            .collect::<std::collections::HashSet<_>>()
            .len()
    }

    // Count all distinct paths (part 2)
    pub fn count_all_paths(&self) -> usize {
        if self.height == 9 {
            1
        } else {
            self.children
                .iter()
                .fold(0, |acc, child| acc + child.count_all_paths())
        }
    }

    fn fmt_tree(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        prefix: &str,
        is_last: bool,
    ) -> std::fmt::Result {
        // Print current node with tree characters
        write!(f, "{}", prefix)?;
        write!(f, "{}", if is_last { "└── " } else { "├── " })?;
        writeln!(f, "({}, {})", self.position.0, self.position.1)?;

        // Prepare prefix for children
        let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

        // Print all children
        for (idx, child) in self.children.iter().enumerate() {
            let is_last_child = idx == self.children.len() - 1;
            child.fmt_tree(f, &child_prefix, is_last_child)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for TrailNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_tree(f, "", true)
    }
}

impl TopographicalMap {
    pub fn parse(input: &str) -> Self {
        TopographicalMap {
            cells: input
                .lines()
                .map(|line| {
                    line.chars()
                        .filter_map(|s| s.to_digit(10))
                        .map(|d| d as u8)
                        .collect()
                })
                .collect(),
        }
    }

    pub fn height_at_pos(&self, pos: Position) -> Option<u8> {
        let (x, y) = pos;
        self.cells.get(x).and_then(|row| row.get(y)).copied()
    }

    pub fn build_trails(&self) -> HashMap<Position, Vec<Vec<Position>>> {
        let mut all_trails: HashMap<Position, Vec<Vec<Position>>> = HashMap::new();

        // BFS from each start position to find all paths to height 9
        for start in self.cells.iter().enumerate().flat_map(|(x, col)| {
            col.iter()
                .enumerate()
                .filter_map(move |(y, &height)| if height == 0 { Some((x, y)) } else { None })
        }) {
            let mut queue = VecDeque::new();
            queue.push_back(vec![start]);

            while let Some(path) = queue.pop_front() {
                let pos = *path.last().unwrap();
                let current_height = self.height_at_pos(pos).unwrap();

                // If we reached height 9, save this trail
                if current_height == 9 {
                    all_trails
                        .entry(start)
                        .and_modify(|trails| {
                            trails.push(path.iter().skip(1).copied().collect::<Vec<_>>())
                        })
                        .or_insert_with(|| vec![path.iter().skip(1).copied().collect()]);
                    continue;
                }

                // Explore all four directions
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let new_pos = (
                        (pos.0 as isize + dx) as usize,
                        (pos.1 as isize + dy) as usize,
                    );

                    // Only move if height increases by exactly 1
                    if self
                        .height_at_pos(new_pos)
                        .is_some_and(|next_height| next_height == current_height + 1)
                    {
                        let mut new_path = path.clone();
                        new_path.push(new_pos);
                        queue.push_back(new_path);
                    }
                }
            }
        }

        all_trails
    }

    pub fn build_trail_trees(&self) -> BTreeMap<Position, TrailNode> {
        let mut trees = BTreeMap::new();

        for start in self.cells.iter().enumerate().flat_map(|(x, col)| {
            col.iter()
                .enumerate()
                .filter_map(move |(y, &height)| if height == 0 { Some((x, y)) } else { None })
        }) {
            let tree = self.build_tree_from(start);
            trees.insert(start, tree);
        }

        trees
    }

    fn build_tree_from(&self, pos: Position) -> TrailNode {
        let height = self.height_at_pos(pos).unwrap();
        let mut node = TrailNode::new(pos, height);

        if height == 9 {
            return node;
        }

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let new_pos = (
                (pos.0 as isize + dx) as usize,
                (pos.1 as isize + dy) as usize,
            );

            if self
                .height_at_pos(new_pos)
                .is_some_and(|next_height| next_height == height + 1)
            {
                node.children.push(self.build_tree_from(new_pos));
            }
        }

        node
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let map = TopographicalMap::parse(input);
    let trails = map.build_trail_trees();
    // display_trails(&trails);
    let result = trails
        .values()
        .fold(0, |acc, tree| acc + tree.count_unique_peaks());
    debug_assert_eq!(result, 796);
    result
}

pub fn solve_part_two(input: &str) -> usize {
    let map = TopographicalMap::parse(input);
    let trails = map.build_trail_trees();
    let result = trails
        .values()
        .fold(0, |acc, tree| acc + tree.count_all_paths());
    debug_assert_eq!(result, 1942);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn get_height_at_pos() {
        let map = TopographicalMap::parse(SAMPLE);
        assert_eq!(map.height_at_pos((0, 0)), Some(8));
        assert_eq!(map.height_at_pos((3, 4)), Some(9));
        assert_eq!(map.height_at_pos((6, 7)), Some(1));
        assert_eq!(map.height_at_pos((8, 9)), None);
    }

    #[test]
    fn test_solve_part_one() {
        let map = TopographicalMap::parse(SAMPLE);
        let trails = map.build_trail_trees();
        let result = trails
            .values()
            .fold(0, |acc, tree| acc + tree.count_unique_peaks());
        assert_eq!(result, 36); // Example expected result
    }

    #[test]
    fn test_solve_part_two() {
        let map = TopographicalMap::parse(SAMPLE);
        let trails = map.build_trail_trees();
        let result = trails
            .values()
            .fold(0, |acc, tree| acc + tree.count_all_paths());
        assert_eq!(result, 81); // Example expected result
    }
}
