use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(vec: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if vec.is_empty() {
            return None;
        }

        let mut root = Self::new(vec[0]);
        root.fill(vec, 0);

        Some(Rc::new(RefCell::new(root)))
    }

    fn fill(&mut self, vec: &[i32], index: usize) {
        let left = 2 * index + 1;
        if left < vec.len() {
            self.left = Some(Rc::new(RefCell::new(TreeNode::new(vec[left]))));
            self.left.as_ref().unwrap().borrow_mut().fill(vec, left);
        }
        let right = left + 1;
        if right < vec.len() {
            self.right = Some(Rc::new(RefCell::new(TreeNode::new(vec[right]))));
            self.right.as_ref().unwrap().borrow_mut().fill(vec, right);
        }
    }

    pub fn into_vec(&self) -> Vec<i32> {
        let mut res = vec![];
        self.traverse_inorder(&mut res);
        res
    }

    fn traverse_inorder(&self, result: &mut Vec<i32>) {
        if let Some(left) = &self.left {
            left.as_ref().borrow().traverse_inorder(result);
        }
        result.push(self.val);
        if let Some(right) = &self.right {
            right.as_ref().borrow().traverse_inorder(result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_from_vec() {
        assert_eq!(TreeNode::from_vec(&[]), None);
    }

    #[test]
    fn ex_into_vec() {
        let tree = TreeNode::from_vec(&[4, 2, 7, 1, 3, 6, 9]);
        assert_eq!(tree.unwrap().borrow().into_vec(), vec![1, 2, 3, 4, 6, 7, 9]);
    }
}
