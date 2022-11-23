use std::cmp;

pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    let mut dp = vec![(-1, -1); n];
    dp[0].0 = nums[0];
    dp[n - 1].1 = i32::MAX;
    dp[n - 2].1 = nums[n - 1];

    // scan the maximum of left
    for i in 1..n {
        dp[i].0 = cmp::max(nums[i], dp[i - 1].0);
    }

    // scan the minimum of right
    for i in (0..n - 2).rev() {
        dp[i].1 = cmp::min(nums[i + 1], dp[i + 1].1);
    }

    // traverse to find the maximum of left partition smaller than the minimum
    // of right partition
    let mut i = 0;
    for (left, right) in dp {
        i += 1;
        if left <= right {
            break;
        }
    }

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(partition_disjoint(vec![5, 0, 3, 8, 6]), 3);
    }

    #[test]
    fn ex2() {
        assert_eq!(partition_disjoint(vec![1, 1, 1, 0, 6, 12]), 4);
    }

    #[test]
    fn ex3() {
        assert_eq!(partition_disjoint(vec![1, 1]), 1);
    }
}
