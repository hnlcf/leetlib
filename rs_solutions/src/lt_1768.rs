use std::iter;

pub fn merge_alternately(w1: String, w2: String) -> String {
    let ww1 = w1.chars();
    let ww2 = w2.chars();

    let f = |(first, last)| format!("{}{}", first, last);
    let raw_str: String = if w1.len() < w2.len() {
        ww1.into_iter()
            .chain(iter::repeat(' '))
            .zip(ww2.into_iter())
            .map(f)
            .collect()
    } else {
        ww1.into_iter()
            .zip(ww2.into_iter().chain(iter::repeat(' ')))
            .map(f)
            .collect()
    };

    raw_str
        .chars()
        .into_iter()
        .filter(|&ch| ch != ' ')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr".to_string()
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs".to_string()
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd".to_string()
        );
    }
}
