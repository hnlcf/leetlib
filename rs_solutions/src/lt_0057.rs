pub fn insert(intervals: Vec<Vec<i32>>, new: Vec<i32>) -> Vec<Vec<i32>> {
    let new_start = new[0];
    let new_end = new[1];

    let lap = |start, end| {
        (start <= new_start && new_start <= end)
            || (start <= new_end && new_end <= end)
            || (new_start <= start && end <= new_end)
    };
    let mut lap_start = new_start;
    let mut lap_end = new_end;
    let mut res = vec![];

    for v in intervals {
        let start = v[0];
        let end = v[1];
        let is_lap = lap(start, end);

        if is_lap {
            lap_start = lap_start.min(start);
            lap_end = lap_end.max(end);
        } else {
            res.push(v);
        }
    }

    res.push(vec![lap_start, lap_end]);
    res.sort_by(|a, b| a[0].cmp(&b[0]));

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(insert(vec![], vec![1, 5]), vec![vec![1, 5]]);
        assert_eq!(insert(vec![vec![1, 5]], vec![2, 3]), vec![vec![1, 5]]);
        assert_eq!(
            insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
        assert_eq!(
            insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }
}
