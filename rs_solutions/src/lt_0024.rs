use crate::lt_list::{list_into_vec, ListNode};

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut list = list_into_vec(head);
    let n = list.len();

    let mut i = 0;
    while i + 1 < n {
        list.swap(i, i + 1);
        i += 2;
    }

    list.into_iter().rev().fold(None, |acc, e| {
        Some(Box::new(ListNode { val: e, next: acc }))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lt_list::{list_into_vec, ListNode};

    fn assist_test(list: Vec<i32>, expect: Vec<i32>) {
        let actual = list_into_vec(swap_pairs(ListNode::from_vec(&list)));
        assert_eq!(actual, expect);
    }

    #[test]
    fn ex1() {
        assist_test(vec![1, 2, 3, 4], vec![2, 1, 4, 3]);
        assist_test(vec![1, 2, 3], vec![2, 1, 3]);
        assist_test(vec![], vec![]);
        assist_test(vec![1], vec![1]);
    }
}
