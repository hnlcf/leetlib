use std::collections::{HashMap, HashSet};

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut table: HashMap<i32, usize> = HashMap::new();
    for e in nums {
        table.entry(e).and_modify(|cnt| *cnt += 1).or_insert(1);
    }

    table.into_values().any(|k| k >= 2)
}

pub fn contains_duplicate_v2(nums: Vec<i32>) -> bool {
    let mut exists = HashSet::new();
    nums.into_iter().any(|n| !exists.insert(n))
}
