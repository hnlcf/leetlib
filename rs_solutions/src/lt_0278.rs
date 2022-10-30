pub struct Solution;

impl Solution {
    #[allow(non_snake_case)]
    pub fn isBadVersion(&self, _n: i32) -> bool {
        todo!()
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut lo = 1;
        let mut hi = n;
        let mut mid;
        while lo <= hi {
            mid = lo + (hi - lo) / 2;
            if self.isBadVersion(mid) {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        lo // Or return hi-1
    }
}
