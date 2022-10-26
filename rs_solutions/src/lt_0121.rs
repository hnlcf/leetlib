pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min = i32::MAX;
    let mut profit = 0;

    for e in prices {
        min = std::cmp::min(min, e);
        profit = std::cmp::max(profit, e - min);
    }

    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
