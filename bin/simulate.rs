use rust_death_toy::{ Cell, Coord, kdtree };
use kdtree::{ KDTree, TreeNode };


fn main() {
    let origin = Coord::new(0.0, 0.0);
    let og_cell = Cell::new(0.0, 40 * 60, origin);

    let cells = vec![og_cell];
    let mut tree = KDTree::new(cells);

    for i in 0..(20000) {
        let mut to_add = Vec::<Cell>::new();
        let tree_iter = tree.tree.iter_mut()
                                 .filter_map(
                                     |x| match x {
                                         Some(TreeNode::Leaf(x)) => Some(x),
                                         _ => None
                                     }
                                 );
        for cell in tree_iter {
            match cell.step() {
                Some(x) => to_add.push(x),
                None => ()
            }
        }
        let mut cells = 
            tree.tree.iter()
            .filter_map(
                |x| match x {
                    Some(TreeNode::Leaf(x)) => Some(*x),
                    _ => None
                }
            )
            .collect::<Vec<Cell>>();
        cells.append(&mut to_add);
        tree = kdtree::KDTree::new(cells);
    }

    println!("{:?}", tree.tree.len());
}
