use crate::lt_list::ListNode;

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast = &head;
    let mut slow = &head;
    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        slow = &slow.as_ref().unwrap().next;
    }
    slow.clone()
}
