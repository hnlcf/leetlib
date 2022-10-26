use crate::lt_list::{list_into_vec, ListNode};

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut list = list_into_vec(head);
    list.remove(list.len() - (n as usize));

    list.into_iter().rev().fold(None, |acc, e| {
        Some(Box::new(ListNode { val: e, next: acc }))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lt_list::{list_into_vec, ListNode};

    fn assist_test(list: Vec<i32>, n: i32, expect: Vec<i32>) {
        let actual = list_into_vec(remove_nth_from_end(ListNode::from_vec(&list), n));
        assert_eq!(actual, expect);
    }

    #[test]
    fn ex1() {
        assist_test(vec![1, 2, 3, 4, 5], 2, vec![1, 2, 3, 5]);
        assist_test(vec![1, 2], 1, vec![1]);
        assist_test(vec![1], 1, vec![]);
    }
}
