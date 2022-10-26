pub fn is_palindrome(s: String) -> bool {
    let forward: String = s
        .to_lowercase()
        .chars()
        .into_iter()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect();
    let backward: String = forward.chars().into_iter().rev().collect();

    forward == backward
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
    }

    #[test]
    fn ex2() {
        assert!(!is_palindrome("race a car".to_string()));
    }

    #[test]
    fn ex3() {
        assert!(is_palindrome(" ".to_string()));
    }

    #[test]
    fn ex4() {
        assert!(!is_palindrome("0P".to_string()));
    }
}
