/// DP -- Fibonacci
///
/// # Explanation
///
/// Because for any ways to target `[n]`, the last step must be `1` from `[n-1]` or
/// `2` from `[n-2]`.
///
/// Suppose there is `n1` ways to reach `[n-1]` and `n2` ways to reach `[n-2]`,
/// the two solutions(`n1` and `n2`) cover all cases and are no overlapping.
///
/// # Solution
///
/// - Initial state: `dp[1] = 1, dp[2] = 2`
/// - Invariant: `dp[n] = dp[n-1] + dp[n-2]`
pub fn climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    let mut first = 1;
    let mut second = 2;

    for _ in 3..=n {
        second += first;
        first = second - first;
    }

    second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
    }
}
