use std::collections::HashMap;

pub fn longest_palindrome(s: String) -> i32 {
    let mut counter: HashMap<char, i32> = HashMap::new();
    for ch in s.chars() {
        counter.entry(ch).and_modify(|cnt| *cnt += 1).or_insert(1);
    }

    let mut have_single = false;
    let pair: i32 = counter
        .into_values()
        .map(|v| {
            if v % 2 == 1 {
                have_single = true;
            }
            v / 2
        })
        .sum();

    if have_single {
        2 * pair + 1
    } else {
        2 * pair
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(longest_palindrome("abccccdd".to_string()), 7);
        assert_eq!(longest_palindrome("a".to_string()), 1);
    }
}
