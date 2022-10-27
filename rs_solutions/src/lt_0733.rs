use std::collections::VecDeque;

pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let m = image.len();
    let n = image[0].len();
    let elem = image[sr as usize][sc as usize];
    let mut queue = VecDeque::new();
    queue.push_back((sr as usize, sc as usize));

    while !queue.is_empty() {
        let (row, col) = queue.pop_front().unwrap();
        image[row][col] = color;

        if row + 1 < m && image[row + 1][col] == elem && image[row + 1][col] != color {
            queue.push_back((row + 1, col));
        }
        if row >= 1 && image[row - 1][col] == elem && image[row - 1][col] != color {
            queue.push_back((row - 1, col));
        }
        if col + 1 < n && image[row][col + 1] == elem && image[row][col + 1] != color {
            queue.push_back((row, col + 1));
        }
        if col >= 1 && image[row][col - 1] == elem && image[row][col - 1] != color {
            queue.push_back((row, col - 1));
        }
    }

    image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 0),
            vec![vec![0, 0, 0], vec![0, 0, 0]]
        );
    }
}
