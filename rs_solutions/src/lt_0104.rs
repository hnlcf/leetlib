use std::cell::RefCell;
use std::rc::Rc;

use crate::lt_tree::TreeNode;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            Some(node_rc) => {
                let node_ref = node_rc.as_ref().borrow();
                let left = dfs(&node_ref.left);
                let right = dfs(&node_ref.right);

                left.max(right) + 1
            }
            None => 0,
        }
    }
    dfs(&root)
}
