#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { next, val }
    }

    pub fn from_vec(vec: &[i32]) -> Option<Box<ListNode>> {
        vec.iter()
            .rev()
            .fold(None, |acc, &ele| Some(Box::new(ListNode::new(ele, acc))))
    }
}

pub fn list_into_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut res = vec![];
    while let Some(entry) = list {
        res.push(entry.val);
        list = entry.next;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_from_vec() {
        let vec = vec![1, 2, 3, 4, 5];
        let mut list = ListNode::from_vec(&vec);
        let mut n = 1;

        while let Some(entry) = list {
            assert_eq!(entry.val, n);
            n += 1;
            list = entry.next;
        }
    }

    #[test]
    fn ex_to_vec() {
        let vec = vec![1, 2, 3, 4, 5];
        let list = ListNode::from_vec(&vec);
        assert_eq!(list_into_vec(list), vec);
    }
}
