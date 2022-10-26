use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut t1: HashMap<char, i32> = HashMap::new();
    for ch in s.chars() {
        *t1.entry(ch).or_default() += 1;
    }

    let mut t2: HashMap<char, i32> = HashMap::new();
    for ch in t.chars() {
        *t2.entry(ch).or_default() += 1;
    }

    t1 == t2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
        assert!(!is_anagram("rat".to_string(), "car".to_string()));
    }
}
