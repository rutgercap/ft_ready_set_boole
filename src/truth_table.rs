use std::collections::{HashMap, HashSet};

use itertools::{iproduct, Itertools};

#[derive(PartialEq, Debug, Clone)]
enum Operator {
    And(Box<Operator>, Box<Operator>),
    Or(Box<Operator>, Box<Operator>),
    Xor(Box<Operator>, Box<Operator>),
    Implies(Box<Operator>, Box<Operator>),
    Equals(Box<Operator>, Box<Operator>),
    Not(Box<Operator>),
    Operand(char),
}

impl Operator {
    fn operand(a: char) -> Operator {
        Operator::Operand(a.to_uppercase().next().unwrap())
    }

    fn with_two(new: char, a: Operator, b: Operator) -> Operator {
        match new {
            '&' => Operator::And(Box::new(a), Box::new(b)),
            '|' => Operator::Or(Box::new(a), Box::new(b)),
            '^' => Operator::Xor(Box::new(a), Box::new(b)),
            '>' => Operator::Implies(Box::new(a), Box::new(b)),
            '=' => Operator::Equals(Box::new(a), Box::new(b)),
            _ => panic!("Invalid operator"),
        }
    }

    fn not(a: Operator) -> Operator {
        Operator::Not(Box::new(a))
    }
}

fn nodes_from_formula(formula: &str) -> Option<Operator> {
    let mut stack: Vec<Operator> = Vec::new();

    for token in formula.chars() {
        if token.is_alphabetic() {
            stack.push(Operator::operand(token));
            continue;
        }
        match token {
            '!' => {
                let operand = stack.pop().expect("No operand to negate");
                let node = Operator::not(operand);
                stack.push(node);
            }
            _ => {
                let right = stack.pop().expect("Not enough operators on stack");
                let left = stack.pop().expect("Not enough operators on stack");
                let node = Operator::with_two(token, left, right);
                stack.push(node);
            }
        }
    }
    if stack.len() > 1 {
        panic!("Too many operands");
    }
    stack.pop()
}

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
        }).sorted_by(|a,b| {
            let n_trues = a.iter().fold(0, |a,(_, b) | if *b { a + 1 } else { a });
            let m_trues = b.iter().fold(0, |a,(_, b)| if *b { a + 1 } else { a });
            n_trues.cmp(&m_trues)
        })
        .collect_vec()
}

fn print_values(values: &[(char, bool)]) -> String {
    let mut temp: String = values.iter().map(|(_, b)| format!("| {} ", if *b { 1 } else { 0 })).collect();
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

pub fn print_truth_table(formula: &str) {
    let stack_option = nodes_from_formula(formula);
    if stack_option.is_none() {
        return;
    }

    let stack = stack_option.unwrap();
    let operands = operands_in_formula(formula);
    print_header(&operands);
    let combinations = operand_combinations(&operands);
    for comb in combinations {
        let mut values = print_values(&comb);
        let result = solve(&stack, &HashMap::from_iter(comb));
        values.push_str(format!(" {} |", if result { 1 } else { 0 }).as_str());
        println!("{}", values);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluating_basic_formulas_works() {
        let nodes = nodes_from_formula("AB&");

        assert_eq!(
            nodes.unwrap(),
            Operator::with_two('&', Operator::operand('A'), Operator::operand('B'))
        );
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
    fn evaluating_not_works() {
        let nodes = nodes_from_formula("A!");

        assert_eq!(nodes.unwrap(), Operator::not(Operator::Operand('A')));
    }

    #[test]
    fn evaluating_empty_string_works() {
        let nodes = nodes_from_formula("");

        assert_eq!(nodes, None);
    }

    #[test]
    fn evaluating_simple_formula_works() {
        let nodes = nodes_from_formula("A").unwrap();

        assert_eq!(nodes, Operator::operand('A'));
    }

    #[test]
    #[should_panic]
    fn evaluating_string_with_too_many_operands_throws_error() {
        let _ = nodes_from_formula("ABB&");
    }

    #[test]
    fn evaluationg_formula_with_multiple_operators_works() {
        let tree = nodes_from_formula("AB&C|").unwrap();

        assert_eq!(
            tree,
            Operator::with_two(
                '|',
                Operator::with_two('&', Operator::operand('A'), Operator::operand('B')),
                Operator::operand('C')
            )
        );
    }

    #[test]
    fn evalauting_complicated_formula_works() {
        let tree = nodes_from_formula("ABC|&").unwrap();

        assert_eq!(
            tree,
            Operator::with_two(
                '&',
                Operator::operand('A'),
                Operator::with_two('|', Operator::operand('B'), Operator::operand('C')),
            )
        );
    }

    #[test]
    fn evaluate_another_formula() {
        let tree = nodes_from_formula("ABCD||=").unwrap();

        assert_eq!(
            tree,
            Operator::with_two(
                '=',
                Operator::Operand('A'),
                Operator::with_two(
                    '|',
                    Operator::operand('B'),
                    Operator::with_two('|', Operator::operand('C'), Operator::operand('D'),),
                ),
            )
        );
    }

    #[test]
    fn can_solve_for_correct_values_1() {
        let tree = nodes_from_formula("AB&").unwrap();
        let values = HashMap::from_iter(vec![('A', true), ('B', false)]);
        
        assert_eq!(solve(&tree, &values), false);
    }

    #[test]
    fn can_solve_for_correct_values_2() {
        let tree = nodes_from_formula("AB|").unwrap();
        let values = HashMap::from_iter(vec![('A', true), ('B', false)]);
        
        assert_eq!(solve(&tree, &values), true);
    }

    #[test]
    fn can_solve_for_correct_values_3() {
        let tree = nodes_from_formula("ABCD||=").unwrap();
        let values = HashMap::from_iter(vec![('A', true), ('B', false),('C', true), ('D', true)]);
        
        assert_eq!(solve(&tree, &values), true);
    }

    #[test]
    fn can_solve_for_correct_values_4() {
        let tree = nodes_from_formula("A").unwrap();
        let values = HashMap::from_iter(vec![('A', true)]);
        
        assert_eq!(solve(&tree, &values), true);
    }
}
