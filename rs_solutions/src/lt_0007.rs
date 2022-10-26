pub fn reverse(x: i32) -> i32 {
    let xx: i32 = x
        .to_string()
        .chars()
        .into_iter()
        .filter(|n| n.is_numeric())
        .rev()
        .collect::<String>()
        .parse()
        .unwrap_or(0);

    if x.is_negative() {
        -xx
    } else {
        xx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(reverse(123), 321);
    }

    #[test]
    fn ex2() {
        assert_eq!(reverse(-123), -321);
    }

    #[test]
    fn ex3() {
        assert_eq!(reverse(120), 21);
    }

    #[test]
    fn ex4() {
        assert_eq!(reverse(0), 0);
    }

    #[test]
    fn ex5() {
        assert_eq!(reverse(1534236469), 0);
    }
}
