# Array

## Binary search

```rust
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left:usize = 0;
        let mut right:usize = nums.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else if nums[mid] > target {
                right = mid;
            } else {
                return mid as i32;
            }
        }
        -1
    }
}
```

- [34 find-first-and-last-position-of-element-in-sorted-array](lt_34.md)
- [35 search-insert-location](lt_35.md)
- [69 sqrtx](lt_69.md)
- [367 valid-perfect-square](lt_367.md)
- [704 binary-search](lt_704.md)

## Remove element
