use std::collections::{HashMap, HashSet};

use crate::operator::Operator;

fn operands_in_formula(formula: &str) -> Vec<char> {
    let mut operands = formula
        .chars()
        .filter(|c| c.is_alphabetic())
        .fold(vec![], |mut acc, c| {
            if !acc.contains(&c) {
                acc.push(c);
            }
            acc
        });
    operands.sort();
    operands
}

fn solve(operator: &Operator, sets: HashMap<char, HashSet<i32>>) -> HashSet<i32> {
    match operator {
        Operator::Operand(c) => sets.get(c).unwrap().clone(),
        Operator::Not(c) => {
            let c = solve(c, sets.clone());
            let all = sets.values().flatten().cloned().collect::<Vec<i32>>();
            all.iter().filter(|x| !c.contains(x)).cloned().collect()
        }
        Operator::Or(a, b) => {
            let a = solve(a, sets.clone());
            let b = solve(b, sets.clone());
            a.iter().chain(b.iter()).cloned().collect()
        }
        Operator::And(a, b) => {
            let a = solve(a, sets.clone());
            let b = solve(b, sets.clone());
            a.iter().filter(|x| b.contains(x)).cloned().collect()
        }
        Operator::Xor(a, b) => {
            let a = solve(a, sets.clone());
            let b = solve(b, sets.clone());
            let a: HashSet<i32> = a.into_iter().filter(|x| !b.contains(x)).collect();
            let b: HashSet<i32> = b.into_iter().filter(|x| !a.contains(x)).collect();
            a.into_iter().chain(b.into_iter()).collect()
        }
        Operator::Implies(a, b) => {
            let a = solve(a, sets.clone());
            let b = solve(b, sets.clone());
            let is_subset = a.iter().all(|x| b.contains(&x));
            if is_subset {
                a
            } else {
                HashSet::new()
            }
        }
        Operator::Equals(a, b) => {
            let a = solve(a, sets.clone());
            let b = solve(b, sets.clone());
            if a == b {
                a
            } else {
                HashSet::new()
            }
        }
    }
}

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    let operands = operands_in_formula(formula);
    if operands.len() != sets.len() {
        panic!("Number of operands and sets must be equal");
    }
    let operator = Operator::from_formula(formula);
    if operator.is_none() {
        panic!("Empty formula");
    }
    let operator = operator.unwrap();
    let set = solve(
        &operator,
        operands
            .into_iter()
            .zip(sets)
            .map(|(c, values)| (c, HashSet::from_iter(values)))
            .collect(),
    );
    set.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn incorrect_sets_for_formula_should_panic() {
        let sets = vec![vec![1]];
        eval_set("A|B", sets.clone());
    }

    #[test]
    #[should_panic]
    fn test_empty_set_should_panic() {
        let sets = vec![vec![1]];
        eval_set("", sets.clone());
    }

    #[test]
    fn eval_set_works_with_disjunction() {
        let sets = vec![vec![1], vec![2]];
        let result = eval_set("AB|B|", sets.clone());
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn eval_set_works_with_conjunction() {
        let sets = vec![vec![1], vec![2]];
        let result = eval_set("AB&", sets.clone());
        assert_eq!(result, vec![]);

        let sets = vec![vec![1], vec![1]];
        let result = eval_set("AB&", sets.clone());
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn eval_set_works_with_negation() {
        let sets = vec![vec![1]];
        let result = eval_set("A!", sets.clone());
        assert_eq!(result, vec![]);

        let sets = vec![vec![1, 3], vec![1, 2]];
        let result = eval_set("AB|!", sets.clone());
        assert_eq!(result, vec![]);

        let sets = vec![vec![1, 3], vec![1, 2]];
        let mut result = eval_set("AB!|", sets.clone());
        result.sort();
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn eval_set_works_with_equals() {
        let sets = vec![vec![1], vec![2]];
        let result = eval_set("AB=", sets.clone());
        assert_eq!(result, vec![]);

        let sets = vec![vec![1], vec![1]];
        let result = eval_set("AB=", sets.clone());
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn eval_set_works_with_xor() {
        let sets = vec![vec![1], vec![2]];
        let mut result = eval_set("AB^", sets.clone());
        result.sort();
        assert_eq!(result, vec![1, 2]);

        let sets = vec![vec![1], vec![1]];
        let result = eval_set("AB^", sets.clone());
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn eval_set_works_with_implies() {
        let sets = vec![vec![1], vec![2]];
        let result = eval_set("AB>", sets.clone());
        assert_eq!(result, vec![]);

        let sets = vec![vec![1, 2], vec![1]];
        let result = eval_set("AB>", sets.clone());
        assert_eq!(result, vec![]);

        let sets = vec![vec![1], vec![1, 2]];
        let result = eval_set("AB>", sets.clone());
        assert_eq!(result, vec![1]);
    }
}
