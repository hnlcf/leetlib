pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut res = vec![vec![0; n]; n];
    let mut count = 1; // the counter

    let mut st_x = 0; // the start row
    let mut st_y = 0; // the start column

    let mut n_loop = n / 2; // the loop number
    let mut offset = 1; // the offset to end
    let mid = n / 2; // the last middle block

    while n_loop != 0 {
        (st_y..n - offset).for_each(|i| {
            res[st_x][i] = count;

            count += 1;
        });

        (st_x..n - offset).for_each(|i| {
            res[i][n - offset] = count;

            count += 1;
        });

        (st_y + 1..=n - offset).rev().for_each(|i| {
            res[n - offset][i] = count;

            count += 1;
        });

        (st_x + 1..=n - offset).rev().for_each(|i| {
            res[i][st_y] = count;

            count += 1;
        });

        st_x += 1;
        st_y += 1;
        offset += 1;

        n_loop -= 1;
    }

    if n % 2 == 1 {
        res[mid][mid] = count;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(generate_matrix(1), vec![vec![1]]);
    }

    #[test]
    fn ex3() {
        assert_eq!(
            generate_matrix(4),
            vec![
                vec![1, 2, 3, 4],
                vec![12, 13, 14, 5],
                vec![11, 16, 15, 6],
                vec![10, 9, 8, 7]
            ]
        );
    }
}
