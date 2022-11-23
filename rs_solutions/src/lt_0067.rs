pub fn add_binary(a: String, b: String) -> String {
    let a = u128::from_str_radix(&a, 2).unwrap();
    let b = u128::from_str_radix(&b, 2).unwrap();
    let mut sum = a + b;

    let mut res = String::new();
    if sum == 0 {
        res.push('0');
    } else {
        while sum > 0 {
            if sum % 2 == 1 {
                res.push('1');
            } else {
                res.push('0');
            }
            sum /= 2;
        }
    }

    res.chars().into_iter().rev().collect()
}

pub fn add_binary_v2(mut a: String, mut b: String) -> String {
    if a.len() < b.len() {
        std::mem::swap(&mut a, &mut b);
    }
    let mut extra = false;
    let mut raw: String = a
        .chars()
        .into_iter()
        .rev()
        .zip(b.chars().into_iter().rev().chain(std::iter::repeat('0')))
        .map(|k| match k {
            ('0', '0') if !extra => '0',
            ('1', '0') | ('0', '1') if !extra => '1',
            ('1', '1') if !extra => {
                extra = true;
                '0'
            }
            ('0', '0') if extra => {
                extra = false;
                '1'
            }
            ('1', '0') | ('0', '1') if extra => {
                extra = true;
                '0'
            }
            ('1', '1') if extra => {
                extra = true;
                '1'
            }
            _ => ' ',
        })
        .collect();
    if extra {
        raw.push('1');
    }
    raw.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            add_binary("0".to_string(), "0".to_string()),
            "0".to_string()
        );
        assert_eq!(
            add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );
        assert_eq!(
            add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            add_binary_v2("0".to_string(), "0".to_string()),
            "0".to_string()
        );
        assert_eq!(
            add_binary_v2("11".to_string(), "1".to_string()),
            "100".to_string()
        );
        assert_eq!(
            add_binary_v2("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }
}
