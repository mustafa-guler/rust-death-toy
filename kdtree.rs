use crate::{ Coord, Cell };
use rand::Rng;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::cmp;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy)]
pub enum TreeNode {
    Internal(f64),
    Leaf(Cell)
}

pub struct KDTree {
    pub tree: Vec<Option<TreeNode>>
}


// impl IntoIterator for KDTree {
//     type Item = Cell;
//     type IntoIter = ::std::vec::IntoIter<Self::Item>;
//     // type IntoIter = ::std::iter::Filter<::std::vec::IntoIter<Self::Item>>;

//     fn into_iter(self) -> Self::IntoIter {
//         let first_iter = self.tree.into_iter()
//         .filter(
//             |x| match x {
//                 Some(TreeNode::Leaf(x)) => true,
//                 _ => false
//             }
//         )
//         .map(
//             |x| match x {
//                 Some(TreeNode::Leaf(x)) => x,
//                 _ => panic!("impossible")
//             }
//         )
//         .collect::<Vec<Self::Item>>();

//         first_iter.into_iter()
//     }
// }


pub fn median(vals: &[f64]) -> f64 {
    if vals.len() == 0 { panic!("empty list"); }

    if vals.len() % 2 == 1 {
        quickselect(vals, vals.len() / 2)
    } else {
        0.5 * (
            quickselect(vals, vals.len() / 2 - 1) +
            quickselect(vals, vals.len() / 2)
        )
    }
}


fn quickselect(vals: &[f64], k: usize) -> f64 {
    if vals.len() == 1 {
        return vals[0];
    }

    // choose random index to pivot on
    let pivot: usize = rand::thread_rng().gen_range(0, vals.len());

    let mut pivot_cnt: usize = 0;
    let mut lows = Vec::<f64>::new();
    let mut highs = Vec::<f64>::new();

    for i in vals.iter() {
        if *i < vals[pivot] { lows.push(*i); }
        else if *i > vals[pivot] { highs.push(*i); }
        else { pivot_cnt += 1; }
    }

    match k {
        x if x < lows.len() => quickselect(&lows, k),
        x if x < lows.len() + pivot_cnt => vals[pivot],
        _ => quickselect(&highs, k - lows.len() - pivot_cnt)
    }
}


fn x_dim(node: &TreeNode) -> f64 {
    match node {
        TreeNode::Internal(val) => *val,
        TreeNode::Leaf(cell) => cell.pos.x
    }
}

fn y_dim(node: &TreeNode) -> f64 {
    match node {
        TreeNode::Internal(val) => *val,
        TreeNode::Leaf(cell) => cell.pos.y
    }
}

fn split_median<'a, 'b, F>(median: &'a TreeNode, leaves: &'b Vec<TreeNode>, dim_func: F) -> (Vec<TreeNode>, Vec<TreeNode>)
where F: Fn(&TreeNode) -> f64 {
    let mut left = Vec::<TreeNode>::new();
    let mut right = Vec::<TreeNode>::new();

    for leaf in leaves.iter() {
        if dim_func(leaf) <= dim_func(median) {
           left.push(*leaf);
        } else {
           right.push(*leaf);
        }
    }

    (left, right)
}

impl KDTree {
    pub fn new(cells: Vec<Cell>) -> KDTree {
        // convert cell vector to leaf nodes vector
        let leaves = cells.iter().map(|cell| TreeNode::Leaf(*cell)).collect::<Vec<TreeNode>>();

        // 2D vector, will pop 2^{level} from here each iteration
        let mut splits = VecDeque::<Vec<TreeNode>>::new();
        splits.push_back(leaves);

        // the tree itself is serialized into a vector
        let mut tree = Vec::<Option<TreeNode>>::new();

        // dim_selectors are functions that extra the x or y dimension from a tree node
        let dim_selectors = &[x_dim, y_dim];
        let mut level = 0usize;

        while splits.iter().map(|x| x.len()).sum::<usize>() > 0 {
            let num_pop = 2usize.pow(level.try_into().unwrap());
            assert!(num_pop <= splits.len());
            let curr_dim = level % 2;

            for _ in 0..num_pop {
                let curr_leaves = splits.pop_front().unwrap();
                if curr_leaves.len() == 1 {
                    tree.push(Some(curr_leaves[0]));
                    splits.push_back(vec![]);
                    splits.push_back(vec![]);
                    continue;
                } else if curr_leaves.len() == 0 {
                    tree.push(None);
                    splits.push_back(vec![]);
                    splits.push_back(vec![]);
                    continue;
                }

                let median = TreeNode::Internal(
                    median(&curr_leaves.iter().map(dim_selectors[curr_dim]).collect::<Vec<f64>>())
                );

                let (left, right) = split_median(&median, &curr_leaves, dim_selectors[curr_dim]);
                splits.push_back(left);
                splits.push_back(right);
                tree.push(Some(median));
            }

            level += 1;
        }

        assert_eq!((0..level).map(|x| 2usize.pow(x.try_into().unwrap())).sum::<usize>(), tree.len());
        KDTree { tree }
    }

    pub fn query(self, x_range: RangeInclusive<f64>, y_range: RangeInclusive<f64>) {
        let stack = Vec::<usize>::new();
        stack.push(0);
        let dim_selectors = &[x_dim, y_dim];
        let ranges = &[x_range, y_range];

        while stack.len() > 0 {
            let ind = stack.pop().unwrap();
            let curr_dim = ((ind as f64 + 1.0).log2() as usize) % 2;
            let curr_node = match self.tree[ind] {
                Some(x) => dim_selectors[curr_dim],
                None => panic!("None")
            };
        }
    }
}
