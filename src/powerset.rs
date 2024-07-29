use itertools::Itertools;

pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    for i in 0..set.len() + 1 {
        let combinations = set.clone().into_iter().combinations(i).collect_vec();
        result.extend(combinations);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_powerset() {
        let set = vec![1];
        let result = powerset(set.clone());
        assert_eq!(result, vec![vec![], vec![1]]);
    }

    #[test]
    fn test_two_numbers_powerset() {
        let set = vec![1, 2];
        let result = powerset(set.clone());
        assert_eq!(result, vec![vec![], vec![1], vec![2], vec![1, 2]]);
    }

    #[test]
    fn test_powerset() {
        let set = vec![1, 2, 3];
        let result = powerset(set.clone());
        assert_eq!(
            result,
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
    }
}
