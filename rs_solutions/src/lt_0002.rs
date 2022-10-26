use crate::lt_list::{list_into_vec, ListNode};

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // ERROR: cannot add two integers, will overflow
    let n1 = list_into_vec(l1)
        .iter()
        .rev()
        .fold(0, |acc, e| acc * 10 + e);
    let n2 = list_into_vec(l2)
        .iter()
        .rev()
        .fold(0, |acc, e| acc * 10 + e);
    let mut n = n1 + n2;
    let mut res = vec![];

    if n == 0 {
        res.push(0)
    } else {
        while n > 0 {
            res.push(n % 10);
            n /= 10;
        }
    }

    ListNode::from_vec(&res)
}

#[cfg(test)]
mod tests {
    use crate::lt_list::list_into_vec;

    use super::*;

    fn assist_test(l1: &[i32], l2: &[i32], expect: &[i32]) {
        let v1 = ListNode::from_vec(l1);
        let v2 = ListNode::from_vec(l2);

        let actual = list_into_vec(add_two_numbers(v1, v2));
        assert_eq!(actual, expect);
    }

    #[test]
    fn ex1() {
        assist_test(&[2, 4, 3], &[5, 6, 4], &[7, 0, 8]);
        // assist_test(&[0], &[0], &[0]);
        assist_test(
            &[9, 9, 9, 9, 9, 9, 9],
            &[9, 9, 9, 9],
            &[8, 9, 9, 9, 0, 0, 0, 1],
        );
    }
}
