use std::cmp;

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];

    let m = matrix.len();
    let n = matrix[0].len();

    let mut st_x = 0;
    let mut st_y = 0;
    let mut offset = 1;
    let mut n_loop = cmp::min(n, m) / 2;

    while n_loop != 0 {
        (st_y..n - offset).for_each(|i| {
            res.push(matrix[st_x][i]);
        });

        (st_x..m - offset).for_each(|i| {
            res.push(matrix[i][n - offset]);
        });

        (st_y + 1..=n - offset).rev().for_each(|i| {
            res.push(matrix[m - offset][i]);
        });

        (st_x + 1..=m - offset).rev().for_each(|i| {
            res.push(matrix[i][st_y]);
        });

        st_x += 1;
        st_y += 1;
        offset += 1;

        n_loop -= 1;
    }

    // single row else(include only one block else in square matrix)
    if m <= n && m % 2 == 1 {
        (st_y..=n - offset).for_each(|i| {
            res.push(matrix[m - offset][i]);
        });
    }

    // single column else
    if n < m && n % 2 == 1 {
        (st_x..=m - offset).for_each(|i| {
            res.push(matrix[i][n - offset]);
        });
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
    #[test]
    fn ex3() {
        assert_eq!(spiral_order(vec![vec![6, 9, 7],]), vec![6, 9, 7]);
    }

    #[test]
    fn ex4() {
        assert_eq!(spiral_order(vec![vec![7], vec![9], vec![6]]), vec![7, 9, 6]);
    }

    #[test]
    fn ex5() {
        assert_eq!(
            spiral_order(vec![
                vec![2, 3, 4],
                vec![5, 6, 7],
                vec![8, 9, 10],
                vec![11, 12, 13],
                vec![14, 15, 16]
            ]),
            vec![2, 3, 4, 7, 10, 13, 16, 15, 14, 11, 8, 5, 6, 9, 12]
        );
    }
}
