use crate::operator::Operator;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn operands_in_formula(formula: &str) -> Vec<char> {
    formula
        .chars()
        .filter(|c| c.is_alphabetic())
        .fold(vec![], |mut acc, c| {
            if !acc.contains(&c) {
                acc.push(c);
            }
            acc
        })
}

fn print_header(operands: &[char]) {
    let mut temp: String = operands.iter().map(|c| format!("| {} ", c)).collect();
    temp.push_str("| = |");
    println!("{}", temp);
    let mut temp: String = operands.iter().map(|_| "|---").collect();
    temp.push_str("|---|");
    println!("{}", temp);
}

fn operand_combinations(operands: &[char]) -> Vec<Vec<(char, bool)>> {
    operands
        .iter()
        .cartesian_product(vec![true, false])
        .map(|(&c, b)| (c, b))
        .combinations(operands.len())
        .filter(|c| {
            let mut seen = HashSet::new();
            for (c, _) in c {
                if seen.contains(c) {
                    return false;
                }
                seen.insert(c);
            }
            true
        })
        .sorted_by(|a, b| {
            let n_trues = a.iter().fold(0, |a, (_, b)| if *b { a + 1 } else { a });
            let m_trues = b.iter().fold(0, |a, (_, b)| if *b { a + 1 } else { a });
            n_trues.cmp(&m_trues)
        })
        .collect_vec()
}

fn print_values(values: &[(char, bool)]) -> String {
    let mut temp: String = values
        .iter()
        .map(|(_, b)| format!("| {} ", if *b { 1 } else { 0 }))
        .collect();
    temp.push_str("|");
    temp
}

fn solve(node: &Operator, values: &HashMap<char, bool>) -> bool {
    match node {
        Operator::And(a, b) => solve(a, values) && solve(b, values),
        Operator::Or(a, b) => solve(a, values) || solve(b, values),
        Operator::Xor(a, b) => solve(a, values) ^ solve(b, values),
        Operator::Implies(a, b) => !solve(a, values) || solve(b, values),
        Operator::Equals(a, b) => solve(a, values) == solve(b, values),
        Operator::Not(a) => !solve(a, values),
        Operator::Operand(c) => *values.get(c).expect("No value for operand"),
    }
}

pub fn truth_table(operator: &Operator, operands: &[char]) -> Vec<(Vec<(char, bool)>, bool)> {
    let combinations = operand_combinations(operands);
    combinations
        .iter()
        .map(|comb| {
            let values = HashMap::from_iter(comb.clone());
            (comb.clone(), solve(operator, &values))
        })
        .collect()
}

pub fn print_truth_table(formula: &str) {
    let operator = Operator::from_formula(formula);
    if operator.is_none() {
        return;
    }
    let operator = operator.unwrap();
    let operands = operands_in_formula(formula);
    print_header(&operands);
    let table = truth_table(&operator, &operands);
    for (row, result) in table {
        let mut values = print_values(&row);
        values.push_str(format!(" {} |", if result { 1 } else { 0 }).as_str());
        println!("{}", values);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn truth_tables_equal(a: &str, b: &str) {
        let a_operator = Operator::from_formula(a).unwrap();
        let b_operator = Operator::from_formula(b).unwrap();
        let a_operands = operands_in_formula(a.to_string().as_str());
        let b_operands = operands_in_formula(b.to_string().as_str());
        let a_table = truth_table(&a_operator, &a_operands);
        let b_table = truth_table(&b_operator, &b_operands);
        for (a_row, b_row) in a_table.iter().zip(b_table.iter()) {
            assert_eq!(a_row, b_row);
        }
    }

    #[test]
    fn getting_operands_from_formula_works() {
        let operands = operands_in_formula("AB&");

        assert_eq!(operands, vec!['A', 'B',]);
    }

    #[test]
    fn getting_operands_from_formula_does_not_count_doubles() {
        let operands = operands_in_formula("ABB&");

        assert_eq!(operands, vec!['A', 'B',]);
    }

    #[test]
    fn getting_operands_from_formula_works_for_complicated_formulas() {
        let operands = operands_in_formula("ABCD||=E");

        assert_eq!(operands, vec!['A', 'B', 'C', 'D', 'E',]);
    }

    #[test]
    fn can_solve_for_correct_values_1() {
        let tree = Operator::from_formula("AB&").unwrap();
        let values = HashMap::from_iter(vec![('A', true), ('B', false)]);

        assert_eq!(solve(&tree, &values), false);
    }

    #[test]
    fn can_solve_for_correct_values_2() {
        let tree = Operator::from_formula("AB|").unwrap();
        let values = HashMap::from_iter(vec![('A', true), ('B', false)]);

        assert_eq!(solve(&tree, &values), true);
    }

    #[test]
    fn can_solve_for_correct_values_3() {
        let tree = Operator::from_formula("ABCD||=").unwrap();
        let values = HashMap::from_iter(vec![('A', true), ('B', false), ('C', true), ('D', true)]);

        assert_eq!(solve(&tree, &values), true);
    }

    #[test]
    fn can_solve_for_correct_values_4() {
        let tree = Operator::from_formula("A").unwrap();
        let values = HashMap::from_iter(vec![('A', true)]);

        assert_eq!(solve(&tree, &values), true);
    }

    #[test]
    fn can_solve_for_correct_values_5() {
        let tree = Operator::from_formula("A!!").unwrap();
        let values = HashMap::from_iter(vec![('A', true)]);

        assert_eq!(solve(&tree, &values), true);
    }

    #[test]
    fn can_solve_for_correct_values_6() {
        let tree = Operator::from_formula("AB&!").unwrap();
        let values = HashMap::from_iter(vec![('A', true), ('B', true)]);

        assert_eq!(solve(&tree, &values), false);
    }
}
