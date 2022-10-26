use crate::lt_list::{list_into_vec, ListNode};

// list -> vec -> list
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    list_into_vec(head).into_iter().fold(None, |acc, e| {
        Some(Box::new(ListNode { val: e, next: acc }))
    })
}

// double pointer
pub fn reverse_list_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head;
    let mut pre = None;
    while let Some(mut node) = cur.take() {
        cur = node.next;

        node.next = pre;
        pre = Some(node);
    }
    pre
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lt_list::{list_into_vec, ListNode};

    fn assist_test(list: Vec<i32>, expect: Vec<i32>) {
        let actual = list_into_vec(reverse_list(ListNode::from_vec(&list)));
        assert_eq!(actual, expect);

        let actual = list_into_vec(reverse_list_2(ListNode::from_vec(&list)));
        assert_eq!(actual, expect);
    }

    #[test]
    fn ex1() {
        assist_test(vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]);
        assist_test(vec![1, 2], vec![2, 1]);
        assist_test(vec![], vec![]);
    }
}
