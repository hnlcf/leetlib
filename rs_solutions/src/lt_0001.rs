use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res = vec![];
    let mut table = HashMap::new();
    for (i, &e) in nums.iter().enumerate() {
        let other = target - e;
        if let Some(&j) = table.get(&other) {
            res.push(j as i32);
            res.push(i as i32);
        }
        table.insert(e, i);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex1() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
}
