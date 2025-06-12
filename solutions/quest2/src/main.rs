use std::path::Path;

fn main() {
    println!("Quest 2: Tangled Trees");

    let (left_tree, right_tree) =
        parse_and_build("everybody_codes_e1_q02_p1.txt", SwapOp::SingleNode);
    let left = left_tree.widest_level();
    let right = right_tree.widest_level();
    println!("  part 1 = {left}{right}");

    let (left_tree, right_tree) =
        parse_and_build("everybody_codes_e1_q02_p2.txt", SwapOp::SingleNode);
    // let (left_tree, right_tree) = parse_and_build("test2_1.txt");
    let left = left_tree.widest_level();
    let right = right_tree.widest_level();
    println!("  part 2 = {left}{right}");

    let (left_tree, right_tree) = parse_and_build("everybody_codes_e1_q02_p3.txt", SwapOp::Subtree);
    // let (left_tree, right_tree) = parse_and_build("test3_2.txt", SwapOp::Subtree);
    let left = left_tree.widest_level();
    let right = right_tree.widest_level();
    println!("  part 3 = {left}{right}");
}

fn parse_and_build<P: AsRef<Path>>(path: P, swap_op: SwapOp) -> (VecTree, VecTree) {
    let lines = aoclib::read_lines(path);
    // let lines = aoclib::read_lines("test1_1.txt");

    let mut left_tree = VecTree::default();
    let mut right_tree = VecTree::default();

    for line in lines {
        let words = line.split(' ').collect::<Vec<_>>();
        match words[0] {
            "ADD" => {
                let (_, id) = words[1].split_once('=').unwrap();
                let id = id.parse().unwrap();

                let (left_rank, left_symbol) = rank_and_symbol(words[2]);
                let (right_rank, right_symbol) = rank_and_symbol(words[3]);

                let left_node = Node::new(id, left_rank, left_symbol);
                let right_node = Node::new(id, right_rank, right_symbol);

                left_tree.add(left_node);
                right_tree.add(right_node);
            }
            "SWAP" => {
                let id = words[1].parse::<usize>().unwrap();
                let found_left = left_tree.find_by_id(id);
                let found_right = right_tree.find_by_id(id);

                match swap_op {
                    SwapOp::SingleNode => {
                        left_tree.vec[found_left.pos()] = Some(found_right.node());
                        right_tree.vec[found_right.pos()] = Some(found_left.node());
                    }
                    SwapOp::Subtree => {
                        if found_left.entry.is_empty() {
                            let p1 = found_right.entry[0].1;
                            let p2 = found_right.entry[1].1;
                            let mut r1 = right_tree.extract(p1);
                            let mut r2 = right_tree.extract(p2);
                            right_tree.plant(p1, &mut r2);
                            right_tree.plant(p2, &mut r1);
                        } else if found_right.entry.is_empty() {
                            let p1 = found_left.entry[0].1;
                            let p2 = found_left.entry[1].1;
                            let mut l1 = left_tree.extract(p1);
                            let mut l2 = left_tree.extract(p2);
                            left_tree.plant(p1, &mut l2);
                            left_tree.plant(p2, &mut l1);
                        } else {
                            let mut left = left_tree.extract(found_left.pos());
                            let mut right = right_tree.extract(found_right.pos());
                            left_tree.plant(found_left.pos(), &mut right);
                            right_tree.plant(found_right.pos(), &mut left);
                        }
                    }
                }
            }
            _ => {
                panic!("Invalid command {}", words[0]);
            }
        }
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
struct Found {
    entry: Vec<(Node, usize)>,
}

impl Found {
    fn pos(&self) -> usize {
        self.entry[0].1
    }

    fn node(&self) -> Node {
        self.entry[0].0.clone()
    }
}

#[derive(Debug)]
enum SwapOp {
    SingleNode,
    Subtree,
}

#[derive(Debug, Default, Clone)]
struct Node {
    id: usize,
    rank: usize,
    symbol: char,
}

impl Node {
    fn new(id: usize, rank: usize, symbol: char) -> Self {
        Self { id, rank, symbol }
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
        let mut result: Option<String> = None;
        let levels = self.levels();
        for level in levels {
            if result.is_none() || result.as_ref().unwrap().len() < level.len() {
                result = Some(level.clone());
            }
        }

        result.unwrap()
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

    fn find_by_id(&self, id: usize) -> Found {
        let mut result = Found::default();
        for (pos, node) in self.vec.iter().enumerate() {
            if let Some(node) = node {
                if node.id == id {
                    result.entry.push((node.clone(), pos));
                }
            }
        }

        result
    }

    fn extract(&mut self, pos: usize) -> VecTree {
        let mut result = VecTree::default();
        self.extract_subtree(pos, 0, &mut result);
        result
    }

    fn extract_subtree(&mut self, src: usize, dst: usize, result: &mut VecTree) {
        if self.has(src * 2 + 1) {
            self.extract_subtree(src * 2 + 1, dst * 2 + 1, result);
        }

        let node = self.vec[src].take();
        if dst >= result.vec.len() {
            result.vec.resize_with(dst + 1, Default::default);
        }
        result.vec[dst] = node;

        if self.has(src * 2 + 2) {
            self.extract_subtree(src * 2 + 2, dst * 2 + 2, result);
        }
    }

    fn plant(&mut self, pos: usize, other: &mut VecTree) {
        self.plant_subtree(pos, 0, other);
    }

    fn plant_subtree(&mut self, dst: usize, src: usize, other: &mut VecTree) {
        if other.has(src * 2 + 1) {
            self.plant_subtree(dst * 2 + 1, src * 2 + 1, other);
        }

        if dst >= self.vec.len() {
            self.vec.resize_with(dst + 1, Default::default);
        }
        self.vec[dst] = other.vec[src].take();

        if other.has(src * 2 + 2) {
            self.plant_subtree(dst * 2 + 2, src * 2 + 2, other);
        }
    }
}
