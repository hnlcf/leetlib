//! # 76 minimum-window-substring
//!
//! - tag: array, slide window
//! - link: https://leetcode.cn/problems/minimum-window-substring/
use std::collections::HashMap;

pub fn min_window(s: &str, t: &str) -> String {
    let mut ans = "";
    let mut ans_len = s.len();
    // count is the number of s not include characters of t
    let mut count = t.len();

    // record all characters in t and corresponding numbers
    let tt: Vec<_> = t.chars().collect();
    let mut map = HashMap::new();
    for ch in tt {
        let cnt = map.entry(ch).or_insert(0);
        *cnt += 1;
    }

    let ss: Vec<_> = s.chars().collect();
    let mut lo = 0;
    for hi in 0..ss.len() {
        if !map.contains_key(&ss[hi]) {
            continue;
        }
        if let Some(n) = map.get_mut(&ss[hi]) {
            // n = number of char in t - number of char in s
            *n -= 1;
            if *n >= 0 {
                count -= 1;
            }
        }

        // a success substr search
        while count == 0 {
            // compare the window length and record answer if it's shorter
            if ans_len > hi - lo {
                ans = &s[lo..=hi];
                ans_len = hi - lo + 1;
            }

            // if t not contains the char in s[lo], shorten the window from left and directly jump to next loop
            if !map.contains_key(&ss[lo]) {
                lo += 1;
                continue;
            }

            // otherwise, decrease the number of s[lo] in t's map
            if let Some(n) = map.get_mut(&ss[lo]) {
                *n += 1;
                if *n > 0 {
                    count += 1;
                }
            }
            lo += 1;
        }
    }

    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let case = vec![
            ("cabwefgewcwaefgcf".to_string(), "cae".to_string()),
            ("a".to_string(), "a".to_string()),
            ("a".to_string(), "aa".to_string()),
        ];
        let expect = vec!["cwae".to_string(), "a".to_string(), "".to_string()];

        (0..3).for_each(|i| {
            assert_eq!(min_window(&case[i].0, &case[i].1), expect[i]);
        })
    }
}
