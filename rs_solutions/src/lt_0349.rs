use std::collections::{BTreeSet, HashSet};

// record every element to btree-set and search
pub fn intersection(n1: Vec<i32>, n2: Vec<i32>) -> Vec<i32> {
    let mut s1 = BTreeSet::new();
    let mut s2 = BTreeSet::new();
    for e in n1 {
        s1.insert(e);
    }
    for e in n2 {
        s2.insert(e);
    }

    let mut res = vec![];
    if s1.len() < s2.len() {
        for e in s1 {
            if s2.contains(&e) {
                res.push(e);
            }
        }
    } else {
        for e in s2 {
            if s1.contains(&e) {
                res.push(e);
            }
        }
    }

    res
}

// record a table
pub fn intersection_2(n1: Vec<i32>, n2: Vec<i32>) -> Vec<i32> {
    let mut table = [0; 1005];
    let mut res = HashSet::new();
    for e in n1 {
        table[e as usize] = 1;
    }
    for e in n2 {
        if table[e as usize] == 1 {
            res.insert(e);
        }
    }
    res.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
        assert_eq!(intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);

        assert_eq!(intersection_2(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
        assert!(
            intersection_2(vec![4, 9, 5], vec![9, 4, 9, 8, 4]) == vec![4, 9]
                || intersection_2(vec![4, 9, 5], vec![9, 4, 9, 8, 4]) == vec![9, 4]
        );
    }
}
