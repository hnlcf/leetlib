use std::cell::RefCell;
use std::rc::Rc;

use crate::lt_tree::TreeNode;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let p = p.unwrap().as_ref().borrow().val;
    let q = q.unwrap().as_ref().borrow().val;

    // For BST, left < root < right
    // So for lca of two nodes, it is unique and the first middle value in [p, q]
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        p: i32,
        q: i32,
        res: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(n) = node {
            let node_ref = n.as_ref().borrow();
            let node_val = node_ref.val;

            *res = Some(Rc::new(RefCell::new(TreeNode::new(node_val))));

            if node_val < p && node_val < q {
                dfs(&node_ref.right, p, q, res)
            }
            if node_val > p && node_val > q {
                dfs(&node_ref.left, p, q, res)
            }
        }
    }

    let mut res = None;
    dfs(&root, p, q, &mut res);
    res
}

pub fn lowest_common_ancestor_2(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut node = root;
    let p = p.unwrap().as_ref().borrow().val;
    let q = q.unwrap().as_ref().borrow().val;

    loop {
        let n = node.as_ref().unwrap().borrow().val;
        if n > p && n > q {
            node = node.unwrap().borrow().left.clone();
        } else if n < p && n < q {
            node = node.unwrap().borrow().right.clone();
        } else {
            return node;
        }
    }
}
