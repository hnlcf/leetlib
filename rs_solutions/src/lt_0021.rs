use crate::lt_list::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // FIX: recursive version
    let mut l1 = list1;
    let mut l2 = list2;
    let mut res = vec![];

    while let Some(entry) = l1 {
        res.push(entry.val);
        l1 = entry.next;
    }
    while let Some(entry) = l2 {
        res.push(entry.val);
        l2 = entry.next;
    }

    res.sort();

    res.into_iter().rev().fold(None, |acc, e| {
        Some(Box::new(ListNode { val: e, next: acc }))
    })
}
