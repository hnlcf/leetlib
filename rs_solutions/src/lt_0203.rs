use crate::lt_list::ListNode;

pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut arr = vec![];
    let mut list = head;

    while let Some(entry) = list {
        arr.push(entry.val);
        list = entry.next;
    }
    arr.iter()
        .filter(|&&e| e != val)
        .rev()
        .fold(None, |acc, &e| {
            Some(Box::new(ListNode { next: acc, val: e }))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lt_list::{list_into_vec, ListNode};

    fn assist_test(head: &[i32], val: i32, expect: &[i32]) {
        let actual = list_into_vec(remove_elements(ListNode::from_vec(head), val));
        assert_eq!(actual, expect);
    }

    #[test]
    fn ex1() {
        assist_test(&[1, 2, 6, 3, 4, 5, 6], 6, &[1, 2, 3, 4, 5]);
        assist_test(&[], 7, &[]);
        assist_test(&[7, 7, 7, 7], 7, &[]);
    }
}
