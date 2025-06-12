use std::path::Path;

fn main() {
    println!("Quest 2: Tangled Trees");

    let (left_tree, right_tree) = parse_and_build("everybody_codes_e1_q02_p1.txt");
    let left = left_tree.widest_level();
    let right = right_tree.widest_level();
    println!("  part 1 = {left}{right}");

    // let (left_tree, right_tree) = parse_and_build("everybody_codes_e1_q02_p2.txt");
    // let left = left_tree.widest_level();
    // let right = right_tree.widest_level();
    // println!("  part 2 = {left}{right}");
}

fn parse_and_build<P: AsRef<Path>>(path: P) -> (VecTree, VecTree) {
    let lines = aoclib::read_lines(path);
    // let lines = aoclib::read_lines("test1_1.txt");

    let mut left_tree = VecTree::default();
    let mut right_tree = VecTree::default();

    for line in lines {
        let words = line.split(' ').collect::<Vec<_>>();
        let (_, id) = words[1].split_once('=').unwrap();
        let id = id.parse().unwrap();

        let (left_rank, left_symbol) = rank_and_symbol(words[2]);
        let (right_rank, right_symbol) = rank_and_symbol(words[3]);

        let left_node = Node::new(id, left_rank, left_symbol);
        let right_node = Node::new(id, right_rank, right_symbol);

        left_tree.add(left_node);
        right_tree.add(right_node);
    }

    (left_tree, right_tree)
}

fn rank_and_symbol(s: &str) -> (usize, char) {
    let (rank, symbol) = s
        .split_once("[")
        .unwrap()
        .1
        .split_once(']')
        .unwrap()
        .0
        .split_once(',')
        .unwrap();

    (rank.parse().unwrap(), symbol.chars().next().unwrap())
}

#[derive(Debug, Default)]
struct Node {
    _id: usize,
    rank: usize,
    symbol: char,
}

impl Node {
    fn new(id: usize, rank: usize, symbol: char) -> Self {
        Self {
            _id: id,
            rank,
            symbol,
        }
    }
}

#[derive(Debug, Default)]
struct VecTree {
    vec: Vec<Option<Node>>,
}

impl VecTree {
    fn add(&mut self, node: Node) {
        self.insert(node, 0);
    }

    // pos -> start of search
    // [root: 0, left-root: 1, right-root: 2, left-1: 3, right-1: 4, ...]
    // left node: pos * 2 + 1
    // right node: pos * 2 + 2
    fn insert(&mut self, node: Node, pos: usize) {
        if pos >= self.vec.len() {
            self.vec.resize_with(pos + 1, Default::default);
            self.vec[pos] = Some(node);
        } else if let Some(Some(entry)) = self.vec.get(pos) {
            if entry.rank > node.rank {
                self.insert(node, pos * 2 + 1);
            } else if entry.rank < node.rank {
                self.insert(node, pos * 2 + 2);
            } else {
                panic!("{node:?} == {entry:?}");
            }
        } else {
            self.vec[pos] = Some(node);
        }
    }

    fn widest_level(&self) -> String {
        self.levels()
            .into_iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
    }

    fn levels(&self) -> Vec<String> {
        let mut result = Vec::new();
        self.traverse(0, 0, &mut result);
        result
    }

    fn traverse(&self, pos: usize, depth: usize, levels: &mut Vec<String>) {
        if self.has(pos * 2 + 1) {
            self.traverse(pos * 2 + 1, depth + 1, levels);
        }

        if depth >= levels.len() {
            levels.resize_with(depth + 1, Default::default);
        }
        levels[depth].push(self.vec[pos].as_ref().unwrap().symbol);

        if self.has(pos * 2 + 2) {
            self.traverse(pos * 2 + 2, depth + 1, levels);
        }
    }

    fn has(&self, pos: usize) -> bool {
        pos < self.vec.len() && self.vec[pos].is_some()
    }
}
