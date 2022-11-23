/// dp
pub fn max_sub_array(n: Vec<i32>) -> i32 {
    let mut dp = vec![n[0]; n.len()];
    let mut i = 1;
    while i < n.len() {
        dp[i] = std::cmp::max(n[i], n[i] + dp[i - 1]);

        i += 1;
    }
    *dp.iter().max().unwrap()
}

/// dp + optimization
pub fn max_sub_array_v2(n: Vec<i32>) -> i32 {
    let mut curr = 0; // the maximum sum of ending at the current element
    let mut max = i32::MIN; // global maximum sum
    for e in n {
        curr = std::cmp::max(e, e + curr);
        max = std::cmp::max(max, curr);
    }

    max
}

/// divide and conquer
///
/// TODO
pub fn max_sub_array_v3(n: Vec<i32>) -> i32 {
    fn max_sub_array_of_slice(n: &Vec<i32>, lo: usize, hi: usize) -> i32 {
        if lo > hi {
            return i32::MIN;
        }

        let mid = lo + (hi - lo) / 2;
        let mut left_max = 0;
        let mut right_max = 0;
        let mut left_sum = 0;
        let mut right_sum = 0;

        (lo..mid).rev().for_each(|i| {
            left_sum += n[i];
            left_max = left_max.max(left_sum);
        });

        (mid + 1..=hi).for_each(|i| {
            right_sum += n[i];
            right_max = right_max.max(right_sum);
        });

        std::cmp::max(
            std::cmp::max(
                max_sub_array_of_slice(n, lo, mid - 1),
                max_sub_array_of_slice(n, mid + 1, hi),
            ),
            left_sum + n[mid] + right_sum,
        )
    }
    max_sub_array_of_slice(&n, 0, n.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(max_sub_array(vec![1]), 1);
    }
    #[test]
    fn ex2() {
        assert_eq!(max_sub_array_v2(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sub_array_v2(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(max_sub_array_v2(vec![1]), 1);
    }

    // #[test]
    // fn ex3() {
    //     assert_eq!(max_sub_array_v3(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    //     assert_eq!(max_sub_array_v3(vec![5, 4, -1, 7, 8]), 23);
    //     assert_eq!(max_sub_array_v3(vec![1]), 1);
    // }
}
