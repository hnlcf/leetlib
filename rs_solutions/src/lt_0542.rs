use std::collections::VecDeque;

pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    fn bfs(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (row, col) = (grid.len(), grid[0].len());
        let mut ans = vec![vec![i32::MAX; col]; row];
        let mut queue = VecDeque::new();

        for (i, row) in grid.iter().enumerate() {
            for (j, ele) in row.iter().enumerate() {
                if *ele == 0 {
                    ans[i][j] = 0;
                    queue.push_back((i, j));
                }
            }
        }

        while let Some((i, j)) = queue.pop_front() {
            for new in [0, 1, 0, usize::MAX, 0].windows(2) {
                let new_i = i.wrapping_add(new[0]);
                let new_j = j.wrapping_add(new[1]);

                if new_i < row && new_j < col && ans[new_i][new_j] > ans[i][j] {
                    ans[new_i][new_j] = ans[i][j] + 1;
                    queue.push_back((new_i, new_j));
                }
            }
        }

        ans
    }

    bfs(mat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0],]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0],]
        );
        assert_eq!(
            update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
        );
    }
}
