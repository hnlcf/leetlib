use std::cell::RefCell;
use std::rc::Rc;

use crate::lt_tree::TreeNode;

/// Note: Define the height of `NULL` node is `-1`, and the height of
/// leaf node is `0`.
///
/// As well as considering edges number to count.
pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match node {
            Some(node_rc) => {
                let node_ref = node_rc.as_ref().borrow();
                let (l_height, l_diameter) = dfs(&node_ref.left);
                let (r_height, r_diameter) = dfs(&node_ref.right);

                let height = l_height.max(r_height) + 1;
                let diameter = (l_height + r_height + 2).max(l_diameter).max(r_diameter);
                (height, diameter)
            }
            None => (-1, -1),
        }
    }

    dfs(&root).1
}

pub fn diameter_of_binary_tree_v2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
        match node {
            Some(node_rc) => {
                let node_ref = node_rc.as_ref().borrow();
                let left = dfs(&node_ref.left, diameter);
                let right = dfs(&node_ref.right, diameter);
                *diameter = *diameter.max(&mut (left + right + 2));

                left.max(right) + 1
            }
            None => -1,
        }
    }

    let mut diameter = 0;
    dfs(&root, &mut diameter);

    diameter
}
