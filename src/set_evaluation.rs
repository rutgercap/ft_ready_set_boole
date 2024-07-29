pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_set_evaluation() {
        let sets = vec![vec![1], vec![2]];
        let result = eval_set("A|B", sets.clone());
        assert_eq!(result, vec![1, 2]);
    }
}