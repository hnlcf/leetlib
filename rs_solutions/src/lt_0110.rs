use std::cell::RefCell;
use std::rc::Rc;

use crate::lt_tree::TreeNode;

/// Refer to discussion
pub fn is_balanced_v1(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        match node {
            Some(node_rc) => {
                let node_ref = node_rc.as_ref().borrow();
                match (dfs(&node_ref.left), dfs(&node_ref.right)) {
                    (Some(left), Some(right)) if (left - right).abs() <= 1 => {
                        Some(left.max(right) + 1)
                    }
                    _ => None,
                }
            }
            None => Some(0),
        }
    }

    dfs(&root).is_some()
}

/// By myself
pub fn is_balanced_v2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn get_height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if node.is_none() {
            return 0;
        }
        return std::cmp::max(
            get_height(&node.as_ref().unwrap().borrow().left),
            get_height(&node.as_ref().unwrap().borrow().right),
        ) + 1;
    }

    fn get_balance(node: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if node.is_none() {
            return true;
        }

        let left = get_height(&node.as_ref().unwrap().borrow().left);
        let right = get_height(&node.as_ref().unwrap().borrow().right);

        let flag = (left - right).abs() <= 1;

        return flag
            && get_balance(&node.as_ref().unwrap().borrow().left)
            && get_balance(&node.as_ref().unwrap().borrow().right);
    }

    get_balance(&root)
}
