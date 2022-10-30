use std::collections::HashMap;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut table: HashMap<char, i32> = HashMap::new();

    for ch in magazine.chars() {
        table.entry(ch).and_modify(|cnt| *cnt += 1).or_insert(1);
    }

    for ch in ransom_note.chars() {
        match table.get_mut(&ch) {
            Some(cnt) if *cnt > 0 => *cnt -= 1,
            _ => return false,
        }
    }

    true
}

pub fn can_construct_v2(ransom_note: String, magazine: String) -> bool {
    let mut src: HashMap<char, i32> = HashMap::new();
    for ch in magazine.chars() {
        src.entry(ch).and_modify(|cnt| *cnt += 1).or_insert(1);
    }

    let mut pat: HashMap<char, i32> = HashMap::new();
    for ch in ransom_note.chars() {
        pat.entry(ch).and_modify(|cnt| *cnt += 1).or_insert(1);
    }

    for (k, v) in pat {
        if !src.contains_key(&k) || src[&k] < v {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert!(!can_construct("a".to_string(), "b".to_string()));
        assert!(!can_construct("aa".to_string(), "ab".to_string()));
        assert!(can_construct("aa".to_string(), "aab".to_string()));
        assert!(can_construct(
            "bg".to_string(),
            "efjbdfbdgfjhhaiigfhbaejahgfbbgbjagbddfgdiaigdadhcfcj".to_string()
        ));
    }

    #[test]
    fn ex2() {
        assert!(!can_construct_v2("a".to_string(), "b".to_string()));
        assert!(!can_construct_v2("aa".to_string(), "ab".to_string()));
        assert!(can_construct_v2("aa".to_string(), "aba".to_string()));
        assert!(can_construct_v2(
            "bg".to_string(),
            "efjbdfbdgfjhhaiigfhbaejahgfbbgbjagbddfgdiaigdadhcfcj".to_string()
        ));
    }
}
