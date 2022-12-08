use std::collections::HashMap;

pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
    let mut record = HashMap::new();
    for i in &a {
        for j in &b {
            *record.entry(i + j).or_default() += 1;
        }
    }

    let mut res = 0;
    for i in &c {
        for j in &d {
            if let Some(num) = record.get(&(0 - i - j)) {
                res += num;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
            2
        );
        assert_eq!(four_sum_count(vec![0], vec![0], vec![0], vec![0]), 1);
    }
}
