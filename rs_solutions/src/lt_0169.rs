use std::collections::HashMap;

pub fn majority_element(n: Vec<i32>) -> i32 {
    let mut table: HashMap<i32, usize> = HashMap::new();
    for e in n {
        table.entry(e).and_modify(|cnt| *cnt += 1).or_insert(1);
    }

    table
        .into_iter()
        .fold((0, 0), |acc, (k, v)| if v > acc.1 { (k, v) } else { acc })
        .0
}

/// Moore's voting algorithm
pub fn majority_element_v2(n: Vec<i32>) -> i32 {
    let mut major = 0;
    let mut count = 0;

    for elem in n {
        if count == 0 {
            major = elem;
        }

        if major == elem {
            count += 1;
        } else {
            count -= 1;
        }
    }

    major
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(majority_element_v2(vec![3, 2, 3]), 3);
        assert_eq!(majority_element_v2(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
