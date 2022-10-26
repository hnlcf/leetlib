use std::cell::RefCell;
use std::rc::Rc;

use crate::lt_tree::TreeNode;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn invert(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(entry) = node {
            let right = invert(&entry.as_ref().borrow().left);
            let left = invert(&entry.as_ref().borrow().right);
            Some(Rc::new(RefCell::new(TreeNode {
                val: entry.as_ref().borrow().val,
                left,
                right,
            })))
        } else {
            None
        }
    }
    invert(&root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            invert_tree(TreeNode::from_vec(&[4, 2, 7, 1, 3, 6, 9]))
                .as_ref()
                .unwrap()
                .borrow()
                .into_vec(),
            vec![9, 7, 6, 4, 3, 2, 1]
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            invert_tree(TreeNode::from_vec(&[2, 1, 3]))
                .as_ref()
                .unwrap()
                .borrow()
                .into_vec(),
            vec![3, 2, 1]
        );
    }

    #[test]
    fn ex3() {
        assert!(invert_tree(TreeNode::from_vec(&[])).is_none());
    }
}
