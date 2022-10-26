pub fn is_valid(s: String) -> bool {
    let mut st = vec![];
    let mut flag = true;

    for ch in s.chars() {
        match ch {
            '(' => st.push(')'),
            '[' => st.push(']'),
            '{' => st.push('}'),
            _ => {
                let close = st.pop();
                if close.is_some() && close.unwrap() == ch {
                    continue;
                } else {
                    flag = false;
                    break;
                }
            }
        }
    }

    if !st.is_empty() {
        flag = false;
    }

    flag
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert!(is_valid("()".to_string()));
        assert!(is_valid("()[]{}".to_string()));
        assert!(!is_valid("(]".to_string()));
    }
}
