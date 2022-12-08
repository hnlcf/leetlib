pub fn rotate_string(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }

    let n = s.len();
    (1..n).any(|i| s[..i] == goal[n - i..] && s[i..] == goal[..n - i])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert!(rotate_string("abcde".to_string(), "cdeab".to_string()));
        assert!(!rotate_string("abcde".to_string(), "abced".to_string()));
        assert!(!rotate_string("aa".to_string(), "a".to_string()));
    }
}
