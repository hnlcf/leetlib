pub fn shortest_subarray(n: Vec<i32>, k: i32) -> i32 {
    // ERROR: cannot use slide window

    let mut lo = 0;
    let mut hi = 1;
    let mut sum = n[0];
    let mut min = if sum < k { 100005 } else { 1 };

    // [lo, hi)
    while hi < n.len() {
        if sum < k {
            sum += n[hi];
            hi += 1;
        } else {
            min = std::cmp::min(min, hi - lo);

            println!(">>>{}", min);

            sum -= n[lo];
            lo += 1;
        }
    }
    if sum >= k {
        min = std::cmp::min(min, hi - lo);
    }

    if min == 100005 {
        -1
    } else {
        min as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(shortest_subarray(vec![1], 1), 1);
        assert_eq!(shortest_subarray(vec![1, 2], 4), -1);
        assert_eq!(shortest_subarray(vec![2, -1, 2], 3), 3);
        assert_eq!(shortest_subarray(vec![77, 19, 35, 10, -14], 19), 1);
    }
}
